use crate::args::Args;
use crate::error::{syntax_error, Error};
use crate::parse::auto_end_stmt::insert_semi_colons;
use crate::parse::token::{Token, TokenType};
use crate::perf;
use crate::position::Position;
use phf::phf_map;
use std::time::Instant;

static IDENTIFIER_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_$";

const SINGLE_CHAR_TOKENS: phf::Map<&'static str, TokenType> = phf_map! {
    "+" => TokenType::Plus,
    "-" => TokenType::Sub,
    "*" => TokenType::Astrix,
    "/" => TokenType::FSlash,
    "(" => TokenType::OpenParen,
    ")" => TokenType::CloseParen,
    "%" => TokenType::Percent,
    "&" => TokenType::Ampersand,
    "," => TokenType::Comma,
    "." => TokenType::Dot,
    ";" => TokenType::EndStatement,
    "=" => TokenType::Equals,
    "{" => TokenType::OpenBrace,
    "}" => TokenType::CloseBrace,
    "!" => TokenType::Not,
    ">" => TokenType::GT,
    "<" => TokenType::LT,
    ":" => TokenType::Colon,
    "#" => TokenType::Hash,
    "?" => TokenType::QM,
    // used as marker for auto-end-stmt insertion,
    // never included in `Lexer.lex()` output
    "\n" => TokenType::NL,
};

const DOUBLE_CHAR_TOKENS: phf::Map<&'static str, TokenType> = phf_map! {
    "||" => TokenType::Or,
    "&&" => TokenType::And,
    ">=" => TokenType::GTE,
    "<=" => TokenType::LTE,
    "!=" => TokenType::NotEquals,
    "=="=> TokenType::DblEquals,
    "??"=> TokenType::DblQM,
    "->"=> TokenType::Arrow,
};

pub struct Lexer {
    input: Vec<char>,
    position: Position,
    current_char: Option<char>,
    cli_args: Args,
}

impl Lexer {
    pub fn new(input: String, file_name: String, cli_args: Args) -> Lexer {
        Lexer {
            input: input.chars().into_iter().collect(),
            position: Position::new(file_name, -1, 0, -1),
            current_char: None,
            cli_args,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        if self.input.len() == 0 {
            return Ok(tokens);
        }

        self.advance();

        // for performance

        let mut double_char_keys: Vec<&str> =
            DOUBLE_CHAR_TOKENS.keys().into_iter().map(|c| *c).collect();
        // sort so that it is binary-search-able
        double_char_keys.sort();

        let mut single_char_keys: Vec<char> = SINGLE_CHAR_TOKENS
            .keys()
            .into_iter()
            .map(|c| c.chars().next().unwrap())
            .collect();
        single_char_keys.sort();

        let mut id_chars_sorted = IDENTIFIER_CHARS.chars().collect::<Vec<char>>();
        id_chars_sorted.sort();

        while let Some(c) = self.current_char {
            if c == '\n' {
                let pos = self.position.clone();
                while self.position.idx < self.input.len() as i64
                    && self.input[self.position.idx as usize] == '\n'
                {
                    self.advance();
                }
                tokens.push(Token::new(TokenType::NL, None, pos.clone(), pos));
                continue;
            }
            if c.is_whitespace() {
                self.advance();
                continue;
            }

            // must come before search for identifier char as
            // that includes digits
            if c.is_numeric() {
                let start = self.position.clone();
                let mut end = self.position.clone();

                // build a number while we can
                let mut number = String::new();
                while self.current_char.is_some() && self.current_char.unwrap().is_numeric() {
                    number.push(self.current_char.unwrap());
                    end = self.position.clone();
                    self.advance();
                }
                tokens.push(Token::new(TokenType::Int, Some(number), start, end));
                continue;
            }

            if id_chars_sorted.binary_search(&c).is_ok() {
                tokens.push(self.make_identifier());
                continue;
            }

            if c == '"' {
                tokens.push(self.make_string()?);
                continue;
            }

            if c == '\'' {
                tokens.push(self.make_char_literal()?);
                continue;
            }

            if c == '/'
                && self.position.idx < (self.input.len() - 2) as i64
                && self.input[(self.position.idx + 1) as usize] == '/'
            {
                self.advance();
                while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
                    self.advance();
                }
                continue;
            }

            if self.position.idx < (self.input.len() - 2) as i64 {
                let current_and_next =
                    c.to_string() + &self.input[(self.position.idx + 1) as usize].to_string();

                if double_char_keys
                    .binary_search(&current_and_next.as_str())
                    .is_ok()
                {
                    let start = self.position.clone();
                    tokens.push(Token::new(
                        DOUBLE_CHAR_TOKENS[current_and_next.as_str()],
                        None,
                        start,
                        self.position.clone().advance(None),
                    ));
                    self.advance();
                    self.advance();
                    continue;
                }
            }

            if single_char_keys.binary_search(&c).is_ok() {
                let pos = self.position.clone();
                tokens.push(Token::new(
                    SINGLE_CHAR_TOKENS[&c.to_string()],
                    None,
                    pos.clone(),
                    pos,
                ));
                self.advance();
                continue;
            }

            return Err(syntax_error(format!("unexpected character '{}'", c))
                .set_interval((self.position.clone(), Position::unknown())));
        }

        let start = Instant::now();
        tokens = insert_semi_colons(tokens);
        perf!(self.cli_args, start, "Insert End-Of-Statements");

        Ok(tokens)
    }

