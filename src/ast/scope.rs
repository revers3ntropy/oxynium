use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::Error;

#[derive(Debug)]
pub struct ScopeNode {
    pub ctx: Ctx,
    pub body: Box<dyn Node>,
}

impl Node for ScopeNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        self.body.asm(ctx)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        self.ctx.borrow_mut().set_parent(Rc::clone(&ctx));
        self.body.type_check(Rc::clone(&self.ctx))?;
        Ok(Rc::clone(&self.ctx).borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}