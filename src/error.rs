use crate::position::{Interval, Position};
use crate::types::Type;
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

    pub fn str_pretty(&self, source_code: String, file_name: String) -> String {
        let mut out = format!("{}:\n{}\n", self.name, self.message);
        out.push('\n');
        out.push('\n');

        let lines: Vec<&str> = source_code.split('\n').collect();

        out.push_str(&format!(
            "'{}' lines {}-{}:\n",
            file_name,
            self.start.line + 1,
            self.end.line + 1
        ));

        let mut line_idx = self.start.line as usize;
        let mut is_first_line = true;
        let mut is_last_line = false;
        for line in self.start.line..self.end.line + 1 {
            let line = lines[line as usize];
            if line_idx == self.end.line as usize {
                is_last_line = true;
            }

            let pre_line = format!("  {} | ", line_idx + 1);
            out.push_str(pre_line.as_str());
            out.push_str(line);
            out.push('\n');
            if is_first_line {
                for _ in 0..self.start.col + (pre_line.len() as i64) {
                    out.push(' ');
                }
                if self.end.line == line_idx as i64 {
                    for _ in self.start.col..self.end.col {
                        out.push('^');
                    }
                } else {
                    for _ in self.start.col..line.len() as i64 {
                        out.push('^');
                    }
                }

            } else if is_last_line {
                for _ in 0..pre_line.len() {
                    out.push(' ');
                }
                for _ in 0..self.end.col+1 {
                    out.push('^');
                }
            } else { // middle line
                for _ in 0..pre_line.len() {
                    out.push(' ');
                }
                for _ in 0..line.len() {
                    out.push('^');
                }
            }
            out.push('\n');
            line_idx += 1;
            is_first_line = false;
        }

        out
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
            "expected '{}', found '{}'",
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
