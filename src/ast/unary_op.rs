use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::{Error, type_error};
use crate::parse::token::{Token, TokenType};

#[derive(Debug)]
pub struct UnaryOpNode {
    pub operator: Token,
    pub rhs: Box<dyn Node>
}

impl Node for UnaryOpNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
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
                    self.rhs.asm(Rc::clone(&ctx))?
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
                    self.rhs.asm(Rc::clone(&ctx))?
                )
            }
            _ => panic!("Invalid arithmetic unary operator: {:?}", self.operator)
        })
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        let t = match self.operator.token_type {
            TokenType::Sub => ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(),
            _ => ctx.borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        };

        let value_type = self.rhs.type_check(Rc::clone(&ctx))?;
        if !t.contains(value_type.as_ref()) {
            return Err(type_error(t.as_ref(), value_type.as_ref()))
        }

        Ok(t)
    }
}