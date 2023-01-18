use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct RawAsmNode {
    pub asm: Token,
}

impl Node for RawAsmNode {
    fn asm(
        &mut self,
        _ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        Ok(self.asm.clone().literal.unwrap())
    }

    fn pos(&self) -> Interval {
        self.asm.interval()
    }
}
