use std::fmt::Display;
use crate::position::{Position};

#[derive(Debug, Clone)]
pub struct Error {
    pub name: String,
    pub message: String,
    pub start: Position,
    pub end: Position,
}

impl Error {
    pub fn new(name: String, message: String) -> Error {
        Error {
            name,
            message,
            start: Position::unknown(),
            end: Position::unknown(),
        }
    }

    pub fn set_pos(&mut self, start: Position, end: Position) -> &mut Error {
        self.start = start;
        self.end = end;
        self
    }

    pub fn str(&self) -> String {
        if self.start.str() == self.end.str() {
            format!(
                "{}: {} at {}",
                self.name,
                self.message,
                self.start.str()
            )
        } else {
            format!(
                "{}: {} at {} to {}",
                self.name,
                self.message,
                self.start.str(),
                self.end.str()
            )
        }
    }
}

pub fn syntax_error(message: String) -> Error {
    Error::new("SyntaxError".to_string(), message)
}

pub fn unknown_symbol(message: String) -> Error {
    Error::new("UnknownSymbol".to_string(), message)
}

pub fn type_error (expected: &dyn Display, got: &dyn Display) -> Error {
    Error::new(
        "TypeError".to_string(),
        format!("expected '{expected}', got '{got}'")
    )
}