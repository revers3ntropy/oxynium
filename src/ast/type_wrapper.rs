use crate::ast::Node;
use crate::ast::types::{Type};
use crate::context::Context;
use crate::error::{Error};

#[derive(Debug)]
pub struct TypeWrapperNode {
    pub identifier: String,
}

impl Node for TypeWrapperNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_dec_from_id(self.identifier.as_str()).type_.clone())
    }
}