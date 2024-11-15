use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct RawAsmNode {
    pub asm: Token,
    pub return_type: MutRc<dyn AstNode>,
}

impl AstNode for RawAsmNode {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        self.return_type.borrow().type_check(ctx)
    }

    fn asm(&mut self, _ctx: MutRc<dyn Context>) -> Result<String, Error> {
        Ok(self.asm.clone().literal.unwrap())
    }

    fn pos(&self) -> Interval {
        self.asm.interval()
    }
}
