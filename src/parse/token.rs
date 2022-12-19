use crate::position::{Interval, Position};
use std::fmt::Debug;

#[derive(Clone, PartialEq, Debug, Copy)]
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
    Dot,          // .
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

pub struct Token {
    pub token_type: TokenType,
    pub literal: Option<String>,
    pub start: Position,
    pub end: Position,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: Option<String>,
        start: Position,
        end: Position,
    ) -> Token {
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

    pub fn interval(&self) -> Interval {
        (self.start.clone(), self.end.clone())
    }

    pub fn str(&self) -> String {
        match self.token_type {
            TokenType::Plus => "+".to_string(),
            TokenType::Sub => "-".to_string(),
            TokenType::Astrix => "*".to_string(),
            TokenType::FSlash => "/".to_string(),
            TokenType::OpenParen => "(".to_string(),
            TokenType::CloseParen => ")".to_string(),
            TokenType::Ampersand => "&".to_string(),
            TokenType::Percent => "%".to_string(),
            TokenType::Identifier | TokenType::Int => {
                self.literal.as_ref().unwrap().clone()
            }
            TokenType::Comma => ",".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::EndStatement => ";".to_string(),
            TokenType::String => {
                format!("\"{}\"", self.literal.as_ref().unwrap().clone())
            }
            TokenType::Equals => "=".to_string(),
            TokenType::DblEquals => "==".to_string(),
            TokenType::OpenBrace => "{".to_string(),
            TokenType::CloseBrace => "}".to_string(),
            TokenType::Or => "||".to_string(),
            TokenType::And => "&&".to_string(),
            TokenType::Not => "!".to_string(),
            TokenType::GT => ">".to_string(),
            TokenType::LT => "<".to_string(),
            TokenType::GTE => ">=".to_string(),
            TokenType::LTE => "<=".to_string(),
            TokenType::NotEquals => "!=".to_string(),
            TokenType::Colon => ":".to_string(),
        }
    }
}

impl Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token<{:?}>{{ '{}' @ {:?} - {:?} }}",
            self.token_type,
            self.str(),
            self.start,
            self.end
        )
    }
}
