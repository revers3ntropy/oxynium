use crate::ast::Node;
use crate::ast::types::built_in::{BOOL, INT};
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, type_error};
use crate::parse::token::{Token, TokenType};

#[derive(Debug)]
pub struct UnaryOpNode {
    pub operator: Token,
    pub rhs: Box<dyn Node>
}

impl Node for UnaryOpNode {
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
            TokenType::Not => {
                format!("
                    {}
                    pop rbx
                    mov rbx, [rbx]
                    mov rax, 0
                    cmp rbx, 0
                    setle al
                    push rax
                    push rsp
                ",
                    self.rhs.asm(ctx)?
                )
            }
            _ => panic!("Invalid arithmetic unary operator: {:?}", self.operator)
        })
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        let t = match self.operator.token_type {
            TokenType::Sub => Box::new(INT),
            _ => Box::new(BOOL),
        };

        let value_type = self.rhs.type_check(ctx)?;
        if !t.contains(value_type.as_ref()) {
            return Err(type_error(t.as_ref(), value_type.as_ref()))
        }

        Ok(t)
    }
}