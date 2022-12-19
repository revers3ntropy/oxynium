use crate::position::{Interval, Position};
use crate::types::Type;
use crate::util::{num_digits, MutRc};
use std::cmp::{max, min};
use std::io::Write;

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
        let mut out = format!("{}:\n  {}\n", self.name, self.message);
        out.push('\n');

        let lines: Vec<&str> = source_code.split('\n').collect();

        let start = self.start.clone();
        let mut end = self.end.clone();

        if end.is_unknown() {
            if start.is_unknown() {
                out.push_str("Unknown error location!");
                return out;
            }
            end = start.clone();
        }

        let max_digits_in_line_number = num_digits(end.line + 2);

        if start.idx == end.idx {
            out.push_str(&format!(
                "{}--> '{}' {}:{}\n",
                " ".repeat(max_digits_in_line_number + 2),
                file_name,
                start.line + 1,
                start.col + 1
            ));

            // to correctly show location of error with '^'s
            end.idx += 1;
            end.col += 1;
        } else {
            out.push_str(&format!(
                "{}--> '{}' {}:{} to {}:{}\n",
                " ".repeat(max_digits_in_line_number + 2),
                file_name,
                start.line + 1,
                start.col + 1,
                end.line + 1,
                end.col + 1
            ));
        }

        let mut line_idx = max(start.line - 1, 0);
        for line in line_idx..=min(end.line + 1, lines.len() as i64 - 1) {
            let line = lines[line as usize];

            let padding = " "
                .repeat(max_digits_in_line_number - num_digits(line_idx + 1));
            let pre_line = format!("  {}{padding} | ", line_idx + 1);

            out.push_str(pre_line.as_str());
            out.push_str(line);
            out.push('\n');
            if line_idx as i64 == start.line {
                // first line of error
                for _ in 0..(start.col + (pre_line.len() as i64)) {
                    out.push(' ');
                }
                if end.line == line_idx as i64 {
                    // single-line error
                    for _ in start.col..end.col {
                        out.push('^');
                    }
                } else {
                    for _ in start.col..line.len() as i64 {
                        out.push('^');
                    }
                }
                out.push('\n');
            } else if line_idx as i64 == end.line {
                // last line of error
                for _ in 0..pre_line.len() {
                    out.push(' ');
                }
                for _ in 0..=end.col {
                    out.push('^');
                }
                out.push('\n');
            } else if line_idx as i64 > start.line
                && (line_idx as i64) < end.line
            {
                // middle line
                for _ in 0..pre_line.len() {
                    out.push(' ');
                }
                for _ in 0..line.len() {
                    out.push('^');
                }
                out.push('\n');
            }
            line_idx += 1;
        }

        out
    }

    pub fn pretty_print_stderr(&self, source_code: String, file_name: String) {
        let _ = std::io::stderr().write(
            format!("{}\n", self.str_pretty(source_code, file_name)).as_bytes(),
        );
    }
    pub fn print_stderr(&self) {
        let _ = std::io::stderr().write(format!("{}\n", self.str()).as_bytes());
    }
}

pub fn syntax_error(message: String) -> Error {
    Error::new("SyntaxError", message)
}

pub fn invalid_symbol(message: String) -> Error {
    Error::new(
        "SyntaxError",
        format!("The symbol '{}' cannot be used here", message),
    )
}

pub fn numeric_overflow(message: String) -> Error {
    Error::new("NumericOverflow", message)
}

pub fn unknown_symbol(message: String) -> Error {
    Error::new("UnknownSymbol", message)
}

pub fn mismatched_types(
    expected: MutRc<dyn Type>,
    got: MutRc<dyn Type>,
) -> Error {
    Error::new(
        "TypeError",
        format!(
            "Expected '{}', found '{}'",
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

pub fn arg_error(message: &str) -> Error {
    Error::new("InvalidArguments", message.to_string())
}
