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
    Hash,         // #
}

#[derive(Clone)]
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
                format!(
                    "\"{}\"",
                    self.literal.as_ref().unwrap().clone()
                )
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
            TokenType::Hash => "#".to_string(),
        }
    }

    pub fn overload_op_id(&self) -> Option<&str> {
        return match self.token_type {
            TokenType::Plus => Some("add"),
            TokenType::Sub => Some("sub"),
            TokenType::Astrix => Some("mul"),
            TokenType::FSlash => Some("div"),
            TokenType::Percent => Some("mod"),
            TokenType::GTE => Some("gte"),
            TokenType::LTE => Some("lte"),
            TokenType::GT => Some("gt"),
            TokenType::LT => Some("lt"),
            TokenType::DblEquals => Some("eq"),
            TokenType::NotEquals => Some("neq"),
            TokenType::Or => Some("or"),
            TokenType::And => Some("and"),
            _ => None,
        };
    }
}

impl Debug for Token {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "Token<{:?}>{{ '{}' at {:?} to {:?} }}",
            self.token_type,
            self.str(),
            self.start,
            self.end
        )
    }
}
