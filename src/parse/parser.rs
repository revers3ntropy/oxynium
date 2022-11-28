use std::collections::HashMap;
use std::ops::Deref;
use crate::ast::bin_op::BinOpNode;
use crate::ast::unary_op::{UnaryOpNode};
use crate::ast::const_decl::{ConstDeclNode, EmptyConstDeclNode};
use crate::ast::exec_root::{EmptyExecRootNode, ExecRootNode};
use crate::ast::fn_call::FnCallNode;
use crate::ast::fn_declaration::FnDeclarationNode;
use crate::ast::for_loop::ForLoopNode;
use crate::ast::int::IntNode;
use crate::ast::mutate_var::MutateVar;
use crate::ast::r#break::BreakNode;
use crate::ast::r#if::IfNode;
use crate::ast::Node;
use crate::ast::statements::StatementsNode;
use crate::ast::str::StrNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::type_expr::TypeNode;
use crate::ast::type_wrapper::TypeWrapperNode;
use crate::parse::parse_results::ParseResults;
use crate::parse::token::{Token, TokenType};
use crate::error::{Error, syntax_error};
use crate::parse::lexer::token_type_str;
use crate::position::Position;

pub struct Parser {
    tokens: Vec<Token>,
    tok_idx: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            tok_idx: 0
        }
    }

    pub fn parse(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if self.tokens.len() == 0 {
            res.success(Box::new(EmptyExecRootNode { }));
            return res;
        }

        let expr = res.register(self.statements());
        if res.error.is_some() { return res; }
        let root_node = ExecRootNode { statements: expr.unwrap() };

        if self.tok_idx < self.tokens.len() {
            res.failure(
                syntax_error(format!("Unexpected token {:?}",
                                     token_type_str(&self.tokens[self.tok_idx].token_type))),
                Some(self.tokens[self.tok_idx].start.clone()),
                Some(self.tokens[self.tok_idx].end.clone())
            );
            return res;
        }

        res.success(Box::new(root_node));
        res
    }

    fn advance(&mut self, res: &mut ParseResults) {
        res.register_advancement();
        self.tok_idx += 1;
    }

    fn reverse (&mut self, amount: usize) -> Option<Token> {
        self.tok_idx -= amount;
        // Can't go backwards and not have a token there
        self.try_peak()
    }

    fn consume(&mut self, res: &mut ParseResults, tok_type: TokenType) -> Token {
        if let Some(tok) = self.try_peak() {
            if tok.token_type == tok_type {
                self.advance(res);
                return tok;
            }

            let err = syntax_error(format!(
                "Expected token type: {:?}, got: {:?}",
                tok_type,
                tok.token_type
            ));
            res.failure(err,
                Some(tok.start.clone()),
                Some(tok.end.clone())
            );
        }

        res.failure(syntax_error(format!(
            "Unexpected EOF, expected {:?}",
            tok_type
        )),
            Some(self.tokens[self.tok_idx-1].start.clone()),
            Some(self.tokens[self.tok_idx-1].end.clone())
        );
        Token::new(TokenType::EndStatement, None,
                   Position::unknown(), Position::unknown())
    }

    fn peak_matches(&self, tok_type: TokenType, value: Option<String>) -> bool {
        if let Some(tok) = self.try_peak() {
            if tok.token_type == tok_type {
                if let Some(value) = value {
                    if tok.literal.is_some() && tok.literal.unwrap() == value {
                        return true;
                    }
                } else {
                    return true;
                }
            }
        }
        false
    }

    fn try_peak(&self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.tok_idx].clone())
    }

    fn clear_end_statements (&mut self, res: &mut ParseResults) {
        while self.peak_matches(TokenType::EndStatement, None) {
            self.advance(res);
        }
    }

    fn bin_op <Fa, Fb>(&mut self, mut func_a: Fa, ops: Vec<TokenType>, mut func_b: Fb) -> ParseResults
        where Fa: FnMut(&mut Self) -> ParseResults
            , Fb: FnMut(&mut Self) -> ParseResults
    {
        let mut res = ParseResults::new();
        let mut left = res.register(func_a(self));
        if res.error.is_some() { return res; }

        while
            self.try_peak().is_some() &&
            ops.contains(&self.try_peak().unwrap().token_type)
        {
            let op_tok = self.try_peak();
            self.advance(&mut res);

            let right = res.register(func_b(self));

            if res.error.is_some() { return res; }
            left = Some(Box::new(BinOpNode {
                lhs: left.unwrap(),
                operator: op_tok.unwrap(),
                rhs: right.unwrap()
            }));
        }

        if res.error.is_some() { return res; }
        res.success(left.unwrap());
        res
    }

    fn statements(&mut self) -> ParseResults {

        let mut res = ParseResults::new();
        let mut statements: Vec<Box<dyn Node>> = Vec::new();
        self.clear_end_statements(&mut res);

        let first_stmt = res.register(self.statement());

        if res.error.is_some() {
            return res;
        }
        if first_stmt.is_none() {
            return res;
        }

        statements.push(first_stmt.unwrap());

        let mut more_statements = true;

        loop {
            let mut nl_count = 0;
            // @ts-ignore
            while self.peak_matches(TokenType::EndStatement, None) {
                self.advance(&mut res);
                nl_count += 1;
            }
            if nl_count == 0 {
                more_statements = false;
            }
            if !more_statements {
                break;
            }

            let statement = res.try_register(self.statement());
            if res.error.is_some() { return res; }
            if statement.is_none() {
                self.reverse(res.reverse_count);
                continue;
            }
            statements.push(statement.unwrap());
        }

        self.clear_end_statements(&mut res);

        res.success(Box::new(StatementsNode { statements }));
        res
    }

    fn statement (&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        if self.peak_matches(TokenType::Identifier, Some("const".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.var_decl(true));
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("var".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.var_decl(false));
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("for".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.for_loop());
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("if".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.if_expr());
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("break".to_string())) {
            self.advance(&mut res);
            res.success(Box::new(BreakNode { }));
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("fn".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.fn_expr());
            return res;
        }
        self.expression()
    }

    fn expression (&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        self.clear_end_statements(&mut res);
        if res.error.is_some() { return res; }
        self.bin_op(
            |this| this.comparison_expr(),
            vec![TokenType::And, TokenType::Or],
            |this| this.comparison_expr(),
        )
    }

    fn comparison_expr (&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let node = res.register(self.bin_op(
            |this| this.arithmetic_expr(),
            vec![
                TokenType::DblEquals, TokenType::NotEquals,
                TokenType::GT, TokenType::GTE,
                TokenType::LTE, TokenType::LT
            ],
            |this| this.arithmetic_expr(),
        ));

        if res.error.is_some() { return res; }
        res.success(node.unwrap());
        res
    }

    fn var_decl(&mut self, is_const: bool) -> ParseResults {
        let mut res = ParseResults::new();
        let name;

        if self.peak_matches(TokenType::Identifier, None) {
            name = Some(self.try_peak().unwrap().literal.unwrap());
            self.advance(&mut res);
        } else {
            res.failure(syntax_error("Expected identifier".to_string()),
                Some(self.tokens[self.tok_idx-1].start.clone()),
                Some(self.tokens[self.tok_idx-1].end.clone())
            );
            return res;
        }

        if self.peak_matches(TokenType::Equals, None) {
            self.advance(&mut res);
        } else {
            self.consume(&mut res, TokenType::Colon);
            if res.error.is_some() { return res; }

            let type_ = res.register(self.type_expr());
            if res.error.is_some() { return res; }

            res.success(Box::new(EmptyConstDeclNode {
                identifier: name.unwrap(),
                type_: type_.unwrap()
            }));
            return res;
        }

        if let Some(tok) = self.try_peak() {
            if tok.token_type == TokenType::Int {
                self.advance(&mut res);
                let value = tok.literal.unwrap().parse::<i64>().unwrap();
                res.success(Box::new(ConstDeclNode {
                    identifier: name.unwrap(),
                    value,
                    is_const
                }));
                return res;
            } else if tok.token_type == TokenType::String {
                self.advance(&mut res);
                res.success(Box::new(ConstDeclNode {
                    identifier: name.unwrap(),
                    value: tok.literal.unwrap(),
                    is_const
                }));
                return res;
            }
            res.failure(syntax_error("Expected int or str".to_string()),
                Some(self.tokens[self.tok_idx-1].start.clone()),
                Some(self.tokens[self.tok_idx-1].end.clone())
            );
            return res;
        }
        res.failure(syntax_error("Unexpected EOF".to_string()),
            Some(self.tokens[self.tok_idx-1].start.clone()),
            Some(self.tokens[self.tok_idx-1].end.clone())
        );
        return res;
    }

    fn unary_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if let Some(tok) = self.try_peak() {
            if tok.token_type == TokenType::Not {
                self.advance(&mut res);
                let node = res.register(self.unary_expr());
                if res.error.is_some() { return res; }
                res.success(Box::new(UnaryOpNode {
                    operator: tok,
                    rhs: node.unwrap()
                }));
                return res;
            }
        }

        self.compound(None)
    }

    fn factor(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let tok = self.try_peak();

        if let Some(t) = tok {
            if t.token_type == TokenType::Sub {
                self.advance(&mut res);
                let factor = res.register(self.factor());
                if res.error.is_some() { return res; }
                res.success(Box::new(UnaryOpNode {
                    operator: t,
                    rhs: factor.unwrap()
                }));
                return res;
            }
        }

        self.unary_expr()
    }

    fn term(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.factor(),
            vec![TokenType::Astrix, TokenType::FSlash, TokenType::Percent],
            |this| this.factor(),
        )
    }

    fn arithmetic_expr(&mut self) -> ParseResults {
        self.bin_op(
            |this| this.term(),
            vec![TokenType::Plus, TokenType::Sub],
            |this| this.term(),
        )
    }

    fn compound(&mut self, base_option: Option<Box<dyn Node>>) -> ParseResults {
        let mut res = ParseResults::new();

        let base;
        if base_option.is_some() {
            base = base_option.unwrap();
        } else {
            let atom = res.register(self.atom());
            if res.error.is_some() { return res; }
            base = atom.unwrap();
        }

        res.success(base);
        res
    }

    fn function_call(&mut self, fn_identifier_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::OpenParen);
        if res.error.is_some() { return res; }

        if let Some(t) = self.try_peak() {
            // fn(), no arguments
            if t.token_type == TokenType::CloseParen {
                self.advance(&mut res);
                res.success(Box::new(FnCallNode {
                    identifier: fn_identifier_tok.literal.unwrap(),
                    args: Vec::new()
                }));
                return res;
            }
        }

        let mut args = Vec::new();

        loop {
            let parameter = res.register(self.expression());
            if res.error.is_some() { return res; }

            args.push(parameter.unwrap());

            if let Some(t) = self.try_peak() {
                if t.token_type == TokenType::CloseParen {
                    break;
                }  else if t.token_type == TokenType::Comma {
                    self.advance(&mut res);
                } else {
                    res.failure(
                        syntax_error(format!("Expected ',' or ')', got '{}'",
                                             token_type_str(&t.token_type))),
                        Some(fn_identifier_tok.start),
                        Some(fn_identifier_tok.end)
                    );
                    return res;
                }

            } else {
                res.failure(
                    syntax_error("Expected ',' or ')', got EOF".to_owned()),
                    Some(fn_identifier_tok.start),
                    Some(fn_identifier_tok.end)
                );
                return res;
            }
        }

        self.consume(&mut res, TokenType::CloseParen);
        if res.error.is_some() { return res; }

        res.success(Box::new(FnCallNode {
            identifier: fn_identifier_tok.literal.unwrap(),
            args
        }));
        res
    }

    fn assign(&mut self, id_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::Equals);

        let value = res.register(self.expression());
        if res.error.is_some() { return res; }

        res.success(Box::new(MutateVar {
            identifier: id_tok.literal.unwrap(),
            value: value.unwrap()
        }));

        res
    }

    fn atom(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        if res.error.is_some() { return res; }

        let token_option = self.try_peak();
        if token_option.is_none() {
            res.failure(syntax_error("Unexpected EOF".to_string()),
                        Some(self.tokens[self.tok_idx-1].start.clone()),
                        Some(self.tokens[self.tok_idx-1].end.clone())
            );
            return res;
        }
        let tok = token_option.unwrap();
        match tok.token_type {
            TokenType::Int => {
                self.advance(&mut res);
                let value = tok.literal.unwrap();
                res.success(Box::new(IntNode {
                    value: value.parse::<i64>().unwrap()
                }));
            },
            TokenType::String => {
                self.advance(&mut res);
                let value = tok.literal.unwrap();
                res.success(Box::new(StrNode { value }));
            },
            TokenType::OpenParen => {
                self.advance(&mut res);

                let expr = res.register(self.expression());
                if res.error.is_some() { return res; }

                self.consume(&mut res, TokenType::CloseParen);
                if res.error.is_some() { return res; }

                res.success(expr.unwrap());
            },
            TokenType::Identifier => {
                self.advance(&mut res);
                if let Some(next) = self.try_peak() {
                    if next.token_type == TokenType::OpenParen {
                        return self.function_call(tok);
                    }
                    if next.token_type == TokenType::Equals {
                        return self.assign(tok);
                    }
                }

                res.success(Box::new(SymbolAccess {
                    identifier: tok.literal.unwrap()
                }));
            },
            _ => {
                res.failure(
                    syntax_error("Expected number, identifier or '('".to_owned()),
                    Some(tok.start),
                    Some(tok.end),
                );
            }
        };
        res
    }

    fn for_loop(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::OpenBrace);
        if res.error.is_some() { return res; }

        let statements = res.register(self.statements());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::CloseBrace);
        if res.error.is_some() { return res; }

        res.success(Box::new(ForLoopNode {
            statements: statements.unwrap()
        }));
        res
    }

    fn if_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let comparison = res.register(self.expression());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::OpenBrace);
        if res.error.is_some() { return res; }

        let statements = res.register(self.statements());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::CloseBrace);
        if res.error.is_some() { return res; }

        res.success(Box::new(IfNode {
            comparison: comparison.unwrap(),
            body: statements.unwrap()
        }));
        res
    }

    fn type_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let type_tok = self.consume(&mut res, TokenType::Identifier);
        if res.error.is_some() { return res; }

        res.success(Box::new(TypeNode {
            identifier: type_tok.literal.unwrap()
        }));
        res
    }

    fn parameter(&mut self) -> Result<(String, Box<dyn Node>), Error> {
        let mut res = ParseResults::new();

        let identifier = self.consume(&mut res, TokenType::Identifier);
        if res.error.is_some() { return Err(res.error.unwrap()); }

        self.consume(&mut res, TokenType::Colon);
        if res.error.is_some() { return Err(res.error.unwrap()); }

        let type_expr = res.register(self.type_expr());
        if res.error.is_some() { return Err(res.error.unwrap()); }

        Ok((identifier.literal.unwrap(), type_expr.unwrap()))
    }

    fn parameters(&mut self) -> Result<HashMap<String, Box<dyn Node>>, Error> {
        let mut res = ParseResults::new();

        let mut parameters = HashMap::new();

        if self.peak_matches(TokenType::CloseParen, None) {
            return Ok(parameters);
        }

        let (identifier, type_expr) = self.parameter()?;
        parameters.insert(identifier, type_expr);

        while let Some(next) = self.try_peak() {
            if next.token_type == TokenType::CloseParen {
                return Ok(parameters);
            }

            self.consume(&mut res, TokenType::Comma);
            if res.error.is_some() { return Err(res.error.unwrap()); }

            let (identifier, type_expr) = self.parameter()?;
            parameters.insert(identifier, type_expr);
        }

        let mut e = syntax_error("Expected ',' or ')', got EOF".to_owned());
        e.set_pos(self.tokens[self.tok_idx - 1].start.clone(),
            self.tokens[self.tok_idx - 1].end.clone());
        Err(e)
    }

    fn fn_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let id_tok = self.consume(&mut res,TokenType::Identifier);
        if res.error.is_some() { return res; }
        let identifier = id_tok.literal.unwrap();

        self.consume(&mut res, TokenType::OpenParen);
        if res.error.is_some() { return res; }

        let params = self.parameters();
        if params.is_err() {
            res.failure(params.err().unwrap(), None, None);
            return res;
        }

        self.consume(&mut res, TokenType::CloseParen);
        if res.error.is_some() { return res; }

        let mut ret_type: Box<dyn Node> = Box::new(TypeWrapperNode {
            identifier: "Void".to_string()
        });

        if self.peak_matches(TokenType::Colon, None) {
            self.advance(&mut res);
            let ret_type_option = res.register(self.type_expr());
            if res.error.is_some() { return res; }
            ret_type = ret_type_option.unwrap();
        }

        res.success(Box::new(FnDeclarationNode {
            identifier,
            ret_type,
            params: params.unwrap()
        }));
        res
    }
}