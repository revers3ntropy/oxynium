use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::{
    is_valid_identifier, SymbolDec, SymbolDef,
};
use crate::util::MutRc;

#[derive(Debug)]
pub struct EmptyGlobalConstNode {
    pub identifier: Token,
    pub type_: MutRc<dyn AstNode>,
    pub is_external: bool,
    pub is_exported: bool,
    pub position: Interval,
}

impl AstNode for EmptyGlobalConstNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        if !is_valid_identifier(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        self.type_.borrow_mut().setup(ctx.clone())
    }
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let TypeCheckRes {
            t: type_, unknowns, ..
        } = self
            .type_
            .borrow_mut()
            .type_check(ctx.clone())?;

        let id = if !type_.borrow().is_ptr() {
            // deref if it shouldn't stay as a pointer
            format!(
                "qword [{}]",
                self.identifier.clone().literal.unwrap()
            )
        } else {
            self.identifier.clone().literal.unwrap()
        };

        ctx.borrow_mut().declare(
            SymbolDec {
                name: self
                    .identifier
                    .clone()
                    .literal
                    .unwrap(),
                id,
                is_constant: true,
                is_type: false,
                require_init: !self.is_external,
                is_defined: false,
                is_param: false,
                type_,
                position: self.pos(),
            },
            self.pos(),
        )?;

        Ok(TypeCheckRes::from_ctx(&ctx, "Void", unknowns))
    }
    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier.clone().literal.unwrap()
            )).set_interval((self.pos().0, self.identifier.end.clone())));
        }
        ctx.borrow_mut().define(
            SymbolDef {
                name: self
                    .identifier
                    .clone()
                    .literal
                    .unwrap(),
                data: Some(format!("dq 0")),
                text: None,
            },
            self.pos(),
        )?;
        Ok("".to_owned())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
