use crate::ast::types::Type;
use crate::position::{Interval, Position};
use crate::util::MutRc;

#[derive(Debug, Clone)]
pub struct Error {
    pub name: String,
    pub message: String,
    pub start: Position,
    pub end: Position,
}

impl Error {
    pub fn new(name: &str, message: String) -> Error {
        Error {
            name: name.to_string(),
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
    pub fn set_interval(mut self, pos: Interval) -> Error {
        self.start = pos.0;
        self.end = pos.1;
        self
    }

    pub fn str(&self) -> String {
        if self.start.str() == self.end.str() {
            format!("{}: {} at {}", self.name, self.message, self.start.str())
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
    Error::new("SyntaxError", message)
}

pub fn unknown_symbol(message: String) -> Error {
    Error::new("UnknownSymbolError", message)
}

pub fn invalid_symbol(message: String) -> Error {
    Error::new(
        "SyntaxError",
        format!("Symbol '{}' is not allowed", message),
    )
}

pub fn numeric_overflow(message: String) -> Error {
    Error::new("NumericOverflow", message)
}

pub fn mismatched_types(
    expected: MutRc<dyn Type>,
    got: MutRc<dyn Type>,
) -> Error {
    Error::new(
        "TypeError",
        format!(
            "expected '{}', got '{}'",
            expected.borrow().str(),
            got.borrow().str()
        ),
    )
}

pub fn type_error(message: String) -> Error {
    Error::new("TypeError", message)
}

pub fn io_error(message: String) -> Error {
    Error::new("IOError", message)
}
