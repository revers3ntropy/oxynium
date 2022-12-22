use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ScopeNode {
    pub ctx: MutRc<Context>,
    pub body: MutRc<dyn Node>,
    pub position: Interval,
}

impl Node for ScopeNode {
    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        self.body.borrow_mut().asm(self.ctx.clone())
    }

    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        // shouldn't really be doing this but assuming that the incoming context is the same
        // on each pass...
        self.ctx.borrow_mut().set_parent(ctx.clone());

        self.body.borrow_mut().type_check(self.ctx.clone())
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
