use crate::parse::token::{Token, TokenType};

fn should_insert_between(tok1: &Token, tok2: &Token) -> bool {
    match tok1.token_type {
        TokenType::Identifier
        | TokenType::Int
        | TokenType::CloseParen
        | TokenType::QM
        | TokenType::String => match tok2.token_type {
            TokenType::Identifier | TokenType::Int | TokenType::Hash | TokenType::String => true,
            _ => false,
        },
        _ => false,
    }
}

fn collapse_newlines(tokens: Vec<Token>) -> Vec<Token> {
    let mut new_tokens = Vec::new();
    let mut last_was_nl = false;
    for token in tokens {
        if token.token_type == TokenType::NL {
            if !last_was_nl {
                new_tokens.push(token);
            }
            last_was_nl = true;
        } else {
            new_tokens.push(token);
            last_was_nl = false;
        }
    }
    new_tokens
}

pub fn insert_semi_colons(mut tokens: Vec<Token>) -> Vec<Token> {
    if tokens.len() == 0 {
        return tokens;
    }

    tokens = collapse_newlines(tokens);

    let mut new_tokens = Vec::new();
    while tokens.len() > 0 {
        let first = tokens.remove(0);
        if first.token_type != TokenType::NL {
            new_tokens.push(first.clone());
        }

        if tokens.len() < 3 {
            continue;
        }

        if tokens[0].token_type == TokenType::NL && should_insert_between(&first, &tokens[1]) {
            new_tokens.push(Token::new(
                TokenType::EndStatement,
                None,
                first.end.clone(),
                first.end.clone(),
            ));
        }
    }

    new_tokens
}
