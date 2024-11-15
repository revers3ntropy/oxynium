use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct UncheckedTypeCastNode {
    pub new_type: MutRc<dyn AstNode>,
    pub value: MutRc<dyn AstNode>,
}

impl AstNode for UncheckedTypeCastNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.new_type.borrow_mut().setup(ctx.clone())?;
        self.value.borrow_mut().setup(ctx.clone())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        self.new_type.borrow().type_check(ctx)
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        self.value.borrow_mut().asm(ctx)
    }

    fn pos(&self) -> Interval {
        self.value.borrow().pos()
    }
}
