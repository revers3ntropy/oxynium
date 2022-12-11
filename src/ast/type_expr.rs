use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{Error, type_error, unknown_symbol};
use crate::util::MutRc;

#[derive(Debug)]
pub struct TypeNode {
    pub identifier: String,
}

impl Node for TypeNode {
    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }
        if !ctx.borrow_mut().get_dec_from_id(&self.identifier)?.is_type {
            return Err(type_error(format!(
                "'{}' cannot be used as a type", self.identifier
            )));
        }
        Ok((ctx.borrow_mut().get_dec_from_id(&self.identifier)?.type_.clone(), None))
    }
}