#[derive(Clone, Debug)]
pub(crate) enum TokenType {
    Int,
    Plus,
    Sub
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) literal: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, literal: Option<String>) -> Token {
        Token {
            token_type,
            literal,
        }
    }

    pub fn clone(&self) -> Token {
        Token {
            token_type: self.token_type.clone(),
            literal: self.literal.clone()
        }
    }
}