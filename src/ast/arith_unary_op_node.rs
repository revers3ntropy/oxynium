use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::{Token, TokenType};

#[derive(Debug)]
pub struct ArithmeticUnaryOpNode {
    pub(crate) operator: Token,
    pub(crate) rhs: Box<dyn Node>
}

impl Node for ArithmeticUnaryOpNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        Ok(match self.operator.token_type {
            TokenType::Sub => {
                format!("
                    {}
                    pop rcx
                    mov rax, [rcx]
                    neg rax
                    mov [rcx], rax
                    push rcx
                ",
                    self.rhs.asm(ctx)?
                )
            }
            _ => panic!("Invalid arithmetic unary operator: {:?}", self.operator)
        })
    }
}