use crate::ast::arith_bin_op_node::ArithmeticBinOpNode;
use crate::ast::arith_unary_op_node::{ArithmeticUnaryOpNode, ArithUnaryOp};
use crate::ast::exec_root_node::ExecRootNode;
use crate::ast::int_node::IntNode;
use crate::ast::term_bin_op_node::TermBinOpNode;
use crate::parse::parse_results::ParseResults;
use crate::parse::token::{Token, TokenType};
use crate::error::syntax_error;

pub(crate) struct Parser {
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

        let expr = res.register(self.expression());
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

    fn advance(&mut self, res: Option<&mut ParseResults>) -> Token {
        if self.tok_idx >= self.tokens.len() {
            panic!("Unexpected end of input at token {}", self.tok_idx);
        }

        if let Some(res) = res {
            res.register_advancement();
        }

        self.tok_idx += 1;
        self.tokens[self.tok_idx-1].clone()
    }

    fn try_advance(&mut self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        self.tok_idx += 1;
        Some(self.tokens[self.tok_idx-1].clone())
    }

    fn consume(&mut self, res: &mut ParseResults, tok_type: TokenType) {
        if let Some(tok) = self.try_peak() {
            if tok.token_type == tok_type {
                self.advance(Some(res));
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

    fn try_peak(&self) -> Option<Token> {
        if self.tok_idx >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.tok_idx].clone())
    }

    fn expression (&mut self) -> ParseResults {
        self.arithmetic_expr()
    }

    fn unary_expr(&mut self) -> ParseResults {
        let mut res = ParseResults::new();

        if let Some(tok) = self.try_peak() {
            if tok.token_type == TokenType::Plus {
                return self.atom();
            }
            if tok.token_type == TokenType::Sub {
                self.advance(None);
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

        self.atom()
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
                    self.advance(Some(&mut res));

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
                    self.advance(Some(&mut res));

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
                _ => {
                    break;
                }
            }
        }

        ParseResults::from_node(lhs.unwrap())
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
            self.advance(Some(&mut res));

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

        println!("Arithmetic expression: {:?}", lhs);

        ParseResults::from_node(lhs.unwrap())
    }

    fn atom(&mut self) -> ParseResults {
        let mut res = ParseResults::new();
        let tok = self.advance(Some(&mut res));
        match tok.token_type {
            TokenType::Int => {
                let value = tok.literal.unwrap();
                res.success(Box::new(IntNode::new(value.parse::<i64>().unwrap())));
                res
            },
            TokenType::LParen => {
                let expr = res.register(self.expression());
                if res.error.is_some() {
                    return res;
                }
                println!("after expr {:?}", self.try_peak());
                self.consume(&mut res, TokenType::RParen);
                if res.error.is_some() {
                    return res;
                }
                res.success(expr.unwrap());
                res
            },
            _ => {
                res.failure(
                    syntax_error("Expected int or '('".to_owned()),
                    Some(tok.start),
                    Some(tok.end),
                );
                res
            }
        }
    }
}