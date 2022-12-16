use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::is_valid_identifier;
use crate::util::MutRc;

#[derive(Debug)]
pub struct SymbolAccess {
    pub identifier: Token,
}

impl SymbolAccess {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for SymbolAccess {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let decl = ctx.borrow_mut().get_dec_from_id(&self.id()).unwrap();
        if decl.require_init && !decl.is_defined {
            return Err(type_error(format!(
                "Cannot use uninitialized variable '{}'",
                self.id()
            )));
        }

        Ok(format!(
            "
            push {}
        ",
            ctx.borrow_mut().get_dec_from_id(&self.id())?.id
        ))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.id()) {
            return Err(unknown_symbol(self.id()));
        }
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            return Err(unknown_symbol(format!(
                "Symbol '{}' does not exist",
                self.id()
            )));
        }
        if ctx.borrow_mut().get_dec_from_id(&self.id())?.is_type {
            return Err(type_error(format!(
                "'{}' is a type and does not exist at runtime",
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
