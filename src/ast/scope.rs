use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{Error, ErrorSource};
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ScopeNode {
    pub ctx: MutRc<Context>,
    pub body: MutRc<dyn AstNode>,
    pub position: Interval,
    pub err_source: Option<ErrorSource>,
}

impl ScopeNode {
    fn try_add_source_to_res<T: Clone>(
        &self,
        res: Result<T, Error>,
    ) -> Result<T, Error> {
        if let Some(mut err) = res.clone().err() {
            if let Some(source) = &self.err_source {
                err.try_set_source(source.clone());
            }
            return Err(err);
        }
        res
    }
}

impl AstNode for ScopeNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        self.ctx.borrow_mut().set_parent(ctx.clone());
        self.try_add_source_to_res(
            self.body.borrow_mut().setup(self.ctx.clone()),
        )
    }

    fn type_check(
        &self,
        _ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        self.try_add_source_to_res(
            self.body
                .borrow_mut()
                .type_check(self.ctx.clone()),
        )
    }

    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        self.try_add_source_to_res(
            self.body.borrow_mut().asm(self.ctx.clone()),
        )
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
