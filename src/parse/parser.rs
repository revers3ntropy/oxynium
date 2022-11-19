use crate::ast::arith_bin_op_node::ArithmeticBinOpNode;
use crate::ast::exec_root_node::ExecRootNode;
use crate::ast::int_node::IntNode;
use crate::ast::node::Node;
use crate::parse::token::{Token, TokenType};

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

    pub fn parse(&mut self) -> ExecRootNode {
        println!("Parsing tokens: {:?}", self.tokens);

        if self.tokens.len() == 0 {
            return ExecRootNode::new(None);
        }

        let res = ExecRootNode::new(Some(self.expression()));

        if self.tok_idx < self.tokens.len() {
            panic!("Unexpected token: {:?}", self.tokens[self.tok_idx]);
        }

        res
    }

    fn advance(&mut self) -> Token {
        if self.tok_idx >= self.tokens.len() {
            panic!("Unexpected end of input at token {}", self.tok_idx);
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

    fn expression(&mut self) -> Box<dyn Node> {
        let lhs = self.atom();
        let operand = self.try_advance();
        if let Some(op) = operand {
            if op.token_type == TokenType::Plus {
                let rhs = self.expression();
                return Box::new(ArithmeticBinOpNode::new(lhs, "add".to_owned(), rhs));
            }
            if op.token_type == TokenType::Sub {
                let rhs = self.expression();
                return Box::new(ArithmeticBinOpNode::new(lhs, "sub".to_owned(), rhs));
            }
        }

        lhs
    }

    fn atom(&mut self) -> Box<dyn Node> {
        let tok = self.advance();
        return match tok.token_type {
            TokenType::Int => {
                let value = tok.literal.unwrap();
                Box::new(IntNode::new(value.parse::<i64>().unwrap()))
            }
            _ => panic!("Unexpected token: {:?}", tok)
        }
    }
}