use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::{Error, mismatched_types, type_error, unknown_symbol};
use crate::symbols::is_valid_identifier;

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: String,
    pub value: MutRc<dyn Node>
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let id = ctx.borrow_mut().get_dec_from_id(&self.identifier)?.id;

        // get value before setting variable as initialised
        // so that self-references are invalid until AFTER the variable is initialised
        let value = self.value.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut().set_dec_as_defined(&self.identifier)?;

        Ok(format!("
           {value}
           pop rax
           mov {id}, rax
        "))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier)
            || !ctx.borrow_mut().has_dec_with_id(&self.identifier)
        {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let (assign_type, _) = self.value.borrow_mut().type_check(ctx.clone())?;
        let symbol = ctx.borrow_mut().get_dec_from_id(&self.identifier)?.clone();
        if symbol.is_constant {
            return Err(type_error(format!(
                "Expected mutable variable, found constant '{}'",
                self.identifier
            )));
        }
        if symbol.is_type {
            return Err(type_error(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        if !symbol.type_.contains(assign_type.clone()) {
            return Err(mismatched_types(symbol.type_.clone(), assign_type.clone()));
        }
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None))
    }
}