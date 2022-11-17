use crate::ast::{BinOpNode, IntNode, Node, ProgramNode};
use crate::token::{Token, TokenType};

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

    pub fn parse(&mut self) -> ProgramNode {
        println!("Parsing tokens: {:?}", self.tokens);
        ProgramNode::new(self.expression())
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
        println!("Expression on token #{}", self.tok_idx);
        let lhs = self.atom();
        let operand = self.try_advance();
        if let Some(Token { token_type: TokenType::Plus, .. }) = operand {
            let rhs = self.expression();
            Box::new(BinOpNode::new(lhs, "add".to_owned(), rhs))
        } else {
            lhs
        }
    }

    fn atom(&mut self) -> Box<dyn Node> {
        let tok = self.advance();
        return match tok.token_type {
            TokenType::Int => {
                let value = tok.literal.unwrap();
                Box::new(IntNode::new(value.parse::<i32>().unwrap()))
            }
            _ => panic!("Unexpected token: {:?}", tok)
        }
    }
}