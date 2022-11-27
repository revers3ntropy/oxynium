use crate::position::Position;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TokenType {
    Int,          // 123
    Plus,         // +
    Sub,          // -
    Astrix,       // *
    FSlash,       // /
    OpenParen,    // (
    CloseParen,   // )
    Ampersand,    // &
    Percent,      // %
    Identifier,   // foo
    Comma,        // ,
    EndStatement, // ;
    String,       // "foo"
    Equals,       // =
    DblEquals,    // ==
    OpenBrace,    // {
    CloseBrace,   // }
    Or,           // ||
    And,          // &&
    Not,          // !
    GT,           // >
    LT,           // <
    GTE,          // >=
    LTE,          // <=
    NotEquals,    // !=
    Colon,        // :
}

#[derive(Debug)]
pub struct Token {
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