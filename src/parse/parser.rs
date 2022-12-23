use crate::ast::bin_op::BinOpNode;
use crate::ast::bool::BoolNode;
use crate::ast::class_declaration::{
    ClassDeclarationNode, ClassField,
};
use crate::ast::class_field_access::FieldAccessNode;
use crate::ast::class_init::ClassInitNode;
use crate::ast::empty_exec_root::EmptyExecRootNode;
use crate::ast::empty_global_const_decl::EmptyGlobalConstNode;
use crate::ast::empty_local_var_decl::EmptyLocalVarNode;
use crate::ast::exec_root::ExecRootNode;
use crate::ast::fn_call::FnCallNode;
use crate::ast::fn_declaration::{
    FnDeclarationNode, Parameter,
};
use crate::ast::global_const_decl::GlobalConstNode;
use crate::ast::int::IntNode;
use crate::ast::local_var_decl::LocalVarNode;
use crate::ast::mutate_var::MutateVar;
use crate::ast::pass::PassNode;
use crate::ast::r#break::BreakNode;
use crate::ast::r#continue::ContinueNode;
use crate::ast::r#if::IfNode;
use crate::ast::r#return::ReturnNode;
use crate::ast::r#while::WhileLoopNode;
use crate::ast::scope::ScopeNode;
use crate::ast::statements::StatementsNode;
use crate::ast::str::StrNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::type_expr::TypeNode;
use crate::ast::unary_op::UnaryOpNode;
use crate::ast::Node;
use crate::context::Context;
use crate::error::{numeric_overflow, syntax_error, Error};
use crate::parse::parse_results::ParseResults;
use crate::parse::token::{Token, TokenType};
use crate::position::Position;
use crate::util::{new_mut_rc, MutRc};
use std::any::Any;

macro_rules! ret_on_err {
    ($e:expr) => {
        if $e.error.is_some() {
            return $e;
        }
    };
}
macro_rules! result_ret_on_err {
    ($e:expr) => {
        if $e.error.is_some() {
            return Err($e.error.unwrap());
        }
    };
}

macro_rules! consume {
    ($self:expr, $res:expr) => {
        $self.advance(&mut $res);
        if $res.error.is_some() {
            return $res;
        }
        if $self.tok_idx >= $self.tokens.len() {
            $res.failure(
                syntax_error(
                    "Unexpected end of file".to_string(),
                ),
                Some(
                    $self.tokens[$self.tokens.len() - 1]
                        .end
                        .clone(),
                ),
                None,
            );
            return $res;
        }
    };
    ($t:ident, $self:expr, $e:expr) => {
        $self.consume(&mut $e, TokenType::$t);
        if $e.error.is_some() {
            return $e;
        }
    };
    ($res:ident = $t:ident, $self:expr, $e:expr) => {
        let $res = $self.consume(&mut $e, TokenType::$t);
        if $e.error.is_some() {
            return $e;
        }
    };
}

macro_rules! result_consume {
    ($t:ident, $self:expr, $res:expr) => {
        $self.consume(&mut $res, TokenType::$t);
        if $res.error.is_some() {
            return Err($res.error.unwrap());
        }
    };
    ($res:ident = $t:ident, $self:expr, $r:expr) => {
        let $res = $self.consume(&mut $r, TokenType::$t);
        if $r.error.is_some() {
            return Err($r.error.unwrap());
        }
    };
}

