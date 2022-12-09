use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx};
use crate::error::{Error, mismatched_types, type_error, unknown_symbol};

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: String,
    pub value: Box<dyn Node>
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let id = ctx.borrow_mut().get_dec_from_id(&self.identifier)?.id;

        // get value before setting variable as initialised
        // so that self-references are invalid until AFTER the variable is initialised
        let value = self.value.asm(ctx.clone())?;

        ctx.borrow_mut().set_dec_as_defined(&self.identifier)?;

        Ok(format!("
           {value}
           pop rax
           mov {id}, rax
        "))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let (assign_type, _) = self.value.type_check(ctx.clone())?;
        let symbol = ctx.borrow_mut().get_dec_from_id(&self.identifier)?.clone();
        if symbol.is_constant {
            return Err(mismatched_types(&"<var>", &"<const>"));
        }
        if symbol.is_type {
            return Err(type_error(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        if !symbol.type_.contains(assign_type.clone()) {
            return Err(mismatched_types(symbol.type_.as_ref(), assign_type.as_ref()));
        }
        Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), None))
    }
}