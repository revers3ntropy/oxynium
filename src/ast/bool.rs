use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BoolNode {
    pub value: bool,
    pub position: Interval,
}

impl AstNode for BoolNode {
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Bool", 0))
    }

    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        Ok(format!(
            "\n push {} \n",
            if self.value { 1 } else { 0 }
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