pub struct Parser {
    tokens: Vec<Token>,
    tok_idx: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, tok_idx: 0 }
    }

    pub fn parse(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if self.tokens.len() == 0 {
            res.success(new_mut_rc(EmptyExecRootNode {
                position: (
                    Position::unknown(),
                    Position::unknown(),
                ),
            }));
            return res;
        }

        let expr = res.register(self.statements());
        ret_on_err!(res);
        let root_node = ExecRootNode {
            statements: expr.unwrap(),
        };

        if self.tok_idx < self.tokens.len() {
            let current = self.current_tok().unwrap();
            res.failure(
                syntax_error(format!(
                    "Unexpected token {:?}",
                    current.str()
                )),
                Some(current.start.clone()),
                Some(current.end.clone()),
            );
            return res;
        }

        res.success(new_mut_rc(root_node));
        res
    }

    fn advance(&mut self, res: &mut ParseResults) {
        res.register_advancement();
        self.tok_idx += 1;
    }

    fn reverse(&mut self, amount: usize) -> Option<Token> {
        self.tok_idx -= amount;
        // Can't go backwards and not have a token there
        self.current_tok()
    }

    fn add_end_statement(
        &mut self,
        res: &mut ParseResults,
    ) {
        let start;
        let end;
        if self.tok_idx >= self.tokens.len() {
            start = self.last_tok().unwrap().end.clone();
            end = self.last_tok().unwrap().end.clone();
        } else {
            start = self.current_tok().unwrap().end.clone();
            end = self.current_tok().unwrap().end.clone();
        }
        self.tokens.insert(
            self.tok_idx,
            Token {
                token_type: TokenType::EndStatement,
                literal: None,
                start,
                end,
            },
        );
        self.reverse(1);
        self.advance(res);
    }

    fn consume(
        &mut self,
        res: &mut ParseResults,
        tok_type: TokenType,
    ) -> Token {
        if let Some(tok) = self.current_tok() {
            if tok.token_type == tok_type {
                self.advance(res);
                return tok;
            }
            res.failure(
                syntax_error(format!(
                    "Expected token type: {:?}, found {:?}",
                    tok_type, tok.token_type
                )),
                Some(tok.start.clone()),
                None,
            );
        } else {
            res.failure(
                syntax_error(format!(
                    "Expected {:?}, found EOF",
                    tok_type
                )),
                Some(
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                ),
                None,
            );
        }
        Token::new(
            TokenType::EndStatement,
            None,
            Position::unknown(),
            Position::unknown(),
        )
    }

    fn current_matches(
        &self,
        tok_type: TokenType,
        value: Option<String>,
    ) -> bool {
        if let Some(tok) = self.current_tok() {
            if tok.token_type == tok_type {
                if let Some(value) = value {
                    if tok.literal.is_some()
                        && tok.literal.unwrap() == value
                    {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn current_tok(&self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.tok_idx].clone())
    }

    fn last_tok(&self) -> Option<Token> {
        if self.tok_idx == 0 {
            return None;
        }
        Some(self.tokens[self.tok_idx - 1].clone())
    }

    fn clear_end_statements(
        &mut self,
        res: &mut ParseResults,
    ) {
        while self
            .current_matches(TokenType::EndStatement, None)
        {
            self.advance(res);
        }
    }

    fn bin_op<Fa, Fb>(
        &mut self,
        mut func_a: Fa,
        mut ops: Vec<(TokenType, Option<String>)>,
        mut func_b: Fb,
    ) -> ParseResults
    where
        Fa: FnMut(&mut Self) -> ParseResults,
        Fb: FnMut(&mut Self) -> ParseResults,
    {
        let mut res = ParseResults::new();
        let mut left = res.register(func_a(self));
        ret_on_err!(res);

        while let Some(op_tok) = self.current_tok() {
            let matches =
                ops.iter_mut().filter(|(op, value)| {
                    if value.is_none() {
                        return &op_tok.token_type == op;
                    }
                    &op_tok.token_type == op
                        && op_tok.literal.is_some()
                        && op_tok.literal.clone().unwrap()
                            == value.clone().unwrap()
                });
            if matches.count() == 0 {
                break;
            }
            self.advance(&mut res);

            let right = res.register(func_b(self));
            ret_on_err!(res);

            left = Some(new_mut_rc(BinOpNode {
                lhs: left.unwrap(),
                operator: op_tok,
                rhs: right.unwrap(),
            }));
        }

        ret_on_err!(res);
        res.success(left.unwrap());
        res
    }

    fn statements(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let mut statements: Vec<MutRc<dyn Node>> =
            Vec::new();
        self.clear_end_statements(&mut res);

        let first_stmt = res.register(self.statement());

        ret_on_err!(res);
        if first_stmt.is_none() {
            return res;
        }

        statements.push(first_stmt.unwrap());

        let mut more_statements = true;

        loop {
            let mut nl_count = 0;
            // @ts-ignore
            while self.current_matches(
                TokenType::EndStatement,
                None,
            ) {
                self.advance(&mut res);
                nl_count += 1;
            }
            if nl_count == 0 {
                more_statements = false;
            }
            if !more_statements {
                break;
            }

            let statement =
                res.try_register(self.statement());
            ret_on_err!(res);
            if statement.is_none() {
                self.reverse(res.reverse_count);
                continue;
            }
            statements.push(statement.unwrap());
        }

        self.clear_end_statements(&mut res);

        res.success(new_mut_rc(StatementsNode {
            statements,
        }));
        res
    }

    fn scope(
        &mut self,
        make_scope_node: bool,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        consume!(OpenBrace, self, res);

        let start = self.last_tok().unwrap().start.clone();

        if self.current_tok().is_none() {
            res.failure(
                syntax_error(
                    "Expected statement or '}'".to_string(),
                ),
                Some(start.clone().advance(None)),
                None,
            );
            return res;
        }

        let mut statements: Option<MutRc<dyn Node>> =
            Some(new_mut_rc(PassNode {
                position: (
                    start.clone(),
                    self.current_tok().unwrap().end.clone(),
                ),
            }));

        if !self
            .current_matches(TokenType::CloseBrace, None)
        {
            statements = res.register(self.statements());
            ret_on_err!(res);
        }

        consume!(CloseBrace, self, res);

        let end = self.last_tok().unwrap().end.clone();

        if make_scope_node {
            res.success(new_mut_rc(ScopeNode {
                ctx: Context::new(),
                body: statements.unwrap(),
                position: (start, end),
            }));
        } else {
            res.success(statements.unwrap());
        }

        res
    }

    fn statement(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        if self.current_tok().is_none() {
            res.failure(
                syntax_error(format!("Expected statement")),
                Some(
                    self.last_tok().unwrap().start.clone(),
                ),
                Some(self.last_tok().unwrap().end.clone()),
            );
            return res;
        }
        let start =
            self.current_tok().unwrap().start.clone();
        if self.current_matches(
            TokenType::Identifier,
            Some("const".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.global_const_decl(false));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("let".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(self.local_var_decl());
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("while".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(self.while_loop());
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("if".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(self.if_expr());
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("break".to_string()),
        ) {
            self.advance(&mut res);
            res.success(new_mut_rc(BreakNode {
                position: (
                    start.clone(),
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("continue".to_string()),
        ) {
            self.advance(&mut res);
            res.success(new_mut_rc(ContinueNode {
                position: (
                    start.clone(),
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("return".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(self.return_expr());
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("fn".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.fn_def_expr(false, None));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("class".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.class_def_expr(false));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("primitive".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.class_def_expr(true));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("extern".to_string()),
        ) {
            self.advance(&mut res);
            if self.current_matches(
                TokenType::Identifier,
                Some("fn".to_string()),
            ) {
                self.advance(&mut res);
                res.node = res
                    .register(self.fn_def_expr(true, None));
            } else if self.current_matches(
                TokenType::Identifier,
                Some("const".to_string()),
            ) {
                self.advance(&mut res);
                res.node = res
                    .register(self.global_const_decl(true));
            } else {
                res.failure(
                    syntax_error(
                        "Expected 'fn', 'var' or 'const'"
                            .to_string(),
                    ),
                    Some(self.last_tok().unwrap().end),
                    None,
                );
            }
            return res;
        }
        self.expression()
    }

    fn expression(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        self.clear_end_statements(&mut res);
        ret_on_err!(res);
        self.bin_op(
            |this| this.as_expr(),
            vec![
                (TokenType::And, None),
                (TokenType::Or, None),
            ],
            |this| this.as_expr(),
        )
    }

    fn as_expr(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.comparison_expr(),
            vec![(
                TokenType::Identifier,
                Some(format!("as")),
            )],
            |this| this.comparison_expr(),
        )
    }

    fn comparison_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let node = res.register(self.bin_op(
            |this| this.arithmetic_expr(),
            vec![
                (TokenType::DblEquals, None),
                (TokenType::NotEquals, None),
                (TokenType::GT, None),
                (TokenType::GTE, None),
                (TokenType::LTE, None),
                (TokenType::LT, None),
            ],
            |this| this.arithmetic_expr(),
        ));
        ret_on_err!(res);
        res.success(node.unwrap());
        res
    }

    fn global_const_decl(
        &mut self,
        is_external: bool,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let name;
        let start = self.last_tok().unwrap().start.clone();

        if self.current_matches(TokenType::Identifier, None)
        {
            name = self.current_tok().unwrap();
            self.advance(&mut res);
        } else {
            res.failure(
                syntax_error(
                    "Expected identifier".to_string(),
                ),
                Some(
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                ),
                None,
            );
            return res;
        }

        let mut type_: Option<MutRc<dyn Node>> = None;

        if self.current_matches(TokenType::Colon, None) {
            self.advance(&mut res);
            type_ = res.register(self.type_expr());
            ret_on_err!(res);
        }

        if !self.current_matches(TokenType::Equals, None) {
            if type_.is_none() {
                res.failure(
                    syntax_error(
                        "Expected type annotation"
                            .to_string(),
                    ),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .start
                            .clone()
                            .advance(None),
                    ),
                    None,
                );
                return res;
            }
            res.success(new_mut_rc(EmptyGlobalConstNode {
                identifier: name,
                type_: type_.unwrap(),
                is_external,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }
        consume!(self, res); // '='

        if self.current_tok().is_none() {
            res.failure(
                syntax_error("Unexpected EOF".to_string()),
                Some(
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                ),
                None,
            );
            return res;
        }

        let tok = self.current_tok().unwrap();

        if tok.token_type == TokenType::Int {
            self.advance(&mut res);
            let value = tok
                .literal
                .unwrap()
                .parse::<i64>()
                .unwrap();
            res.success(new_mut_rc(GlobalConstNode {
                identifier: name,
                value,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        } else if tok.token_type == TokenType::String {
            self.advance(&mut res);
            res.success(new_mut_rc(GlobalConstNode {
                identifier: name,
                value: tok.literal.unwrap(),
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }
        res.failure(
            syntax_error(format!(
                "Can only have integers or strings as global constants"
            )),
            Some(self.current_tok().unwrap().start.clone()),
            Some(self.current_tok().unwrap().end.clone()),
        );
        res
    }

    fn local_var_decl(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let mut mutable = false;
        let start = self.last_tok().unwrap().start.clone();

        if self.current_matches(
            TokenType::Identifier,
            Some("mut".to_string()),
        ) {
            self.advance(&mut res);
            mutable = true;
        }

        consume!(name_tok = Identifier, self, res);

        let name =
            name_tok.literal.as_ref().unwrap().clone();

        let mut type_annotation = None;

        if self.current_matches(TokenType::Colon, None) {
            self.advance(&mut res);

            let type_ = res.register(self.type_expr());
            ret_on_err!(res);
            type_annotation = Some(type_.unwrap());
        }

        if !self.current_matches(TokenType::Equals, None) {
            if !mutable {
                res.failure(
                    syntax_error(
                        "Cannot declare uninitialized local constant"
                            .to_string(),
                    ),
                    Some(self.last_tok().unwrap().end.clone()),
                    None,
                );
                return res;
            }

            if type_annotation.is_none() {
                res.failure(
                    syntax_error(
                        "Expected type annotation"
                            .to_string(),
                    ),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                    None,
                );
                return res;
            }

            res.success(new_mut_rc(EmptyLocalVarNode {
                identifier: name,
                type_: type_annotation.unwrap(),
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }

        consume!(Equals, self, res);

        let expr = res.register(self.expression());
        ret_on_err!(res);

        res.success(new_mut_rc(LocalVarNode {
            identifier: name_tok,
            value: expr.unwrap(),
            mutable,
            type_annotation,
            start,
        }));
        res
    }

    fn unary_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if let Some(tok) = self.current_tok() {
            if tok.token_type == TokenType::Not {
                consume!(self, res);
                let node = res.register(self.unary_expr());
                ret_on_err!(res);
                res.success(new_mut_rc(UnaryOpNode {
                    operator: tok,
                    rhs: node.unwrap(),
                }));
                return res;
            }
        }

        self.compound(None)
    }

    fn factor(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let tok = self.current_tok();

        if let Some(t) = tok {
            if t.token_type == TokenType::Sub {
                self.advance(&mut res);
                let factor = res.register(self.factor());
                ret_on_err!(res);
                res.success(new_mut_rc(UnaryOpNode {
                    operator: t,
                    rhs: factor.unwrap(),
                }));
                return res;
            }
        }

        self.unary_expr()
    }

    fn term(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.factor(),
            vec![
                (TokenType::Astrix, None),
                (TokenType::FSlash, None),
                (TokenType::Percent, None),
            ],
            |this| this.factor(),
        )
    }

    fn arithmetic_expr(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.term(),
            vec![
                (TokenType::Plus, None),
                (TokenType::Sub, None),
            ],
            |this| this.term(),
        )
    }

    fn method_call(
        &mut self,
        start: Position,
        base: MutRc<dyn Node>,
        name_tok: Token,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        self.advance(&mut res);
        let mut args = Vec::new();
        if !self
            .current_matches(TokenType::CloseParen, None)
        {
            loop {
                let arg = res.register(self.expression());
                ret_on_err!(res);
                args.push(arg.unwrap());
                if self
                    .current_matches(TokenType::Comma, None)
                {
                    self.advance(&mut res);
                } else {
                    break;
                }
            }
        }
        consume!(CloseParen, self, res);

        res.success(new_mut_rc(FnCallNode {
            object: Some(base),
            identifier: name_tok,
            args,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        return res;
    }

    fn compound(
        &mut self,
        base_option: Option<MutRc<dyn Node>>,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        let base;
        if base_option.is_some() {
            base = base_option.unwrap();
        } else {
            let atom = res.register(self.atom());
            ret_on_err!(res);
            base = atom.unwrap();
        }

        if self.current_matches(TokenType::Dot, None) {
            let start =
                self.current_tok().unwrap().start.clone();
            self.advance(&mut res);
            consume!(name_tok = Identifier, self, res);

            if self
                .current_matches(TokenType::OpenParen, None)
            {
                let base_option =
                    res.register(self.method_call(
                        start.clone(),
                        base,
                        name_tok.clone(),
                    ));
                ret_on_err!(res);
                return self
                    .compound(Some(base_option.unwrap()));
            }

            return self.compound(Some(new_mut_rc(
                FieldAccessNode {
                    base,
                    field_name: name_tok,
                    position: (
                        start,
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                },
            )));
        }

        res.success(base);
        res
    }

    fn function_call(
        &mut self,
        fn_identifier_tok: Token,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        let start = self.last_tok().unwrap().start;

        consume!(OpenParen, self, res);

        if let Some(t) = self.current_tok() {
            // fn(), no arguments
            if t.token_type == TokenType::CloseParen {
                self.advance(&mut res);
                res.success(new_mut_rc(FnCallNode {
                    object: None,
                    identifier: fn_identifier_tok,
                    args: Vec::new(),
                    position: (
                        start,
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                }));
                return res;
            }
        }

        let mut args = Vec::new();

        loop {
            let parameter = res.register(self.expression());
            ret_on_err!(res);

            args.push(parameter.unwrap());

            if let Some(t) = self.current_tok() {
                if t.token_type == TokenType::CloseParen {
                    break;
                } else if t.token_type == TokenType::Comma {
                    self.advance(&mut res);
                } else {
                    res.failure(
                        syntax_error(format!(
                            "Expected ',' or ')', found '{}'",
                            t.str()
                        )),
                        Some(fn_identifier_tok.start),
                        Some(fn_identifier_tok.end),
                    );
                    return res;
                }
            } else {
                res.failure(
                    syntax_error(
                        "Expected ',' or ')', found EOF"
                            .to_owned(),
                    ),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone()
                            .advance(None),
                    ),
                    None,
                );
                return res;
            }
        }

        consume!(CloseParen, self, res);

        res.success(new_mut_rc(FnCallNode {
            object: None,
            identifier: fn_identifier_tok,
            args,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }

    fn assign(&mut self, id_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(Equals, self, res);

        let value = res.register(self.expression());
        ret_on_err!(res);

        res.success(new_mut_rc(MutateVar {
            identifier: id_tok,
            value: value.unwrap(),
        }));

        res
    }

    fn atom(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let token_option = self.current_tok();
        if token_option.is_none() {
            res.failure(
                syntax_error("Unexpected EOF".to_string()),
                Some(
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                ),
                None,
            );
            return res;
        }
        let tok = token_option.unwrap();

        match tok.token_type {
            TokenType::Int => {
                self.advance(&mut res);
                let int_str = tok.literal.unwrap();
                let int_res = int_str.parse::<i64>();
                if int_res.is_err() {
                    res.failure(
                        numeric_overflow(format!(
                            "Invalid integer literal: '{}'",
                            int_str
                        )),
                        Some(tok.start),
                        Some(tok.end),
                    );
                    return res;
                }
                res.success(new_mut_rc(IntNode {
                    value: int_res.unwrap(),
                    position: (
                        tok.start.clone(),
                        tok.end.clone(),
                    ),
                }));
            }
            TokenType::String => {
                self.advance(&mut res);
                res.success(new_mut_rc(StrNode {
                    value: tok,
                }));
            }
            TokenType::OpenParen => {
                self.advance(&mut res);

                let expr = res.register(self.expression());
                ret_on_err!(res);

                consume!(CloseParen, self, res);

                res.success(expr.unwrap());
            }
            TokenType::Identifier => {
                self.advance(&mut res);
                if tok.clone().literal.unwrap() == "new" {
                    return self.class_init();
                }
                if tok.clone().literal.unwrap() == "false"
                    || tok.clone().literal.unwrap()
                        == "true"
                {
                    res.success(new_mut_rc(BoolNode {
                        value: tok.clone().literal.unwrap()
                            == "true",
                        position: (
                            tok.start.clone(),
                            tok.end.clone(),
                        ),
                    }));
                    return res;
                }
                if let Some(next) = self.current_tok() {
                    if next.token_type
                        == TokenType::OpenParen
                    {
                        return self.function_call(tok);
                    }
                    if next.token_type == TokenType::Equals
                    {
                        return self.assign(tok);
                    }
                }

                res.success(new_mut_rc(SymbolAccess {
                    identifier: tok,
                }));
            }
            _ => {
                res.failure(
                    syntax_error(format!(
                        "Expected number, identifier, keyword, string or '(' but found '{}'",
                        tok.str()
                    )),
                    Some(tok.start.clone()),
                    None,
                );
            }
        };
        res
    }

    fn while_loop(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start;

        let mut condition = None;

        if !self.current_matches(TokenType::OpenBrace, None)
        {
            condition = res.register(self.expression());
            ret_on_err!(res);
            if condition.is_none() {
                res.failure(
                    syntax_error(
                        "Expected expression".to_owned(),
                    ),
                    Some(self.last_tok().unwrap().start),
                    Some(self.last_tok().unwrap().end),
                );
                return res;
            }
        }

        let statements = res.register(self.scope(true));
        ret_on_err!(res);

        self.add_end_statement(&mut res);
        ret_on_err!(res);

        res.success(new_mut_rc(WhileLoopNode {
            condition,
            statements: statements.unwrap(),
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }

    fn if_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start;

        let comparison = res.register(self.expression());
        ret_on_err!(res);

        let statements = res.register(self.scope(true));
        ret_on_err!(res);

        let mut else_body: Option<MutRc<dyn Node>> = None;

        if self.current_matches(
            TokenType::Identifier,
            Some("else".to_string()),
        ) {
            self.advance(&mut res);
            if self
                .current_matches(TokenType::OpenBrace, None)
            {
                else_body = res.register(self.scope(true));
                ret_on_err!(res);
            } else {
                let else_expr_option =
                    res.register(self.statement());
                ret_on_err!(res);
                else_body = Some(else_expr_option.unwrap());
            }
        }

        self.add_end_statement(&mut res);
        ret_on_err!(res);

        res.success(new_mut_rc(IfNode {
            comparison: comparison.unwrap(),
            body: statements.unwrap(),
            else_body,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }

    fn type_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(type_tok = Identifier, self, res);

        res.success(new_mut_rc(TypeNode {
            identifier: type_tok,
        }));
        res
    }

    fn parameter(&mut self) -> Result<Parameter, Error> {
        let mut res = ParseResults::new();

        result_consume!(identifier = Identifier, self, res);
        let start = self.last_tok().unwrap().start;

        let mut type_expr = None;
        if self.current_matches(TokenType::Colon, None) {
            result_consume!(Colon, self, res);
            type_expr = res.register(self.type_expr());
            result_ret_on_err!(res);
        }

        let mut default_value = None;
        if self.current_matches(TokenType::Equals, None) {
            self.advance(&mut res);
            let default_value_option =
                res.register(self.expression());
            result_ret_on_err!(res);
            default_value =
                Some(default_value_option.unwrap());
        }

        Ok(Parameter {
            identifier: identifier.literal.unwrap(),
            type_: type_expr,
            default_value,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        })
    }

    fn parameters(
        &mut self,
    ) -> Result<Vec<Parameter>, Error> {
        let mut res = ParseResults::new();

        let mut parameters = Vec::new();

        if self.current_matches(TokenType::CloseParen, None)
        {
            return Ok(parameters);
        }

        parameters.push(self.parameter()?);

        while let Some(next) = self.current_tok() {
            if next.token_type == TokenType::CloseParen {
                return Ok(parameters);
            }

            result_consume!(Comma, self, res);

            parameters.push(self.parameter()?);
        }

        Err(syntax_error(
            "Expected ',' or ')', found EOF".to_owned(),
        )
        .set_interval((
            self.last_tok()
                .unwrap()
                .end
                .clone()
                .advance(None),
            Position::unknown(),
        )))
    }

    fn fn_def_expr(
        &mut self,
        is_external: bool,
        class_name: Option<String>,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start;

        if self.current_tok().is_none() {
            res.failure(
                syntax_error(
                    "Expected identifier after 'fn'"
                        .to_owned(),
                ),
                Some(
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                ),
                None,
            );
            return res;
        }

        let identifier;
        if !self
            .current_matches(TokenType::Identifier, None)
        {
            if self
                .current_tok()
                .unwrap()
                .overload_op_id()
                .is_none()
            {
                res.failure(
                    syntax_error(format!(
                        "'{}' is not an overloadable operator",
                        self.current_tok().unwrap().str()
                    )),
                    Some(self.last_tok().unwrap().start.clone()),
                    Some(self.last_tok().unwrap().end.clone()),
                );
                return res;
            }
            if class_name.is_none() {
                res.failure(
                    syntax_error(format!(
                        "'{}' is not a valid function name",
                        self.current_tok().unwrap().str()
                    )),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .start
                            .clone(),
                    ),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                );
                return res;
            }
            identifier = self.current_tok().unwrap();
            self.advance(&mut res);
        } else {
            consume!(id_tok = Identifier, self, res);
            identifier = id_tok;
        }
        consume!(OpenParen, self, res);

        let mut other_params = true;

        if class_name.is_some() {
            consume!(self_tok = Identifier, self, res);
            if self_tok.literal.unwrap() != "self" {
                res.failure(
                    syntax_error(
                        "Expected 'self'".to_owned(),
                    ),
                    Some(self_tok.start),
                    Some(self_tok.end),
                );
                return res;
            }
            if self.current_matches(TokenType::Comma, None)
            {
                consume!(Comma, self, res);
            } else {
                other_params = false;

                if self
                    .current_matches(TokenType::Colon, None)
                {
                    res.failure(syntax_error(format!(
                            "Cannot give type annotation to parameter 'self' on method '{}'",
                            identifier.str()
                        )),
                        Some(self.last_tok().unwrap().start),
                        Some(self.current_tok().unwrap().end),
                    );
                    return res;
                }
            }
        }
        let mut params = Ok(vec![]);
        if other_params {
            params = self.parameters();
            if params.is_err() {
                res.failure(
                    params.err().unwrap(),
                    None,
                    None,
                );
                return res;
            }
        }

        let mut params = params.unwrap();

        if class_name.is_some() {
            // insert 'self' parameter separately as it's type is not given
            params.insert(
                0,
                Parameter {
                    identifier: "self".to_owned(),
                    type_: Some(new_mut_rc(TypeNode {
                        identifier: Token {
                            token_type:
                                TokenType::Identifier,
                            literal: Some(
                                class_name.clone().unwrap(),
                            ),
                            start: Position::unknown(),
                            end: Position::unknown(),
                        },
                    })),
                    default_value: None,
                    position: Position::unknown_interval(),
                },
            );
        }

        consume!(CloseParen, self, res);

        let mut ret_type: MutRc<dyn Node> =
            new_mut_rc(TypeNode {
                identifier: Token::new(
                    TokenType::Identifier,
                    Some("Void".to_string()),
                    Position::unknown(),
                    Position::unknown(),
                ),
            });

        if !self.current_matches(TokenType::OpenBrace, None)
            && !self.current_matches(TokenType::Comma, None)
            && !self.current_matches(
                TokenType::EndStatement,
                None,
            )
            && self.current_tok().is_some()
        {
            let ret_type_option =
                res.register(self.type_expr());
            ret_on_err!(res);
            ret_type = ret_type_option.unwrap();
        }

        if !self.current_matches(TokenType::OpenBrace, None)
        {
            res.success(new_mut_rc(FnDeclarationNode {
                identifier,
                params_scope: Context::new(),
                ret_type,
                is_external,
                params,
                body: None,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
                class: None,
            }));
            return res;
        }

        if is_external {
            res.failure(
                syntax_error(
                    "External functions cannot have a body"
                        .to_owned(),
                ),
                Some(start),
                Some(
                    self.current_tok().unwrap().end.clone(),
                ),
            );
            return res;
        }

        let body = res.register(self.scope(false));
        ret_on_err!(res);

        if class_name.is_none() {
            self.add_end_statement(&mut res);
            ret_on_err!(res);
        }

        res.success(new_mut_rc(FnDeclarationNode {
            identifier,
            params_scope: Context::new(),
            ret_type,
            params,
            body: Some(body.unwrap()),
            is_external: false,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
            class: None,
        }));
        res
    }

    fn return_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start;

        if self.current_matches(TokenType::EndStatement, None)
            // So far, no expression can start with '}', so we might as well
            // allow returns to not require a semi-colon if they are the last statement
            // in a scope; so `fn a() { return }` is fine
            // Doesn't check that we are in a scope though,
            // so `fn a() { return } }` wouldn't work if '}' was a valid expression
            || self.current_matches(TokenType::CloseBrace, None)
        {
            res.success(new_mut_rc(ReturnNode {
                value: None,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }

        let expr = res.register(self.expression());
        ret_on_err!(res);

        res.success(new_mut_rc(ReturnNode {
            value: Some(expr.unwrap()),
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }

    fn class_def_field(
        &mut self,
    ) -> Result<ClassField, Error> {
        let mut res = ParseResults::new();

        result_consume!(identifier = Identifier, self, res);
        result_consume!(Colon, self, res);

        let type_expr = res.register(self.type_expr());
        result_ret_on_err!(res);

        Ok(ClassField {
            identifier: identifier.literal.unwrap(),
            type_: type_expr.unwrap(),
        })
    }

    fn class_def_expr(
        &mut self,
        is_primitive: bool,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start.clone();

        consume!(id_tok = Identifier, self, res);
        let identifier = id_tok.clone().literal.unwrap();

        let mut fields = Vec::new();
        let mut methods = Vec::new();

        if !self.current_matches(TokenType::OpenBrace, None)
        {
            res.success(new_mut_rc(ClassDeclarationNode {
                identifier: id_tok,
                fields,
                methods,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
                is_primitive,
            }));
            return res;
        }

        consume!(OpenBrace, self, res);

        while let Some(next) = self.current_tok() {
            if next.token_type == TokenType::CloseBrace {
                break;
            }

            if self.current_matches(
                TokenType::Identifier,
                Some("fn".to_string()),
            ) {
                consume!(self, res);

                let fn_decl =
                    res.register(self.fn_def_expr(
                        false,
                        Some(identifier.clone()),
                    ));
                ret_on_err!(res);

                // assume that a FnDeclarationNode is returned from fn_expr
                // and dangerously cast to the concrete type
                unsafe {
                    let fn_ = &*(&fn_decl as *const dyn Any
                        as *const Option<
                            MutRc<FnDeclarationNode>,
                        >);
                    methods.push(fn_.clone().unwrap());
                }

                if self.current_matches(
                    TokenType::CloseBrace,
                    None,
                ) {
                    break;
                }
                continue;
            }

            if self.current_matches(
                TokenType::Identifier,
                Some("extern".to_string()),
            ) {
                consume!(self, res);
                consume!(Identifier, self, res);

                let fn_decl =
                    res.register(self.fn_def_expr(
                        true,
                        Some(identifier.clone()),
                    ));
                ret_on_err!(res);

                // assume that a FnDeclarationNode is returned from fn_expr
                // and dangerously cast to the concrete type
                unsafe {
                    let fn_ = &*(&fn_decl as *const dyn Any
                        as *const Option<
                            MutRc<FnDeclarationNode>,
                        >);
                    methods.push(fn_.clone().unwrap());
                }

                if self.current_matches(
                    TokenType::CloseBrace,
                    None,
                ) {
                    break;
                }
                consume!(Comma, self, res);
                continue;
            }

            let field = self.class_def_field();
            if field.is_err() {
                // don't override more precise position of error
                res.failure(
                    field.err().unwrap(),
                    None,
                    None,
                );
                return res;
            }
            fields.push(field.unwrap());
            if self.current_matches(
                TokenType::CloseBrace,
                None,
            ) {
                break;
            }

            consume!(Comma, self, res);
        }

        consume!(CloseBrace, self, res);
        self.add_end_statement(&mut res);
        ret_on_err!(res);

        if is_primitive && fields.len() > 0 {
            res.failure(
                syntax_error(
                    "Primitive classes (pclass) cannot have fields"
                        .to_owned(),
                ),
                Some(self.last_tok().unwrap().start.clone()),
                Some(self.last_tok().unwrap().end.clone()),
            );
            return res;
        }

        res.success(new_mut_rc(ClassDeclarationNode {
            identifier: id_tok,
            fields,
            methods,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
            is_primitive,
        }));
        res
    }

    fn class_init_field(
        &mut self,
    ) -> Result<(String, MutRc<dyn Node>), Error> {
        let mut res = ParseResults::new();

        result_consume!(identifier = Identifier, self, res);
        result_consume!(Colon, self, res);

        let value = res.register(self.expression());
        result_ret_on_err!(res);

        Ok((identifier.literal.unwrap(), value.unwrap()))
    }

    fn class_init(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start.clone();

        consume!(identifier_tok = Identifier, self, res);

        let mut fields = Vec::new();

        if !self.current_matches(TokenType::OpenBrace, None)
        {
            res.success(new_mut_rc(ClassInitNode {
                identifier: identifier_tok,
                fields,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
            }));
            return res;
        }

        consume!(OpenBrace, self, res);

        while let Some(next) = self.current_tok() {
            if next.token_type == TokenType::CloseBrace {
                break;
            }

            let field = self.class_init_field();
            if field.is_err() {
                // don't override more precise position of error
                res.failure(
                    field.err().unwrap(),
                    None,
                    None,
                );
                return res;
            }
            fields.push(field.unwrap());

            if self.current_matches(
                TokenType::CloseBrace,
                None,
            ) {
                break;
            }
            self.consume(&mut res, TokenType::Comma);
            ret_on_err!(res);
        }

        self.consume(&mut res, TokenType::CloseBrace);
        ret_on_err!(res);

        res.success(new_mut_rc(ClassInitNode {
            identifier: identifier_tok,
            fields,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }
}
