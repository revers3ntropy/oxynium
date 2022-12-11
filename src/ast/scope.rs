use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::Error;

#[derive(Debug)]
pub struct ScopeNode {
    pub ctx: MutRc<Context>,
    pub body: MutRc<dyn Node>,
}

impl Node for ScopeNode {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        self.body.borrow_mut().asm(self.ctx.clone())
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        self.ctx.borrow_mut().set_parent(ctx.clone());
        self.body.borrow_mut().type_check(self.ctx.clone())
    }
}