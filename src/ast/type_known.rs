use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::types::Type;
use crate::util::MutRc;

#[derive(Debug)]
pub struct KnownTypeNode {
    pub t: MutRc<dyn Type>,
    pub pos: Interval,
}

impl Node for KnownTypeNode {
    fn type_check(
        &self,
        _ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from(self.t.clone(), 0))
    }

    fn pos(&self) -> Interval {
        self.pos.clone()
    }
}
