use crate::error::{syntax_error, Error};
use crate::parse::token::{Token, TokenType};
use crate::position::Position;
use phf::phf_map;

static IDENTIFIER_CHARS: &str =
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_$";

const SINGLE_CHAR_TOKENS: phf::Map<
    &'static str,
    TokenType,
> = phf_map! {
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
};

const DOUBLE_CHAR_TOKENS: phf::Map<
    &'static str,
    TokenType,
> = phf_map! {
    "||" => TokenType::Or,
    "&&" => TokenType::And,
    ">=" => TokenType::GTE,
    "<=" => TokenType::LTE,
    "!=" => TokenType::NotEquals,
    "=="=> TokenType::DblEquals,
};

pub struct Lexer {
    input: String,
    position: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String, file_name: String) -> Lexer {
        Lexer {
            input,
            position: Position::new(file_name, -1, 0, -1),
            current_char: None,
        }
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        if self.input.len() == 0 {
            return Ok(tokens);
        }

        self.advance();

        while let Some(c) = self.current_char {
            if char::is_numeric(c) {
                let start = self.position.clone();
                // build a number while we can
                let mut number = String::new();
                while self.current_char.is_some()
                    && self
                        .current_char
                        .unwrap()
                        .is_numeric()
                {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
                tokens.push(Token::new(
                    TokenType::Int,
                    Some(number),
                    start,
                    self.position.clone(),
                ));
            } else if IDENTIFIER_CHARS.contains(c) {
                tokens.push(self.make_identifier());
            } else if c.is_whitespace() {
                self.advance();
            } else if c == '"' {
                tokens.push(self.make_string()?);
            } else if c == '/'
                && self
                    .input
                    .chars()
                    .nth((self.position.idx + 1) as usize)
                    == Some('/')
            {
                self.advance();
                while self.current_char.is_some()
                    && self.current_char.unwrap() != '\n'
                {
                    self.advance();
                }
            } else if DOUBLE_CHAR_TOKENS.contains_key(
                &(c.to_string()
                    + &self
                        .input
                        .chars()
                        .nth(
                            (self.position.idx + 1)
                                as usize,
                        )
                        .unwrap_or('\0')
                        .to_string()),
            ) {
                let start = self.position.clone();
                tokens.push(Token::new(
                    DOUBLE_CHAR_TOKENS[&(c.to_string()
                        + &self
                            .input
                            .chars()
                            .nth(
                                (self.position.idx + 1)
                                    as usize,
                            )
                            .unwrap_or('\0')
                            .to_string())],
                    None,
                    start,
                    self.position.clone().advance(None),
                ));
                self.advance();
                self.advance();
            } else if SINGLE_CHAR_TOKENS
                .contains_key(&c.to_string())
            {
                let start = self.position.clone();
                tokens.push(Token::new(
                    SINGLE_CHAR_TOKENS[&c.to_string()],
                    None,
                    start,
                    self.position.clone(),
                ));
                self.advance();
            } else {
                return Err(syntax_error(format!(
                    "Unexpected character '{}'",
                    c
                ))
                .set_interval((
                    self.position.clone(),
                    self.position.clone(),
                )));
            }
        }

        Ok(tokens)
    }

    fn advance(&mut self) -> Option<char> {
        self.position.advance(self.current_char);

        if self.position.idx >= self.input.len() as i64 {
            self.current_char = None;
            return None;
        }

        let current_char = self
            .input
            .chars()
            .nth(self.position.idx as usize);
        self.current_char = current_char;
        current_char
    }

    fn make_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();

        while self.current_char.is_some()
            && IDENTIFIER_CHARS
                .contains(self.current_char.unwrap())
        {
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
        while self.current_char.is_some()
            && self.current_char.unwrap() != '"'
        {
            if self.current_char.unwrap() == '\\' {
                self.advance();
                match self.current_char.unwrap() {
                    'n' => string.push_str("\n"),
                    't' => string.push_str("\t"),
                    'r' => string.push_str("\r"),
                    '"' => string.push_str("\""),
                    '\\' => string.push_str("\\"),
                    _ => {
                        return Err(syntax_error(format!(
                            "Invalid escape character '\\{}'",
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
            return Err(syntax_error(
                "Unterminated string".to_string(),
            )
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
}
