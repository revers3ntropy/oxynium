use crate::ast::Node;
use crate::context::Context;

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
    fn asm(&mut self, _: &mut Context) -> String {
        format!("
            push {}
        ", self.identifier)
    }
}