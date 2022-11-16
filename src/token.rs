enum TokenType {
    Int,
}

pub(crate) struct Token {
    token_type: TokenType,
    literal: String
}