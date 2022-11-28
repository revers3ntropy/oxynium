use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, unknown_symbol};

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
        if !ctx.has_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        Ok(ctx.get_from_id(&self.identifier).type_.clone())
    }
}