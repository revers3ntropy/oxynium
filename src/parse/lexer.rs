use crate::parse::token::{Token, TokenType};
use crate::position::Position;
use phf::phf_map;

static IDENTIFIER_CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789_";

const SINGLE_CHAR_TOKENS:  phf::Map<&'static str, TokenType> = phf_map! {
    "+" => TokenType::Plus,
    "-" => TokenType::Sub,
    "*" => TokenType::Astrix,
    "/" => TokenType::FSlash,
    "(" => TokenType::LParen,
    ")" => TokenType::RParen,
    "%" => TokenType::Ampersand,
    "," => TokenType::Comma
};

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

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        if self.input.len() == 0 {
            return tokens;
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
            } else if SINGLE_CHAR_TOKENS.contains_key(&c.to_string()) {
                tokens.push(Token::new(
                    SINGLE_CHAR_TOKENS[&c.to_string()],
                    None,
                    self.position.clone(),
                    self.position.clone()
                ));
                self.advance();
            } else {
                panic!("Unexpected character: {}", c)
            }
        }

        tokens
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
}