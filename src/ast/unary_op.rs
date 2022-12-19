use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, Error};
use crate::get_type;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct UnaryOpNode {
    pub operator: Token,
    pub rhs: MutRc<dyn Node>,
}

impl Node for UnaryOpNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        Ok(match self.operator.token_type {
            TokenType::Sub => {
                format!(
                    "
                    {}
                    neg qword [rsp]
                ",
                    self.rhs.borrow_mut().asm(ctx.clone())?
                )
            }
            TokenType::Not => {
                format!(
                    "
                    {}
                    pop rbx
                    xor rax, rax
                    test rbx, rbx
                    setle al
                    push rax
                ",
                    self.rhs.borrow_mut().asm(ctx.clone())?
                )
            }
            _ => {
                panic!("Invalid arithmetic unary operator: {:?}", self.operator)
            }
        })
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let t = match self.operator.token_type {
            TokenType::Sub => get_type!(ctx, "Int"),
            _ => get_type!(ctx, "Bool"),
        };

        let (value_type, _) = self.rhs.borrow_mut().type_check(ctx.clone())?;
        if !t.borrow().contains(value_type.clone()) {
            return Err(mismatched_types(t.clone(), value_type.clone())
                .set_interval(self.rhs.borrow_mut().pos()));
        }

        Ok((t, None))
    }

    fn pos(&mut self) -> Interval {
        (self.operator.start.clone(), self.rhs.borrow_mut().pos().1)
    }
}
