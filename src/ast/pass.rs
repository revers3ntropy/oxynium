use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::Error;

#[derive(Debug)]
pub struct PassNode {}

impl Node for PassNode {
    fn asm(&mut self, mut _ctx: Ctx) -> Result<String, Error> {
        Ok("".to_string())
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}