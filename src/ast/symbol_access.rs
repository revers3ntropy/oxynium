use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{Error, type_error, unknown_symbol};
use crate::symbols::is_valid_identifier;
use crate::util::MutRc;

#[derive(Debug)]
pub struct SymbolAccess {
    pub identifier: String
}

impl Node for SymbolAccess {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let decl = ctx.borrow_mut().get_dec_from_id(&self.identifier).unwrap();
        if decl.require_init && !decl.is_defined {
            return Err(type_error(format!(
                "Cannot use uninitialized variable '{}'",
                self.identifier
            )));
        }

        Ok(format!("
            push {}
        ", ctx.borrow_mut().get_dec_from_id(&self.identifier)?.id))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("Symbol '{}' does not exist", self.identifier)));
        }
        if ctx.borrow_mut().get_dec_from_id(&self.identifier)?.is_type {
            return Err(type_error(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        Ok((ctx.borrow_mut().get_dec_from_id(&self.identifier)?.type_.clone(), None))
    }
}