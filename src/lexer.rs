use crate::token::{Token, TokenType};

pub(crate) struct Lexer {
    input: String
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
       Lexer {
           input
       }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.input.chars().nth(0) {
            match c {
                '+' => {
                    tokens.push(Token::new(TokenType::Plus, None));
                    let new_input= self.input[1..].to_owned();
                    self.input =  new_input;
                },
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
                    tokens.push(Token::new(TokenType::Int, Some(number)));
                },
                _ => {}
            }
        }

        tokens
    }
}