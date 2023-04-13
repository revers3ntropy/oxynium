use crate::ast::bin_op::BinOpNode;
use crate::ast::bool::BoolNode;
use crate::ast::class_declaration::{
    ClassDeclarationNode, ClassField,
};
use crate::ast::class_field_access::FieldAccessNode;
use crate::ast::class_init::ClassInitNode;
use crate::ast::empty_global_const_decl::EmptyGlobalConstNode;
use crate::ast::empty_local_var_decl::EmptyLocalVarNode;
use crate::ast::fn_call::FnCallNode;
use crate::ast::fn_declaration::{
    FnDeclarationNode, Parameter,
};
use crate::ast::global_const_decl::GlobalConstNode;
use crate::ast::int::IntNode;
use crate::ast::local_var_decl::LocalVarNode;
use crate::ast::macro_call::MacroCallNode;
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
use crate::ast::type_expr_generic::GenericTypeNode;
use crate::ast::type_expr_optional::OptionalTypeNode;
use crate::ast::unary_op::UnaryOpNode;
use crate::ast::AstNode;
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
                        .start
                        .clone()
                        .advance(None),
                ),
                None,
            );
            return $res;
        }
    };
    ($t:ident, $self:expr, $res:expr) => {
        $self.consume(&mut $res, TokenType::$t);
        if $res.error.is_some() {
            return $res;
        }
    };
    ($id:ident = $t:ident, $self:expr, $res:expr) => {
        let $id = $self.consume(&mut $res, TokenType::$t);
        if $res.error.is_some() {
            return $res;
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
            res.node = Some(new_mut_rc(PassNode {
                position: Position::unknown_interval(),
            }));
            return res;
        }

        let expr = res.register(self.statements());
        ret_on_err!(res);

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

        res.success(expr.unwrap());
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
                    "expected token of type `{:?}`, found `{}`",
                    tok_type,
                    tok.str()
                )),
                Some(tok.start.clone()),
                None,
            );
        } else {
            res.failure(
                syntax_error(format!(
                    "expected `{:?}`, found EOF",
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
            if tok.token_type != tok_type {
                return false;
            }
            if let Some(value) = value {
                return tok.literal.is_some()
                    && tok.literal.unwrap() == value;
            }
            return true;
        }
        false
    }

    fn next_matches(
        &self,
        tok_type: TokenType,
        value: Option<String>,
    ) -> bool {
        if let Some(tok) = self.next_tok() {
            if tok.token_type != tok_type {
                return false;
            }
            if let Some(value) = value {
                return tok.literal.is_some()
                    && tok.literal.unwrap() == value;
            }
            return true;
        }
        false
    }

    fn next_tok(&self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() - 1 {
            return None;
        }
        Some(self.tokens[self.tok_idx + 1].clone())
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
    ) -> usize {
        let mut i = 0;
        while self
            .current_matches(TokenType::EndStatement, None)
        {
            self.advance(res);
            i += 1;
        }
        i
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
        let mut statements: Vec<MutRc<dyn AstNode>> =
            Vec::new();
        self.clear_end_statements(&mut res);

        let first_stmt = res.register(self.statement());

        ret_on_err!(res);
        if first_stmt.is_none() {
            return res;
        }

        statements.push(first_stmt.unwrap());

        loop {
            if self.clear_end_statements(&mut res) == 0 {
                break;
            }

            // This is dependant on the fact that the only tokens that
            // can occur after the end of `Statements` is CloseBrace,
            // or the end of the file.
            if self.current_tok().is_none()
                || self.current_matches(
                    TokenType::CloseBrace,
                    None,
                )
            {
                break;
            }

            let statement = res.register(self.statement());
            ret_on_err!(res);
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
                    "expected statement or '}'".to_string(),
                ),
                Some(start.clone().advance(None)),
                None,
            );
            return res;
        }

        let mut statements: Option<MutRc<dyn AstNode>> =
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
                ctx: None,
                body: statements.unwrap(),
                position: (start, end),
                err_source: None,
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
            res.node = res.register(
                self.global_const_decl(false, false),
            );
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
            Some("def".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.function_declaration(
                    false, false, None,
                ));
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("class".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.class_declaration(false, false),
            );
            return res;
        }
        if self.current_matches(
            TokenType::Identifier,
            Some("primitive".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.class_declaration(true, false),
            );
            return res;
        }

        if self.current_matches(
            TokenType::Identifier,
            Some("export".to_string()),
        ) {
            self.advance(&mut res);
            return self.export_something();
        }

        if self.current_matches(
            TokenType::Identifier,
            Some("extern".to_string()),
        ) {
            self.advance(&mut res);
            return self.extern_something();
        }
        self.expression()
    }

    fn export_something(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let mut is_extern = false;
        if self.current_matches(
            TokenType::Identifier,
            Some("extern".to_string()),
        ) {
            self.advance(&mut res);
            is_extern = true;
        }

        if self.current_matches(
            TokenType::Identifier,
            Some("def".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.function_declaration(
                    is_extern, true, None,
                ));
        } else if self.current_matches(
            TokenType::Identifier,
            Some("class".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.class_declaration(false, true),
            );
        } else if self.current_matches(
            TokenType::Identifier,
            Some("primitive".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.class_declaration(true, true),
            );
        } else if self.current_matches(
            TokenType::Identifier,
            Some("const".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.global_const_decl(is_extern, true),
            );
        } else {
            let current =
                self.current_tok().clone().unwrap();
            res.failure(
                syntax_error(
                    "expected 'fn', 'class' or 'primitive'"
                        .to_string(),
                ),
                Some(current.start.clone()),
                Some(current.end.clone()),
            );
            return res;
        }

        res
    }

    fn extern_something(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if self.current_matches(
            TokenType::Identifier,
            Some("def".to_string()),
        ) {
            self.advance(&mut res);
            res.node =
                res.register(self.function_declaration(
                    true, false, None,
                ));
        } else if self.current_matches(
            TokenType::Identifier,
            Some("const".to_string()),
        ) {
            self.advance(&mut res);
            res.node = res.register(
                self.global_const_decl(true, false),
            );
        } else {
            res.failure(
                syntax_error(
                    "expected 'fn', 'var' or 'const'"
                        .to_string(),
                ),
                Some(self.last_tok().unwrap().end),
                None,
            );
        }
        return res;
    }

    fn expression(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        self.clear_end_statements(&mut res);
        ret_on_err!(res);
        self.bin_op(
            |this| this.none_coal_expr(),
            vec![
                (TokenType::And, None),
                (TokenType::Or, None),
            ],
            |this| this.none_coal_expr(),
        )
    }

    fn none_coal_expr(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.comparison_expr(),
            vec![(TokenType::DblQM, None)],
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
        is_exported: bool,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start.clone();

        consume!(name = Identifier, self, res);

        let mut type_: Option<MutRc<dyn AstNode>> = None;

        if self.current_matches(TokenType::Colon, None) {
            self.advance(&mut res);
            type_ = res.register(self.type_expr(None));
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
                is_exported,
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
            let value = tok.literal.unwrap().parse::<i64>();
            if value.is_err() {
                res.failure(
                    numeric_overflow(
                        "Invalid integer literal"
                            .to_string(),
                    ),
                    Some(tok.start.clone()),
                    Some(tok.end.clone()),
                );
                return res;
            }
            let value = value.unwrap();
            res.success(new_mut_rc(GlobalConstNode {
                identifier: name,
                value,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
                is_exported,
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
                is_exported,
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

            let type_ = res.register(self.type_expr(None));
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

            self.add_end_statement(&mut res);
            ret_on_err!(res);

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

    fn compound(
        &mut self,
        base_option: Option<MutRc<dyn AstNode>>,
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

            consume!(Dot, self, res);
            consume!(name_tok = Identifier, self, res);

            if self
                .current_matches(TokenType::OpenParen, None)
            {
                let base_option =
                    res.register(self.function_call(
                        start.clone(),
                        Some(base),
                        name_tok.clone(),
                        vec![],
                    ));
                ret_on_err!(res);
                return self
                    .compound(Some(base_option.unwrap()));
            }

            if self.current_matches(TokenType::Not, None) {
                let base_option =
                    res.register(self.identifier_bang(
                        start,
                        Some(base),
                        self.last_tok().unwrap(),
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

    fn args(
        &mut self,
    ) -> Result<Vec<MutRc<dyn AstNode>>, Error> {
        let mut args = Vec::new();
        let mut res = ParseResults::new();

        loop {
            let parameter = res.register(self.expression());
            result_ret_on_err!(res);

            args.push(parameter.unwrap());

            if let Some(t) = self.current_tok() {
                if t.token_type == TokenType::CloseParen {
                    break;
                } else if t.token_type == TokenType::Comma {
                    result_consume!(Comma, self, res);
                } else {
                    return Err(syntax_error(format!(
                        "expected ',' or ')', found '{}'",
                        t.str()
                    ))
                    .set_interval(
                        self.last_tok().unwrap().interval(),
                    ));
                }
            } else {
                return Err(syntax_error(
                    "expected ',' or ')', found EOF"
                        .to_owned(),
                )
                .set_interval((
                    self.last_tok()
                        .unwrap()
                        .end
                        .clone()
                        .advance(None),
                    Position::unknown(),
                )));
            }
        }

        Ok(args)
    }

    /// IdentifierBang ::= '!' '<' GenericArgs '>' ('.' Identifier IdentifierBang? )? '(' Args ')'
    ///
    /// Only ever for either calling a function or calling a method
    fn identifier_bang(
        &mut self,
        start: Position,
        base: Option<MutRc<dyn AstNode>>,
        identifier: Token,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(Not, self, res);
        consume!(LT, self, res);

        let generic_args_result = self.generic_args();
        if let Some(err) = generic_args_result.clone().err()
        {
            res.failure(err, None, None);
            return res;
        }
        let generic_args = generic_args_result.unwrap();
        consume!(GT, self, res);

        // `c!<T>(...)`
        if self.current_matches(TokenType::OpenParen, None)
        {
            return self.function_call(
                start,
                base,
                identifier,
                generic_args,
            );
        }

        let base =
            base.unwrap_or(new_mut_rc(SymbolAccess {
                identifier: identifier.clone(),
            }));

        consume!(Dot, self, res);
        consume!(name_tok = Identifier, self, res);

        if self.current_matches(TokenType::OpenParen, None)
        {
            // `C!<T>.f(...)`
            return self.function_call(
                start.clone(),
                Some(new_mut_rc(GenericTypeNode {
                    base,
                    generic_args,
                    position: (
                        start,
                        self.last_tok()
                            .clone()
                            .unwrap()
                            .end,
                    ),
                })),
                name_tok,
                vec![],
            );
        }

        consume!(Not, self, res);
        consume!(LT, self, res);

        let generic_args_result = self.generic_args();
        if let Some(err) = generic_args_result.clone().err()
        {
            res.failure(err, None, None);
            return res;
        }
        let f_call_generic_args =
            generic_args_result.unwrap();
        consume!(GT, self, res);

        // `C!<T1>.f!<T2>(...)`
        self.function_call(
            start.clone(),
            Some(new_mut_rc(GenericTypeNode {
                base,
                generic_args,
                position: (
                    start,
                    self.last_tok().unwrap().end,
                ),
            })),
            name_tok,
            f_call_generic_args,
        )
    }

    /// FunctionCall ::= '(' Args ')'
    fn function_call(
        &mut self,
        start: Position,
        base: Option<MutRc<dyn AstNode>>,
        name_tok: Token,
        generic_args: Vec<MutRc<dyn AstNode>>,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(OpenParen, self, res);

        if let Some(t) = self.current_tok() {
            // fn(), no arguments
            if t.token_type == TokenType::CloseParen {
                self.advance(&mut res);
                res.success(new_mut_rc(FnCallNode {
                    object: base,
                    identifier: name_tok,
                    args: Vec::new(),
                    generic_args,
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

        let args = self.args();
        if args.is_err() {
            res.failure(args.err().unwrap(), None, None);
            return res;
        }
        let args = args.unwrap();

        consume!(CloseParen, self, res);

        res.success(new_mut_rc(FnCallNode {
            object: base,
            identifier: name_tok,
            args,
            generic_args,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
        }));
        res
    }

    fn op_assign(&mut self, id_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        let operator = self.current_tok().unwrap();
        consume!(self, res);

        consume!(Equals, self, res);

        let value = res.register(self.expression());
        ret_on_err!(res);

        res.success(new_mut_rc(MutateVar {
            identifier: id_tok.clone(),
            value: new_mut_rc(BinOpNode {
                lhs: new_mut_rc(SymbolAccess {
                    identifier: id_tok,
                }),
                operator,
                rhs: value.unwrap(),
            }),
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
                if tok.clone().literal.unwrap() == "typeof"
                {
                    let type_of =
                        res.register(self.expression());
                    ret_on_err!(res);

                    res.success(new_mut_rc(UnaryOpNode {
                        operator: tok,
                        rhs: type_of.unwrap(),
                    }));
                    return res;
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
                        return self.function_call(
                            tok.start.clone(),
                            None,
                            tok,
                            vec![],
                        );
                    }

                    if self.next_matches(
                        TokenType::Equals,
                        None,
                    ) {
                        if TokenType::op_assign_operators()
                            .contains(&next.token_type)
                        {
                            return self.op_assign(tok);
                        }
                    }

                    if next.token_type == TokenType::Not {
                        return self.identifier_bang(
                            tok.start.clone(),
                            None,
                            tok,
                        );
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
            TokenType::Hash => {
                // macro
                self.advance(&mut res);
                return self.macro_call();
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

    /// MacroCall ::= Identifier OpenParen <Arguments> CloseParen
    fn macro_call(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(id_tok = Identifier, self, res);
        let start = id_tok.start.clone();

        let args: Vec<MutRc<dyn AstNode>>;

        if self.current_matches(TokenType::OpenParen, None)
        {
            self.advance(&mut res);
            ret_on_err!(res);

            if self.current_matches(
                TokenType::CloseParen,
                None,
            ) {
                self.advance(&mut res);
                ret_on_err!(res);
                args = Vec::new();
            } else {
                let args_res = self.args();
                if args_res.is_err() {
                    res.failure(
                        args_res.err().unwrap(),
                        None,
                        None,
                    );
                    return res;
                }
                args = args_res.unwrap();

                consume!(CloseParen, self, res);
            }
        } else {
            let arg = res.register(self.expression());
            ret_on_err!(res);
            args = vec![arg.unwrap()];
        }

        res.success(new_mut_rc(MacroCallNode {
            identifier: id_tok,
            args,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
            resolved: None,
        }));
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

        let mut else_body: Option<MutRc<dyn AstNode>> =
            None;

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

    /// SimpleTypeExpr ::= Identifier (LT TypeExpr (Comma TypeExpr)* GT)?
    fn simple_or_generic_type_expr(
        &mut self,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        consume!(type_tok = Identifier, self, res);

        if !self.current_matches(TokenType::LT, None) {
            res.success(new_mut_rc(TypeNode {
                identifier: type_tok,
            }));
            return res;
        }

        // is generic type
        self.advance(&mut res);
        let mut generic_args = Vec::new();
        loop {
            let generic_type =
                res.register(self.type_expr(None));
            ret_on_err!(res);
            generic_args.push(generic_type.unwrap());
            if self.current_matches(TokenType::GT, None) {
                break;
            }
            consume!(Comma, self, res);
        }

        consume!(GT, self, res);

        res.success(new_mut_rc(GenericTypeNode {
            base: new_mut_rc(TypeNode {
                identifier: type_tok.clone(),
            }),
            generic_args,
            position: (
                type_tok.start,
                self.last_tok().unwrap().end,
            ),
        }));
        res
    }

    /// TypeExpr ::= SimpleTypeExpr QM?
    fn type_expr(
        &mut self,
        base: Option<MutRc<dyn AstNode>>,
    ) -> ParseResults {
        let mut res = ParseResults::new();

        let value;
        if let Some(base) = base {
            value = base;
        } else {
            let simple_type_res = res.register(
                self.simple_or_generic_type_expr(),
            );
            ret_on_err!(res);
            value = simple_type_res.unwrap();
        }

        if self.current_matches(TokenType::DblQM, None) {
            // eg `Int??`
            // which should doubly wrap the type in an optional

            consume!(DblQM, self, res);

            let start = value.clone().borrow().pos().0;
            return self.type_expr(Some(new_mut_rc(
                OptionalTypeNode {
                    value: new_mut_rc(OptionalTypeNode {
                        value,
                        position: (
                            start.clone(),
                            self.last_tok().unwrap().end,
                        ),
                    }),
                    position: (
                        start,
                        self.last_tok().unwrap().end,
                    ),
                },
            )));
        }

        if self.current_matches(TokenType::QM, None) {
            consume!(QM, self, res);

            let start = value.clone().borrow().pos().0;
            return self.type_expr(Some(new_mut_rc(
                OptionalTypeNode {
                    value,
                    position: (
                        start,
                        self.last_tok().unwrap().end,
                    ),
                },
            )));
        }
        res.success(value);
        res
    }

    /// Parameter ::= Identifier (Colon TypeExpr)? (Equals Expression)?
    fn parameter(&mut self) -> Result<Parameter, Error> {
        let mut res = ParseResults::new();

        result_consume!(identifier = Identifier, self, res);
        let start = self.last_tok().unwrap().start;

        let mut type_expr = None;
        if self.current_matches(TokenType::Colon, None) {
            result_consume!(Colon, self, res);
            type_expr = res.register(self.type_expr(None));
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

            if self.current_matches(
                TokenType::CloseParen,
                None,
            ) {
                return Ok(parameters);
            }

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

    fn generic_parameter(
        &mut self,
    ) -> Result<Token, Error> {
        let mut res = ParseResults::new();
        result_consume!(identifier = Identifier, self, res);
        Ok(identifier)
    }

    fn generic_parameters(
        &mut self,
    ) -> Result<Vec<Token>, Error> {
        let mut res = ParseResults::new();

        let mut parameters = Vec::new();

        if self.current_matches(TokenType::GT, None) {
            return Ok(parameters);
        }

        parameters.push(self.generic_parameter()?);

        while let Some(next) = self.current_tok() {
            if next.token_type == TokenType::GT {
                return Ok(parameters);
            }

            result_consume!(Comma, self, res);

            if self.current_matches(TokenType::GT, None) {
                return Ok(parameters);
            }

            parameters.push(self.generic_parameter()?);
        }

        Err(syntax_error(
            "Expected ',' or '>', found EOF".to_owned(),
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

    /// FuncDef ::= Identifier ('.' Identifier)? ('<' Identifier (',' Identifier)*'>')? '(' Parameters? ')' Type? '{' Scope '}'
    fn function_declaration(
        &mut self,
        is_external: bool,
        is_exported: bool,
        mut class_name: Option<String>,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start;

        let should_add_end_stmts = class_name.is_none();
        let mut is_external_method = false;

        // `class C { export fn f() {} }` not allowed
        assert!(!(is_exported && class_name.is_some()));

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

        if self.next_matches(TokenType::Dot, None) {
            if class_name.is_some() {
                res.failure(
                    syntax_error(
                        "Unexpected '.'".to_owned(),
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
            class_name = Some(
                self.current_tok()
                    .unwrap()
                    .literal
                    .clone()
                    .unwrap(),
            );
            self.advance(&mut res);
            self.advance(&mut res);
            ret_on_err!(res);
            is_external_method = true;
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

        let mut generic_parameters = vec![];
        if self.current_matches(TokenType::LT, None) {
            consume!(LT, self, res);
            let generic_parameters_option = res
                .register_result(self.generic_parameters());
            ret_on_err!(res);
            generic_parameters =
                generic_parameters_option.unwrap();
            consume!(GT, self, res);
        }

        consume!(OpenParen, self, res);

        let mut other_params = true;

        let mut self_param_interval =
            Position::unknown_interval();

        let mut is_static_method = false;
        if class_name.is_some() {
            if self.current_matches(
                TokenType::Identifier,
                Some("self".to_string()),
            ) {
                consume!(s = Identifier, self, res);
                self_param_interval = s.interval();
                if self
                    .current_matches(TokenType::Comma, None)
                {
                    consume!(Comma, self, res);
                } else {
                    other_params = false;

                    // catch what might be a common mistake
                    if self.current_matches(
                        TokenType::Colon,
                        None,
                    ) {
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
            } else {
                is_static_method = true;
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

        if class_name.is_some() && !is_static_method {
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
                            start: self_param_interval
                                .0
                                .clone(),
                            end: self_param_interval
                                .1
                                .clone(),
                        },
                    })),
                    default_value: None,
                    position: self_param_interval,
                },
            );
        }

        consume!(CloseParen, self, res);

        let mut ret_type: MutRc<dyn AstNode> =
            new_mut_rc(TypeNode {
                identifier: Token::new(
                    TokenType::Identifier,
                    Some("Void".to_string()),
                    Position::unknown(),
                    Position::unknown(),
                ),
            });

        // needs updating when the possible values after a return type
        // change. For now:
        // - EOF for, well, EOF
        // - Comma for undefined method
        // - EndStatement for undefined function
        // - OpenBrace for the function definition
        if !self.current_matches(TokenType::OpenBrace, None)
            && !self.current_matches(TokenType::Comma, None)
            && !self.current_matches(
                TokenType::EndStatement,
                None,
            )
            && self.current_tok().is_some()
        {
            let ret_type_option =
                res.register(self.type_expr(None));
            ret_on_err!(res);
            ret_type = ret_type_option.unwrap();
        }

        if !self.current_matches(TokenType::OpenBrace, None)
        {
            if is_external_method {
                res.failure(
                    syntax_error(format!(
                        "Expected method body for method '{}'",
                        identifier.str()
                    )),
                    Some(self.last_tok().unwrap().start),
                    Some(self.last_tok().unwrap().end),
                );
                return res;
            }
            // edge case for end-of-statement insertion:
            // extern fn a () Option<Int>
            // this_is_the_same_line()
            if is_external && class_name.is_none() {
                self.add_end_statement(&mut res);
                ret_on_err!(res);
            }

            res.success(new_mut_rc(FnDeclarationNode {
                identifier,
                params_scope: None,
                ret_type,
                is_external,
                params,
                body: None,
                generic_parameters,
                position: (
                    start,
                    self.last_tok().unwrap().end.clone(),
                ),
                class_name,
                has_usage: false,
                is_exported,
                absolute_path: None,
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

        if should_add_end_stmts {
            self.add_end_statement(&mut res);
            ret_on_err!(res);
        }

        res.success(new_mut_rc(FnDeclarationNode {
            identifier,
            params_scope: None,
            ret_type,
            params,
            body: Some(body.unwrap()),
            generic_parameters,
            is_external: false,
            position: (
                start,
                self.last_tok().unwrap().end.clone(),
            ),
            class_name,
            has_usage: false,
            is_exported,
            absolute_path: None,
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

        let type_expr = res.register(self.type_expr(None));
        result_ret_on_err!(res);

        Ok(ClassField {
            identifier: identifier.literal.unwrap(),
            type_: type_expr.unwrap(),
        })
    }

    fn class_declaration(
        &mut self,
        is_primitive: bool,
        is_exported: bool,
    ) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start.clone();

        consume!(id_tok = Identifier, self, res);
        let identifier = id_tok.clone().literal.unwrap();

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        let mut generic_parameters = Vec::new();

        if self.current_matches(TokenType::LT, None) {
            self.advance(&mut res);
            let generic_params_res =
                self.generic_parameters();
            if generic_params_res.is_err() {
                res.failure(
                    generic_params_res.err().unwrap(),
                    Some(start.clone()),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                );
                return res;
            }
            generic_parameters =
                generic_params_res.unwrap();

            consume!(GT, self, res);
        }

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
                generic_parameters,
                generics_ctx: None,
                is_exported,
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
                Some("def".to_string()),
            ) {
                consume!(self, res);

                let fn_decl = res.register(
                    self.function_declaration(
                        false,
                        false,
                        Some(identifier.clone()),
                    ),
                );
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

                if self
                    .current_matches(TokenType::Comma, None)
                {
                    consume!(self, res);
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

                let fn_decl = res.register(
                    self.function_declaration(
                        true,
                        false,
                        Some(identifier.clone()),
                    ),
                );
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
            generic_parameters,
            generics_ctx: None,
            is_exported,
        }));
        res
    }

    fn class_init_field(
        &mut self,
    ) -> Result<(String, MutRc<dyn AstNode>), Error> {
        let mut res = ParseResults::new();

        result_consume!(identifier = Identifier, self, res);

        if !self.current_matches(TokenType::Colon, None) {
            // `new C { a }` should be equivalent to `new C { a: a }`
            return Ok((
                identifier.clone().literal.unwrap(),
                new_mut_rc(SymbolAccess { identifier }),
            ));
        }

        result_consume!(Colon, self, res);

        let value = res.register(self.expression());
        result_ret_on_err!(res);

        Ok((identifier.literal.unwrap(), value.unwrap()))
    }

    fn generic_args(
        &mut self,
    ) -> Result<Vec<MutRc<dyn AstNode>>, Error> {
        let mut res = ParseResults::new();
        let mut args = Vec::new();

        loop {
            let arg = res.register(self.type_expr(None));
            result_ret_on_err!(res);
            args.push(arg.unwrap());

            if self.current_matches(TokenType::Comma, None)
            {
                self.advance(&mut res);
            } else {
                break;
            }
        }

        Ok(args)
    }

    fn class_init(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let start = self.last_tok().unwrap().start.clone();

        consume!(identifier_tok = Identifier, self, res);
        let mut generic_args = Vec::new();

        if self.current_matches(TokenType::LT, None) {
            self.advance(&mut res);
            let generic_args_res = self.generic_args();
            if generic_args_res.is_err() {
                res.failure(
                    generic_args_res.err().unwrap(),
                    Some(start.clone()),
                    Some(
                        self.last_tok()
                            .unwrap()
                            .end
                            .clone(),
                    ),
                );
                return res;
            }
            generic_args = generic_args_res.unwrap();

            consume!(GT, self, res);
        }

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
                generic_args,
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
            consume!(Comma, self, res);
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
            generic_args,
        }));
        res
    }
}
