use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct TypeNode {
    pub identifier: Token,
}

impl TypeNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for TypeNode {
    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(self.id()).set_interval(self.pos()));
            }
            return Ok(TypeCheckRes::unknown());
        }
        if !ctx.borrow_mut().get_dec_from_id(&self.id()).is_type {
            return Err(type_error(format!(
                "'{}' cannot be used as a type",
                self.id()
            ))
            .set_interval(self.pos()));
        }
        Ok(TypeCheckRes::from_ctx(&ctx, &self.id(), 0))
    }

    fn pos(&self) -> Interval {
        self.identifier.interval()
    }
}
