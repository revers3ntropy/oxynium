use crate::ast::{IntNode, ProgramNode};
use crate::token::Token;

pub(crate) struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens
        }
    }

    pub fn parse(&self) -> ProgramNode {
        ProgramNode::new(
            Box::new(IntNode::new(1))
        )
    }
}