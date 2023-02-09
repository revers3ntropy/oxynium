use crate::ast::str::StrNode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, Error};
use crate::get_type;
use crate::parse::token::{Token, TokenType};
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct UnaryOpNode {
    pub operator: Token,
    pub rhs: MutRc<dyn AstNode>,
}

impl AstNode for UnaryOpNode {
    fn setup(
        &mut self,
        ctx: MutRc<dyn Context>,
    ) -> Result<(), Error> {
        self.rhs.borrow_mut().setup(ctx)
    }

    fn type_check(
        &self,
        ctx: MutRc<dyn Context>,
    ) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: value_type,
            unknowns,
            ..
        } = self.rhs.borrow().type_check(ctx.clone())?;

        let t = match self.operator.token_type {
            TokenType::Sub => get_type!(ctx, "Int"),
            TokenType::Identifier => match self.operator.clone().literal.unwrap().as_str() {
                "typeof" => return Ok(TypeCheckRes::from_ctx(&ctx, "Str", unknowns, true)),
                _ => panic!(
                    "Invalid arithmetic unary operator: {:?}",
                    self.operator
                )
            }
            _ => get_type!(ctx, "Bool"),
        };

        if !t.borrow().contains(value_type.clone()) {
            return Err(mismatched_types(
                t.clone(),
                value_type.clone(),
            )
            .set_interval(self.rhs.borrow_mut().pos()));
        }

        Ok(TypeCheckRes::from(t, unknowns))
    }

    fn asm(
        &mut self,
        ctx: MutRc<dyn Context>,
    ) -> Result<String, Error> {
        Ok(match self.operator.token_type {
            TokenType::Sub => {
                format!(
                    "
                    {}
                    neg qword [rsp]
                ",
                    self.rhs
                        .borrow_mut()
                        .asm(ctx.clone())?
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
                    self.rhs
                        .borrow_mut()
                        .asm(ctx.clone())?
                )
            }
            TokenType::Identifier => {
                match self.operator.clone().literal.unwrap().as_str() {
                    "typeof" => {
                        let rhs_type = self.rhs
                            .borrow()
                            .type_check(ctx.clone())?;
                        let rhs_type = rhs_type.t;
                        let rhs_type =
                            if rhs_type.borrow().as_type_type().is_some() {
                                "Type".to_string()
                            } else {
                                rhs_type.borrow().str()
                            };
                        StrNode {
                            value: Token {
                                token_type: TokenType::String,
                                literal: Some(rhs_type),
                                start: self.rhs.borrow().pos().0,
                                end: self.rhs.borrow().pos().1,
                            },
                        }.asm(ctx.clone())?
                    }
                    _ => panic!(
                        "Invalid arithmetic unary operator: {:?}",
                        self.operator
                    )
                }
            }
            _ => {
                panic!(
                    "Invalid arithmetic unary operator: {:?}",
                    self.operator
                )
            }
        })
    }

    fn pos(&self) -> Interval {
        (
            self.operator.start.clone(),
            self.rhs.borrow_mut().pos().1,
        )
    }
}
