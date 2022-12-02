use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct PassNode {}

impl Node for PassNode {
    fn asm(&mut self, _ctx: &mut Context) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_from_id("Void").type_.clone())
    }
}