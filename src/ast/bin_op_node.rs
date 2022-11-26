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
            TokenType::Plus | TokenType::Sub => {
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
                       _ => panic!("Invalid arithmetic binary operator: {:?}", self.operator)
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
                      if self.operator.token_type == TokenType::Astrix
                          { "imul"  } else { "idiv" }
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
            _ => panic!("Invalid operator: {:?}", self.operator)
        }
    }
}