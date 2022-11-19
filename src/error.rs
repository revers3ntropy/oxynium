use crate::position::{Position};

pub(crate) struct Error {
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
        format!(
            "{}: {} at {} to {}",
            self.name,
            self.message,
            self.start.str(),
            self.end.str()
        )
    }
}

pub(crate) fn syntax_error(message: String) -> Error {
    Error::new("SyntaxError".to_string(), message)
}

pub(crate) fn illegal_char_error(char: char) -> Error {
    syntax_error(format!("Illegal character: {}", char))
}