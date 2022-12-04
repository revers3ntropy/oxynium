use crate::ast::Node;
use crate::ast::types::{Type};
use crate::context::{Ctx};
use crate::error::{Error, type_error_unstructured, unknown_symbol};

#[derive(Debug)]
pub struct TypeNode {
    pub identifier: String,
}

impl Node for TypeNode {
    fn asm(&mut self, _ctx: Ctx) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }
        if !ctx.borrow_mut().get_dec_from_id(&self.identifier)?.is_type {
            return Err(type_error_unstructured(format!(
                "'{}' cannot be used as a type", self.identifier
            )));
        }
        Ok(ctx.borrow_mut().get_dec_from_id(&self.identifier)?.type_.clone())
    }
}