use crate::ast::Node;
use crate::ast::types::{Type};
use crate::context::Context;
use crate::error::{Error, type_error};
use crate::parse::token::{Token, TokenType};

#[derive(Debug)]
pub struct BinOpNode {
    pub lhs: Box<dyn Node>,
    pub operator: Token,
    pub rhs: Box<dyn Node>
}

impl Node for BinOpNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        match self.operator.token_type {
            TokenType::Plus | TokenType::Sub | TokenType::And | TokenType::Or => {
                Ok(format!("
                    {}
                    {}
                    pop rax
                    pop rbx
                    mov rbx, [rbx]
                    {} [rax], rbx
                    push rax
                ",
                   self.rhs.asm(ctx)?,
                   self.lhs.asm(ctx)?,
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
                        mov [rcx], rax
                        push rcx
                    ",
                      self.rhs.asm(ctx)?,
                      self.lhs.asm(ctx)?,
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
                        mov [rcx], rdx
                        push rcx
                    ",
                       self.rhs.asm(ctx)?,
                       self.lhs.asm(ctx)?,
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
                        push rsp
                ",
                       self.rhs.asm(ctx)?,
                       self.lhs.asm(ctx)?,
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

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {

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
                => ctx.get_from_id("Int").type_.clone(),
            _ => ctx.get_from_id("Bool").type_.clone(),
        };

        let lhs_type = self.lhs.type_check(ctx)?;
        if !operand_types.contains(lhs_type.as_ref()) {
            return Err(type_error(operand_types.as_ref(), lhs_type.as_ref()))
        }
        let rhs_type = self.rhs.type_check(ctx)?;
        if !operand_types.contains(rhs_type.as_ref()) {
            return Err(type_error(operand_types.as_ref(), rhs_type.as_ref()))
        }

        return Ok(match self.operator.token_type {
            TokenType::Percent
            | TokenType::Plus
            | TokenType::Sub
            | TokenType::Astrix
            | TokenType::FSlash
            => ctx.get_from_id("Int").type_.clone(),
            _ => ctx.get_from_id("Bool").type_.clone(),
        });
    }
}