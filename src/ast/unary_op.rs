use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx};
use crate::error::{Error, mismatched_types};
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
                    neg qword [rsp]
                ",
                    self.rhs.asm(Rc::clone(&ctx))?
                )
            }
            TokenType::Not => {
                format!("
                    {}
                    pop rbx
                    mov rax, 0
                    cmp rbx, 0
                    setle al
                    push rax
                ",
                    self.rhs.asm(Rc::clone(&ctx))?
                )
            }
            _ => panic!("Invalid arithmetic unary operator: {:?}", self.operator)
        })
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        let t = match self.operator.token_type {
            TokenType::Sub => ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(),
            _ => ctx.borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        };

        let (value_type, _) = self.rhs.type_check(Rc::clone(&ctx))?;
        if !t.contains(value_type.as_ref()) {
            return Err(mismatched_types(t.as_ref(), value_type.as_ref()))
        }

        Ok((t, None))
    }
}