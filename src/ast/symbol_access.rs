use crate::ast::Node;
use crate::context::Context;
use crate::error::{Error, unknown_symbol};

#[derive(Debug)]
pub struct SymbolAccess {
    identifier: String
}

impl SymbolAccess {
    pub fn new(identifier: String) -> SymbolAccess {
        SymbolAccess {
            identifier
        }
    }
}

impl Node for SymbolAccess {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        if !ctx.symbol_exists(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        Ok(format!("
            push {}
        ", self.identifier))
    }
}