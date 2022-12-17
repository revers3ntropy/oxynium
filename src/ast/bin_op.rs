use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, Error};
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BinOpNode {
    pub lhs: MutRc<dyn Node>,
    pub operator: Token,
    pub rhs: MutRc<dyn Node>,
}

impl Node for BinOpNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        match self.operator.token_type {
            TokenType::Plus
            | TokenType::Sub
            | TokenType::And
            | TokenType::Or => Ok(format!(
                "
                    {}
                    {}
                    pop rax
                    pop rbx
                    {} rax, rbx
                    push rax
                ",
                self.rhs.borrow_mut().asm(ctx.clone())?,
                self.lhs.borrow_mut().asm(ctx.clone())?,
                match self.operator.token_type {
                    TokenType::Plus => "add",
                    TokenType::Sub => "sub",
                    TokenType::And => "and",
                    _ => "or",
                }
            )),
            TokenType::Astrix | TokenType::FSlash => Ok(format!(
                "
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        {} rbx
                        push rax
                    ",
                self.rhs.borrow_mut().asm(ctx.clone())?,
                self.lhs.borrow_mut().asm(ctx.clone())?,
                match self.operator.token_type {
                    TokenType::Astrix => "imul",
                    _ => "idiv",
                }
            )),
            TokenType::Percent => Ok(format!(
                "
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        idiv rbx
                        push rdx
                    ",
                self.rhs.borrow_mut().asm(ctx.clone())?,
                self.lhs.borrow_mut().asm(ctx.clone())?,
            )),
            TokenType::GT
            | TokenType::LT
            | TokenType::LTE
            | TokenType::GTE
            | TokenType::DblEquals
            | TokenType::NotEquals => Ok(format!(
                "
                        {}
                        {}
                        pop rcx ; lhs
                        pop rdx ; rhs
                        xor rax, rax     ; al is first byte of rax,
                        cmp rcx, rdx
                        {} al          ; so clear rax and put into al
                        push rax
                ",
                self.rhs.borrow_mut().asm(ctx.clone())?,
                self.lhs.borrow_mut().asm(ctx.clone())?,
                match self.operator.token_type {
                    TokenType::DblEquals => "sete",
                    TokenType::NotEquals => "setne",
                    TokenType::GT => "setg",
                    TokenType::LT => "setl",
                    TokenType::GTE => "setge",
                    _ => "setle",
                }
            )),
            _ => panic!("Invalid operator: {:?}", self.operator),
        }
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let operand_types = match self.operator.token_type {
            TokenType::Percent
            | TokenType::Plus
            | TokenType::Sub
            | TokenType::Astrix
            | TokenType::FSlash
            | TokenType::DblEquals
            | TokenType::NotEquals
            | TokenType::GT
            | TokenType::LT
            | TokenType::GTE
            | TokenType::LTE => {
                ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone()
            }
            _ => ctx.borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        };

        let (lhs_type, _) = self.lhs.borrow_mut().type_check(ctx.clone())?;
        if !operand_types.borrow().contains(lhs_type.clone()) {
            return Err(mismatched_types(
                operand_types.clone(),
                lhs_type.clone(),
            ));
        }
        let (rhs_type, _) = self.rhs.borrow_mut().type_check(ctx.clone())?;
        if !operand_types.borrow().contains(rhs_type.clone()) {
            return Err(mismatched_types(
                operand_types.clone(),
                rhs_type.clone(),
            ));
        }

        return Ok((
            match self.operator.token_type {
                TokenType::Percent
                | TokenType::Plus
                | TokenType::Sub
                | TokenType::Astrix
                | TokenType::FSlash => ctx
                    .clone()
                    .borrow_mut()
                    .get_dec_from_id("Int")?
                    .type_
                    .clone(),
                _ => ctx
                    .clone()
                    .borrow_mut()
                    .get_dec_from_id("Bool")?
                    .type_
                    .clone(),
            },
            None,
        ));
    }
    fn pos(&mut self) -> Interval {
        (self.lhs.borrow_mut().pos().0, self.rhs.borrow_mut().pos().1)
    }
}
