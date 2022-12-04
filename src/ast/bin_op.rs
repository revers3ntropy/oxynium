use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
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
                    mov rbx, [rbx]
                    mov rax, [rax]
                    {} rax, rbx
                    push rax
                    mov rdi, 8
                    call malloc WRT ..plt
                    pop rdx
                    mov [rax], rdx
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
                        pop rcx
                        pop rbx
                        mov rbx, [rbx]
                        mov rax, [rcx]
                        cqo ; extend rax to rdx:rax
                        {} rbx
                        push rax
                        mov rdi, 8
                        call malloc WRT ..plt
                        pop rdx
                        mov [rax], rdx
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
                        pop rcx
                        pop rbx
                        mov rbx, [rbx]
                        mov rax, [rcx]
                        cqo ; extend rax to rdx:rax
                        idiv rbx
                        push rdx
                        mov rdi, 8
                        call malloc WRT ..plt
                        pop rdx
                        mov [rax], rdx
                        push rax
                    ",
                       self.rhs.asm(Rc::clone(&ctx))?,
                       self.lhs.asm(Rc::clone(&ctx))?,
                ))
            },
            TokenType::GT | TokenType::LT | TokenType::LTE | TokenType::GTE | TokenType::DblEquals | TokenType::NotEquals => {
                Ok(format!("
                        {}
                        {}
                        pop rcx ; *lhs
                        pop rbx ; *rhs
                        mov rbx, [rbx] ; rhs
                        mov rdx, [rcx] ; lhs
                        cmp rdx, rbx   ; lhs - rhs
                        mov rax, 0     ; al is first byte of rax,
                        {} al          ; so clear rax and put into al
                        push rax
                        mov rdi, 8
                        call malloc WRT ..plt
                        pop rdx
                        mov [rax], rdx
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

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {

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

        let lhs_type = self.lhs.type_check(Rc::clone(&ctx))?;
        if !operand_types.contains(lhs_type.as_ref()) {
            return Err(mismatched_types(operand_types.as_ref(), lhs_type.as_ref()))
        }
        let rhs_type = self.rhs.type_check(Rc::clone(&ctx))?;
        if !operand_types.contains(rhs_type.as_ref()) {
            return Err(mismatched_types(operand_types.as_ref(), rhs_type.as_ref()))
        }

        return Ok(match self.operator.token_type {
            TokenType::Percent
            | TokenType::Plus
            | TokenType::Sub
            | TokenType::Astrix
            | TokenType::FSlash
            => Rc::clone(&ctx).borrow_mut().get_dec_from_id("Int")?.type_.clone(),
            _ => Rc::clone(&ctx).borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
        });
    }
}