use crate::ast::{AstNode, TypeCheckRes};
use crate::context::scope::Scope;
use crate::context::Context;
use crate::error::{Error, ErrorSource};
use crate::position::Interval;
use crate::util::MutRc;
use std::fmt::Debug;

pub struct ScopeNode {
    pub ctx: Option<MutRc<dyn Context>>,
    pub body: MutRc<dyn AstNode>,
    pub position: Interval,
    pub err_source: Option<ErrorSource>,
}

impl ScopeNode {
    fn try_add_source_to_res<T: Clone>(&self, res: Result<T, Error>) -> Result<T, Error> {
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
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.ctx = Some(Scope::new_local(ctx.clone()));
        self.try_add_source_to_res(self.body.borrow_mut().setup(self.ctx.clone().unwrap()))
    }

    fn type_check(&self, _ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        self.try_add_source_to_res(self.body.borrow_mut().type_check(self.ctx.clone().unwrap()))
    }

    fn asm(&mut self, _ctx: MutRc<dyn Context>) -> Result<String, Error> {
        self.try_add_source_to_res(self.body.borrow_mut().asm(self.ctx.clone().unwrap()))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}

impl Debug for ScopeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(ScopeNode {:?})", self.position)
    }
}
