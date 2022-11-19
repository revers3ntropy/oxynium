use crate::parse::token::{Token, TokenType};
use crate::position::Position;
use phf::phf_map;

const SINGLE_CHAR_TOKENS:  phf::Map<&'static str, TokenType> = phf_map! {
    "+" => TokenType::Plus,
    "-" => TokenType::Sub,
    "*" => TokenType::Astrix,
    "/" => TokenType::FSlash,
    "(" => TokenType::LParen,
    ")" => TokenType::RParen,
};

pub(crate) struct Lexer {
    input: String,
    position: Position,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
       Lexer {
           input,
           position: Position::new("".to_owned(), -1, 0, -1),
           current_char: None,
       }
    }

    fn advance (&mut self) -> char {
        if self.position.idx >= self.input.len() as i64 {
            return '\0';
        }
        if self.position.idx < 0 {
            panic!("Lexer idx less than 0");
        }
        self.position.advance(self.input.chars().nth(self.position.idx as usize));
        let current_char = self.input.chars().nth(self.position.idx as usize).unwrap();
        self.current_char = Some(current_char);
        current_char
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        if self.input.len() == 0 {
            return tokens;
        }

        while let Some(c) = self.current_char {
            match c {
                ' ' => self.advance(),
                '0'..='9' => {
                    // build a number while we can
                    let mut number = String::new();
                    while let Some(next) = self.input.chars().nth(0) {
                        if next.is_numeric() {
                            number.push(next);
                            let new_input= self.input[1..].to_owned();
                            self.input =  new_input;
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::new(
                        TokenType::Int,
                        Some(number),
                        self.position.clone(),
                        self.position.clone()
                    ));
                },
                _ => {
                    panic!("Unexpected character: {}", c);
                }
            }
        }

        tokens
    }
}