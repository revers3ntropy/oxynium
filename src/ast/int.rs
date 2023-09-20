use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64,
    pub position: Interval,
}

impl AstNode for IntNode {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from_ctx(&ctx, "Int", 0, true))
    }

    fn asm(&mut self, _ctx: MutRc<dyn Context>) -> Result<String, Error> {
        Ok(format!(
            "
            mov rax, {}
            push rax
        ",
            self.value
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
