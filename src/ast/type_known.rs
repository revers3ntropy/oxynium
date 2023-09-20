use crate::ast::{AstNode, TypeCheckRes};
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

impl AstNode for KnownTypeNode {
    fn type_check(&self, _ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        Ok(TypeCheckRes::from(self.t.clone(), 0))
    }

    fn pos(&self) -> Interval {
        self.pos.clone()
    }
}
