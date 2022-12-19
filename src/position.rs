use std::fmt::{Debug, Formatter};

pub type Interval = (Position, Position);

pub struct Position {
    pub file: String,
    pub idx: i64,
    pub line: i64,
    pub col: i64,
}

impl Position {
    pub fn new(file: String, idx: i64, line: i64, col: i64) -> Position {
        Position {
            file,
            idx,
            line,
            col,
        }
    }

    pub fn unknown() -> Position {
        Position::new("".to_string(), -2, -2, -2)
    }

    pub fn unknown_interval() -> Interval {
        (Position::unknown(), Position::unknown())
    }

    pub fn advance(&mut self, current_char: Option<char>) -> Position {
        self.idx += 1;
        self.col += 1;

        if current_char.is_some() && current_char.unwrap() == '\n' {
            self.line += 1;
            self.col = 0;
        }

        self.clone()
    }

    pub fn reverse(&mut self, current_char: Option<char>) -> Position {
        self.idx -= 1;
        self.col -= 1;

        if current_char.is_some() && current_char.unwrap() == '\n' {
            self.line -= 1;
            self.col = 0;
        }

        self.clone()
    }

    pub fn str(&self) -> String {
        if self.idx == -2 {
            "<unknown>".to_string()
        } else {
            format!("'{}' {}:{}", self.file, self.line + 1, self.col + 1)
        }
    }

    pub fn is_unknown(&self) -> bool {
        self.idx == -2
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}(idx:{})",
            self.file, self.line, self.col, self.idx
        )
    }
}

impl Clone for Position {
    fn clone(&self) -> Position {
        Position {
            file: self.file.clone(),
            idx: self.idx,
            line: self.line,
            col: self.col,
        }
    }
}
