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
    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            return Err(unknown_symbol(self.id()));
        }
        if !ctx.borrow_mut().get_dec_from_id(&self.id())?.is_type {
            return Err(type_error(format!(
                "'{}' cannot be used as a type",
                self.id()
            )));
        }
        Ok((
            ctx.borrow_mut().get_dec_from_id(&self.id())?.type_.clone(),
            None,
        ))
    }

    fn pos(&mut self) -> Interval {
        (self.identifier.start.clone(), self.identifier.end.clone())
    }
}
