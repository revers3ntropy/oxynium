use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64,
    pub position: Interval,
}

impl Node for IntNode {
    fn asm(&mut self, _ctx: MutRc<Context>) -> Result<String, Error> {
        Ok(format!(
            "
            mov rax, {}
            push rax
        ",
            self.value
        ))
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok((ctx.borrow_mut().get_dec_from_id("Int")?.type_.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
