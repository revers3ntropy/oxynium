use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::get_type;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BoolNode {
    pub value: bool,
    pub position: Interval,
}

impl Node for BoolNode {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        Ok(format!("\n push {} \n", if self.value { 1 } else { 0 }))
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok((get_type!(ctx, "Bool"), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
