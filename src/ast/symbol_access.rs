use crate::ast::Node;
use crate::context::Context;
use crate::error::{Error, unknown_symbol};

#[derive(Debug)]
pub struct SymbolAccess {
    pub identifier: String
}

impl Node for SymbolAccess {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        if !ctx.has_id(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        Ok(format!("
            push {}
        ", self.identifier))
    }
}