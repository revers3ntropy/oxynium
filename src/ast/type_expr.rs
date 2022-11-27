use crate::ast::Node;
use crate::ast::types::{Type};
use crate::context::Context;
use crate::error::{Error, unknown_symbol};

#[derive(Debug)]
pub struct TypeNode {
    pub identifier: String,
}

impl Node for TypeNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        if !ctx.has_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }
        Ok(ctx.get_from_id(&self.identifier).type_.clone())
    }
}