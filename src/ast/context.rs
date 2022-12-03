use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct ContextNode {
    pub ctx: Context
}

impl Node for ContextNode {
    fn asm(&mut self, _ctx: &mut Context) -> Result<String, Error> {
        Ok(String::new())
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}