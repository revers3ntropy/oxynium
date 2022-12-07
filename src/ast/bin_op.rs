use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::{Error, mismatched_types};
use crate::parse::token::{Token, TokenType};

#[derive(Debug)]
pub struct BinOpNode {
    pub lhs: Box<dyn Node>,
    pub operator: Token,
    pub rhs: Box<dyn Node>
}

impl Node for BinOpNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        match self.operator.token_type {
            TokenType::Plus | TokenType::Sub | TokenType::And | TokenType::Or => {
                Ok(format!("
                    {}
                    {}
                    pop rax
                    pop rbx
                    {} rax, rbx
                    push rax
                ",
                   self.rhs.asm(Rc::clone(&ctx))?,
                   self.lhs.asm(Rc::clone(&ctx))?,
                   match self.operator.token_type {
                       TokenType::Plus => "add",
                       TokenType::Sub => "sub",
                       TokenType::And => "and",
                       _ => "or",
                   }
                ))
            },
            TokenType::Astrix | TokenType::FSlash => {
                Ok(format!("
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        {} rbx
                        push rax
                    ",
                      self.rhs.asm(Rc::clone(&ctx))?,
                      self.lhs.asm(Rc::clone(&ctx))?,
                      match self.operator.token_type {
                          TokenType::Astrix => "imul",
                          _ => "idiv",
                      }
                ))
            },
            TokenType::Percent => {
                Ok(format!("
                        {}
                        {}
                        pop rax
                        pop rbx
                        cqo ; extend rax to rdx:rax
                        idiv rbx
                        push rdx
                    ",
                       self.rhs.asm(Rc::clone(&ctx))?,
                       self.lhs.asm(Rc::clone(&ctx))?,
                ))
            },
            TokenType::GT
            | TokenType::LT
            | TokenType::LTE
            | TokenType::GTE
            | TokenType::DblEquals
            | TokenType::NotEquals => {
                Ok(format!("
                        {}
                        {}
                        pop rcx ; lhs
                        pop rdx ; rhs
                        mov rax, 0     ; al is first byte of rax,
                        cmp rcx, rdx
                        {} al          ; so clear rax and put into al
                        push rax
                ",
                       self.rhs.asm(Rc::clone(&ctx))?,
                       self.lhs.asm(Rc::clone(&ctx))?,
                       match self.operator.token_type {
                           TokenType::DblEquals => "sete",
                           TokenType::NotEquals => "setne",
                           TokenType::GT => "setg",
                           TokenType::LT => "setl",
                           TokenType::GTE => "setge",
                           _ => "setle",
                       }
                ))
            },
            _ => panic!("Invalid operator: {:?}", self.operator)
        }
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {

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
            | TokenType::LTE
                => ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(),
            _ => ctx.borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        };

        let (lhs_type, _) = self.lhs.type_check(Rc::clone(&ctx))?;
        if !operand_types.contains(lhs_type.as_ref()) {
            return Err(mismatched_types(operand_types.as_ref(), lhs_type.as_ref()))
        }
        let (rhs_type, _) = self.rhs.type_check(Rc::clone(&ctx))?;
        if !operand_types.contains(rhs_type.as_ref()) {
            return Err(mismatched_types(operand_types.as_ref(), rhs_type.as_ref()))
        }

        return Ok((match self.operator.token_type {
            TokenType::Percent
            | TokenType::Plus
            | TokenType::Sub
            | TokenType::Astrix
            | TokenType::FSlash
            => Rc::clone(&ctx).borrow_mut().get_dec_from_id("Int")?.type_.clone(),
            _ => Rc::clone(&ctx).borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        }, None));
    }
}