    fn advance(&mut self) -> Option<char> {
        self.position.advance(self.current_char);

        if self.position.idx >= self.input.len() as i64 {
            self.current_char = None;
            return None;
        }

        let current_char = Some(self.input[self.position.idx as usize]);
        self.current_char = current_char;

        current_char
    }

    fn make_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();

        while self.current_char.is_some() && IDENTIFIER_CHARS.contains(self.current_char.unwrap()) {
            identifier.push(self.current_char.unwrap());
            self.advance();
        }
        Token::new(
            TokenType::Identifier,
            Some(identifier),
            start,
            self.position.clone().reverse(None),
        )
    }

    fn make_string(&mut self) -> Result<Token, Error> {
        let mut string = String::new();
        let start = self.position.clone();
        self.advance();
        while self.current_char.is_some() && self.current_char.unwrap() != '"' {
            if self.current_char.unwrap() == '\\' {
                self.advance();
                match self.current_char.unwrap() {
                    'n' => string.push_str("\n"),
                    't' => string.push_str("\t"),
                    'r' => string.push_str("\r"),
                    '"' => string.push_str("\""),
                    '\\' => string.push_str("\\"),
                    '\n' => {
                        self.advance();
                        continue;
                    }
                    _ => {
                        return Err(syntax_error(format!(
                            "invalid escape character '\\{}'",
                            self.current_char.unwrap()
                        ))
                        .set_interval((
                            self.position.clone().reverse(None),
                            self.position.clone(),
                        )))
                    }
                }
                self.advance();
            } else {
                string.push(self.current_char.unwrap());
                self.advance();
            }
        }
        if self.current_char.is_none() {
            return Err(syntax_error("unterminated string".to_string())
                .set_interval((start, self.position.clone())));
        }
        self.advance();
        Ok(Token::new(
            TokenType::String,
            Some(string),
            start,
            self.position.clone().reverse(None),
        ))
    }

    fn make_char_literal(&mut self) -> Result<Token, Error> {
        let start = self.position.clone();
        self.advance();
        let char: String;
        if self.current_char.unwrap() == '\\' {
            self.advance();
            match self.current_char.unwrap() {
                'n' => char = "\n".to_string(),
                't' => char = "\t".to_string(),
                'r' => char = "\r".to_string(),
                '\'' => char = "'".to_string(),
                '\\' => char = "\\".to_string(),
                _ => {
                    return Err(syntax_error(format!(
                        "invalid escape character '\\{}'",
                        self.current_char.unwrap()
                    ))
                    .set_interval((self.position.clone().reverse(None), self.position.clone())))
                }
            };
        } else {
            char = self.current_char.unwrap().to_string();
        }
        self.advance();

        if self.current_char.unwrap() != '\'' {
            return Err(syntax_error("unterminated character literal".to_string())
                .set_interval((start, self.position.clone())));
        }
        self.advance();

        Ok(Token::new(
            TokenType::CharLiteral,
            Some(char),
            start.clone(),
            self.position.clone(),
        ))
    }
}
