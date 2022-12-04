use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::{Error, type_error, unknown_symbol};

#[derive(Debug)]
pub struct SymbolAccess {
    pub identifier: String
}

impl Node for SymbolAccess {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        Ok(format!("
            push {}
        ", ctx.borrow_mut().get_dec_from_id(&self.identifier)?.id))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        if ctx.borrow_mut().get_dec_from_id(&self.identifier)?.is_type {
            return Err(type_error(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        Ok(ctx.borrow_mut().get_dec_from_id(&self.identifier)?.type_.clone())
    }
}