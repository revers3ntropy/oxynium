use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ScopeNode {
    pub ctx: MutRc<Context>,
    pub body: MutRc<dyn AstNode>,
    pub position: Interval,
}

impl AstNode for ScopeNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        self.ctx.borrow_mut().set_parent(ctx.clone());
        self.body.borrow_mut().setup(self.ctx.clone())
    }

    fn type_check(
        &self,
        _ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        self.body.borrow_mut().type_check(self.ctx.clone())
    }

    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        self.body.borrow_mut().asm(self.ctx.clone())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
