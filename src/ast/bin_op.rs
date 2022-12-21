use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, syntax_error, Error};
use crate::get_type;
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
            _ => Err(syntax_error(format!(
                "Invalid operator: {}",
                self.operator.clone().literal.unwrap()
            ))
            .set_interval(self.pos())),
        }
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;
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
            | TokenType::LTE => get_type!(ctx, "Int"),
            _ => get_type!(ctx, "Bool"),
        };

        let lhs_tr = self.lhs.borrow_mut().type_check(ctx.clone())?;
        unknowns += lhs_tr.unknowns;
        if !operand_types.borrow().contains(lhs_tr.t.clone()) {
            return Err(mismatched_types(
                operand_types.clone(),
                lhs_tr.t.clone(),
            )
            .set_interval(self.lhs.borrow_mut().pos()));
        }
        let rhs_tr = self.rhs.borrow_mut().type_check(ctx.clone())?;
        if !operand_types.borrow().contains(rhs_tr.t.clone()) {
            return Err(mismatched_types(
                operand_types.clone(),
                rhs_tr.t.clone(),
            )
            .set_interval(self.rhs.borrow_mut().pos()));
        }

        return Ok(TypeCheckRes::from(
            match self.operator.token_type {
                TokenType::Percent
                | TokenType::Plus
                | TokenType::Sub
                | TokenType::Astrix
                | TokenType::FSlash => get_type!(ctx, "Int"),
                _ => get_type!(ctx, "Bool"),
            },
            unknowns,
        ));
    }
    fn pos(&self) -> Interval {
        (self.lhs.borrow_mut().pos().0, self.rhs.borrow_mut().pos().1)
    }
}
