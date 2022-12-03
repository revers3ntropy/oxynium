use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, type_error_unstructured, unknown_symbol};

#[derive(Debug)]
pub struct SymbolAccess {
    pub identifier: String
}

impl Node for SymbolAccess {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok(format!("
            push {}
        ", self.identifier))
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        if ctx.get_dec_from_id(&self.identifier).is_type {
            return Err(type_error_unstructured(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        Ok(ctx.get_dec_from_id(&self.identifier).type_.clone())
    }
}