use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
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

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        self.ctx.borrow_mut().set_parent(Rc::clone(&ctx));
        self.body.type_check(Rc::clone(&self.ctx))
    }
}