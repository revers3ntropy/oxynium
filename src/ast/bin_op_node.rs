use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;
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
}