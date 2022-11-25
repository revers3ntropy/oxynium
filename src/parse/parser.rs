use crate::ast::arith_bin_op_node::ArithmeticBinOpNode;
use crate::ast::arith_unary_op_node::{ArithmeticUnaryOpNode, ArithUnaryOp};
use crate::ast::const_decl::ConstDecl;
use crate::ast::exec_root_node::ExecRootNode;
use crate::ast::fn_call_node::FnCallNode;
use crate::ast::for_loop::ForLoopNode;
use crate::ast::glob_var_decl::GlobVarDecl;
use crate::ast::int_node::IntNode;
use crate::ast::mutate_var::MutateVar;
use crate::ast::break_node::BreakNode;
use crate::ast::if_node::IfNode;
use crate::ast::Node;
use crate::ast::statements_node::StatementsNode;
use crate::ast::str_node::StrNode;
use crate::ast::symbol_access::SymbolAccess;
use crate::ast::term_bin_op_node::TermBinOpNode;
use crate::parse::parse_results::ParseResults;
use crate::parse::token::{Token, TokenType};
use crate::error::syntax_error;

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
        //println!("Parsing tokens: {:?}", self.tokens);

        let mut res = ParseResults::new();

        if self.tokens.len() == 0 {
            res.success(Box::new(ExecRootNode::new(None)));
            return res;
        }

        let expr = res.register(self.statements());
        if res.error.is_some() {
            return res;
        }
        let root_node = ExecRootNode::new(expr);

        if self.tok_idx < self.tokens.len() {
            panic!("Unexpected token: {:?}", self.tokens[self.tok_idx]);
        }

        res.success(Box::new(root_node));
        res
    }

    fn advance(&mut self, res: &mut ParseResults) -> Token {
        if self.tok_idx >= self.tokens.len() {
            panic!("Unexpected end of input at token {}", self.tok_idx);
        }

        res.register_advancement();

        self.tok_idx += 1;
        self.tokens[self.tok_idx-1].clone()
    }

    fn reverse (&mut self, amount: usize) -> Option<Token> {
        self.tok_idx -= amount;
        // Can't go backwards and not have a token there
        self.try_peak()
    }

    fn consume(&mut self, res: &mut ParseResults, tok_type: TokenType) {
        if let Some(tok) = self.try_peak() {
            if tok.token_type == tok_type {
                self.advance(res);
                return;
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

        while true {
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

        res.success(Box::new(StatementsNode::new(statements)));
        res
    }

    fn statement (&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        if self.peak_matches(TokenType::Identifier, Some("const".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.const_decl());
            return res;
        }
        if self.peak_matches(TokenType::Identifier, Some("var".to_string())) {
            self.advance(&mut res);
            res.node = res.register(self.var_decl());
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
            res.success(Box::new(BreakNode::new()));
            return res;
        }
        self.expression()
    }

    fn expression (&mut self) -> ParseResults {
        self.arithmetic_expr()
    }

    fn const_decl(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let name;

        if self.peak_matches(TokenType::Identifier, None) {
            name = Some(self.advance(&mut res).literal.unwrap());
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
            res.failure(syntax_error("Expected '='".to_string()),
                Some(self.tokens[self.tok_idx-1].start.clone()),
                Some(self.tokens[self.tok_idx].end.clone())
            );
            return res;
        }

        if let Some(tok) = self.try_peak() {
            if tok.token_type == TokenType::Int {
                self.advance(&mut res);
                let value = tok.literal.unwrap().parse::<i64>().unwrap();
                res.success(Box::new(ConstDecl::new(name.unwrap(), value)));
                return res;
            } else if tok.token_type == TokenType::String {
                self.advance(&mut res);
                res.success(Box::new(ConstDecl::new(name.unwrap(), tok.literal.unwrap())));
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

    fn var_decl(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let name;

        if self.peak_matches(TokenType::Identifier, None) {
            name = Some(self.advance(&mut res).literal.unwrap());
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
            res.failure(syntax_error("Expected '='".to_string()),
                        Some(self.tokens[self.tok_idx-1].start.clone()),
                        Some(self.tokens[self.tok_idx].end.clone())
            );
            return res;
        }

        if let Some(tok) = self.try_peak() {
            if tok.token_type == TokenType::Int {
                self.advance(&mut res);
                let value = tok.literal.unwrap().parse::<i64>().unwrap();
                res.success(Box::new(GlobVarDecl::new(name.unwrap(), value)));
                return res;
            }
            res.failure(syntax_error("Expected int".to_string()),
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
            if tok.token_type == TokenType::Plus {
                return self.compound(None);
            }
            if tok.token_type == TokenType::Sub {
                self.advance(&mut res);
                let exp = res.register(self.unary_expr());
                if res.error.is_some() {
                    return res;
                }
                res.success(Box::new(ArithmeticUnaryOpNode::new(
                    ArithUnaryOp::Minus,
                    exp.unwrap()
                )));
                return res;
            }
        }

        self.compound(None)
    }

    fn term(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let mut lhs = res.register(self.unary_expr());
        if res.error.is_some() {
            return res;
        }

        while let Some(op) = self.try_peak() {
            match op.token_type {
                TokenType::Astrix | TokenType::FSlash => {
                    self.advance(&mut res);

                    let rhs = res.register(self.unary_expr());
                    if res.error.is_some() {
                        return res;
                    }

                    lhs = Some(Box::new(TermBinOpNode::new(
                        lhs.unwrap(),
                        (if op.token_type == TokenType::Astrix { "imul"  } else { "idiv" }).to_owned(),
                        rhs.unwrap(),
                        "rax".to_owned()
                    )));
                }
                TokenType::Ampersand => {
                    self.advance(&mut res);

                    let rhs = res.register(self.unary_expr());
                    if res.error.is_some() {
                        return res;
                    }
                    lhs = Some(Box::new(TermBinOpNode::new(
                        lhs.unwrap(),
                        "idiv".to_owned(),
                        rhs.unwrap(),
                        "rdx".to_owned()
                    )));
                }
                _ => break,
            }
        }

        res.success(lhs.unwrap());
        res
    }

    fn arithmetic_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let mut lhs = res.register(self.term());
        if res.error.is_some() {
            return res;
        }

        while let Some(op) = self.try_peak() {
            if !(op.token_type == TokenType::Plus || op.token_type == TokenType::Sub) {
                break;
            }
            self.advance(&mut res);

            let rhs = res.register(self.term());
            if res.error.is_some() {
                return res;
            }

            lhs = Some(Box::new(ArithmeticBinOpNode::new(
                lhs.unwrap(),
                (if op.token_type == TokenType::Plus { "add"  } else { "sub" }).to_owned(),
                rhs.unwrap()
            )));
        }

        res.success(lhs.unwrap());
        res
    }

    fn compound(&mut self, base_option: Option<Box<dyn Node>>) -> ParseResults {
        let mut res = ParseResults::new();

        let base;
        if base_option.is_some() {
            base = base_option.unwrap();
        } else {
            let atom = res.register(self.atom());
            if res.error.is_some() {
                return res;
            }
            base = atom.unwrap();
        }

        res.success(base);
        res
    }

    fn function_call(&mut self, fn_identifier_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::LParen);
        if res.error.is_some() { return res; }

        if let Some(t) = self.try_peak() {
            // fn(), no arguments
            if t.token_type == TokenType::RParen {
                self.advance(&mut res);
                res.success(Box::new(FnCallNode::new(fn_identifier_tok.literal.unwrap(), Vec::new())));
                return res;
            }
        }

        let mut args = Vec::new();

        while true {
            let parameter = res.register(self.expression());
            if res.error.is_some() {
                return res;
            }

            args.push(parameter.unwrap());

            if let Some(t) = self.try_peak() {
                if t.token_type == TokenType::RParen {
                    break;
                }  else if t.token_type == TokenType::Comma {
                    self.advance(&mut res);
                } else {
                    res.failure(
                        syntax_error("Expected ',' or ')'".to_owned()),
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

        self.consume(&mut res, TokenType::RParen);
        if res.error.is_some() { return res; }

        res.success(Box::new(FnCallNode::new(fn_identifier_tok.literal.unwrap(), args)));
        res
    }

    fn assign(&mut self, id_tok: Token) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::Equals);

        let value = res.register(self.expression());
        if res.error.is_some() { return res; }

        res.success(Box::new(MutateVar::new(id_tok.literal.unwrap(), value.unwrap())));

        res
    }

    fn atom(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let tok = self.advance(&mut res);
        match tok.token_type {
            TokenType::Int => {
                let value = tok.literal.unwrap();
                res.success(Box::new(IntNode::new(value.parse::<i64>().unwrap())));
                res
            },
            TokenType::String => {
                let value = tok.literal.unwrap();
                res.success(Box::new(StrNode::new(value)));
                res
            },
            TokenType::LParen => {
                let expr = res.register(self.expression());
                if res.error.is_some() {
                    return res;
                }
                self.consume(&mut res, TokenType::RParen);
                if res.error.is_some() {
                    return res;
                }
                res.success(expr.unwrap());
                res
            },
            TokenType::Identifier => {
                if let Some(next) = self.try_peak() {
                    if next.token_type == TokenType::LParen {
                        return self.function_call(tok);
                    }
                    if next.token_type == TokenType::Equals {
                        return self.assign(tok);
                    }
                }

                res.success(Box::new(SymbolAccess::new(tok.literal.unwrap())));
                res
            },
            _ => {
                res.failure(
                    syntax_error("Expected number, identifier or '('".to_owned()),
                    Some(tok.start),
                    Some(tok.end),
                );
                res
            }
        }
    }

    fn for_loop(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        self.consume(&mut res, TokenType::LBrace);
        if res.error.is_some() { return res; }

        let statements = res.register(self.statements());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::RBrace);
        if res.error.is_some() { return res; }

        res.success(Box::new(ForLoopNode::new(statements.unwrap())));
        res
    }

    fn if_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        let comparison = res.register(self.expression());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::LBrace);
        if res.error.is_some() { return res; }

        let statements = res.register(self.statements());
        if res.error.is_some() { return res; }

        self.consume(&mut res, TokenType::RBrace);
        if res.error.is_some() { return res; }

        res.success(Box::new(IfNode::new(comparison.unwrap(), statements.unwrap())));
        res
    }
}