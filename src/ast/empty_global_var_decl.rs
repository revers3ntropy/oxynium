use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::{Error, syntax_error};
use crate::symbols::{is_valid_identifier, SymbolDec};

#[derive(Debug)]
pub struct EmptyGlobalConstNode {
    pub identifier: String,
    pub type_: Box<dyn Node>,
    pub is_const: bool,
    pub is_external: bool
}

impl Node for EmptyGlobalConstNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if ctx.borrow_mut().stack_frame_peak().is_some() {
            return Err(syntax_error(format!(
                "Cannot declare global {} '{}' inside function. Try using 'let' instead.",
                if self.is_const { "constant" } else { "variable" },
                self.identifier
            )));
        }
        Ok("".to_owned())
    }
    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(syntax_error(format!(
                "Invalid global variable '{}'",
                self.identifier.clone()
            )));
        }
        let (type_, _) = self.type_.type_check(ctx.clone())?;

        let id = if !type_.is_ptr() {
            // deref if it shouldn't stay as a pointer
            format!("qword [{}]", self.identifier.clone())
        } else {
            self.identifier.clone()
        };

        ctx.borrow_mut().declare(SymbolDec {
            name: self.identifier.clone(),
            id,
            is_constant: self.is_const,
            is_type: false,
            require_init: !self.is_external,
            is_defined: false,
            type_
        })?;
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None))
    }
}