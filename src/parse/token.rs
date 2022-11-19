use crate::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TokenType {
    Int,
    Plus,
    Sub,
    Astrix,
    FSlash,
    LParen,
    RParen
}

#[derive(Debug)]
pub(crate) struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
    pub start: Position,
    pub end: Position,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<String>, start: Position, end: Position) -> Token {
        Token {
            token_type,
            literal,
            start,
            end,
        }
    }

    pub fn clone(&self) -> Token {
        Token {
            token_type: self.token_type.clone(),
            literal: self.literal.clone(),
            start: self.start.clone(),
            end: self.end.clone(),
        }
    }
}