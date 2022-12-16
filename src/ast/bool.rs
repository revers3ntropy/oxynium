use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct BoolNode {
    pub value: bool,
    pub position: Interval,
}

impl Node for BoolNode {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        Ok(format!(
            "
           push {}
        ",
            if self.value { 1 } else { 0 }
        ))
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok((
            ctx.borrow_mut().get_dec_from_id("Bool")?.type_.clone(),
            None,
        ))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
