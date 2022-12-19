use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::position::Interval;
use crate::symbols::{is_valid_identifier, SymbolDec, SymbolDef};
use crate::util::MutRc;

#[derive(Debug)]
pub struct EmptyGlobalConstNode {
    pub identifier: String,
    pub type_: MutRc<dyn Node>,
    pub is_external: bool,
    pub position: Interval,
}

impl Node for EmptyGlobalConstNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global constant '{}' inside function. Try using 'let' instead.",
                self.identifier
            )));
        }
        ctx.borrow_mut().define(SymbolDef {
            name: self.identifier.clone(),
            data: Some(format!("dq 0")),
            text: None,
        })?;
        Ok("".to_owned())
    }
    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone()
            )));
        }
        let (type_, _) = self.type_.borrow_mut().type_check(ctx.clone())?;

        let id = if !type_.borrow().is_ptr() {
            // deref if it shouldn't stay as a pointer
            format!("qword [{}]", self.identifier.clone())
        } else {
            self.identifier.clone()
        };

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id,
            is_constant: true,
            is_type: false,
            require_init: !self.is_external,
            is_defined: false,
            is_param: false,
            type_,
            position: self.pos()
        })?;
        Ok((
            ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(),
            None,
        ))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
