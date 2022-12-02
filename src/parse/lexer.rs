use phf::phf_map;
use crate::parse::token::{Token, TokenType};
use crate::position::Position;
use crate::error::{Error, syntax_error};

static IDENTIFIER_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

const SINGLE_CHAR_TOKENS:  phf::Map<&'static str, TokenType> = phf_map! {
    "+" => TokenType::Plus,
    "-" => TokenType::Sub,
    "*" => TokenType::Astrix,
    "/" => TokenType::FSlash,
    "(" => TokenType::OpenParen,
    ")" => TokenType::CloseParen,
    "%" => TokenType::Percent,
    "&" => TokenType::Ampersand,
    "," => TokenType::Comma,
    ";" => TokenType::EndStatement,
    "=" => TokenType::Equals,
    "{" => TokenType::OpenBrace,
    "}" => TokenType::CloseBrace,
    "!" => TokenType::Not,
    ">" => TokenType::GT,
    "<" => TokenType::LT,
    ":" => TokenType::Colon,
};

const DOUBLE_CHAR_TOKENS:  phf::Map<&'static str, TokenType> = phf_map! {
    "||" => TokenType::Or,
    "&&" => TokenType::And,
    ">=" => TokenType::GTE,
    "<=" => TokenType::LTE,
    "!=" => TokenType::NotEquals,
    "=="=> TokenType::DblEquals,
};

pub fn token_type_str(token_type: &TokenType) -> String {
    match token_type {
        TokenType::Int => "<int>",
        TokenType::Plus => "+",
        TokenType::Sub => "-",
        TokenType::Astrix => "*",
        TokenType::FSlash => "/",
        TokenType::OpenParen => "(",
        TokenType::CloseParen => ")",
        TokenType::Ampersand => "&",
        TokenType::Percent => "%",
        TokenType::Identifier => "<identifier>",
        TokenType::Comma => ",",
        TokenType::EndStatement => ";",
        TokenType::String => "<string>",
        TokenType::Equals => "=",
        TokenType::DblEquals => "==",
        TokenType::OpenBrace => "{",
        TokenType::CloseBrace => "}",
        TokenType::Or => "||",
        TokenType::And => "&&",
        TokenType::Not => "!",
        TokenType::GT => ">",
        TokenType::LT => "<",
        TokenType::GTE => ">=",
        TokenType::LTE => "<=",
        TokenType::NotEquals => "!=",
        TokenType::Colon => ":",
    }.to_string()
}

pub struct Lexer {
    input: String,
    position: Position,
    current_char: Option<char>
}

impl Lexer {
    pub fn new(input: String, file_name: String) -> Lexer {
       Lexer {
           input,
           position: Position::new(file_name, -1, 0, -1),
           current_char: None
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
                // build a number while we can
                let mut number = String::new();
                while self.current_char.is_some() && self.current_char.unwrap().is_numeric() {
                    number.push(self.current_char.unwrap());
                    self.advance();
                }
                tokens.push(Token::new(
                    TokenType::Int,
                    Some(number),
                    self.position.clone(),
                    self.position.clone()
                ));

            } else if char::is_alphabetic(c) {
                tokens.push(self.make_identifier());

            } else if c.is_whitespace() {
                self.advance();

            } else if c == '"' {
                tokens.push(self.make_string());

            } else if c == '/' &&
                self.input.chars().nth((self.position.idx + 1) as usize) == Some('/')
            {
                self.advance();
                while self.current_char.is_some() && self.current_char.unwrap() != '\n' {
                    self.advance();
                }

            } else if DOUBLE_CHAR_TOKENS.contains_key(&(
                c.to_string() +
                    &self.input.chars()
                        .nth((self.position.idx + 1) as usize)
                        .unwrap_or('\0')
                        .to_string()
            )) {
                tokens.push(Token::new(
                    DOUBLE_CHAR_TOKENS[&(
                        c.to_string() +
                            &self.input.chars()
                                .nth((self.position.idx + 1) as usize)
                                .unwrap_or('\0')
                                .to_string())
                        ],
                    None,
                    self.position.clone(),
                    self.position.clone()
                ));
                self.advance();
                self.advance();

            } else if SINGLE_CHAR_TOKENS.contains_key(&c.to_string()) {
                tokens.push(Token::new(
                    SINGLE_CHAR_TOKENS[&c.to_string()],
                    None,
                    self.position.clone(),
                    self.position.clone()
                ));
                self.advance();

            } else {
                return Err(syntax_error(format!("Unexpected character '{}'", c)));
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

        let current_char = self.input.chars().nth(self.position.idx as usize);
        self.current_char = current_char;
        current_char
    }

    fn make_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        let start = self.position.clone();
        while self.current_char.is_some() &&
            IDENTIFIER_CHARS.contains(self.current_char.unwrap())
        {
            identifier.push(self.current_char.unwrap());
            self.advance();
        }
        Token::new(
            TokenType::Identifier,
            Some(identifier),
            start,
            self.position.clone()
        )
    }

    fn make_string(&mut self) -> Token {
        let mut string = String::new();
        let start = self.position.clone();
        self.advance();
        while self.current_char.is_some() && self.current_char.unwrap() != '"' {
            string.push(self.current_char.unwrap());
            self.advance();
        }
        self.advance();
        Token::new(
            TokenType::String,
            Some(string),
            start,
            self.position.clone()
        )
    }
}