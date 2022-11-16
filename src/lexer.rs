use crate::token::Token;

pub(crate) struct Lexer <'a> {
    input: &'a String,
    position: usize,
}

impl Lexer <'_> {
    pub fn new(input: &String) -> Lexer {
       Lexer {
           input,
           position: 0
       }
    }

    pub fn lex(&self) -> Vec<Token> {
        Vec::new()
    }
}