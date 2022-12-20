use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, unknown_symbol, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::is_valid_identifier;
use crate::util::MutRc;

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: Token,
    pub value: MutRc<dyn Node>,
}

impl MutateVar {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let id = ctx.borrow_mut().get_dec_from_id(&self.id()).id;

        // get value before setting variable as initialised
        // so that self-references are invalid until AFTER the variable is initialised
        let value = self.value.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut()
            .set_dec_as_defined(&self.id(), self.pos())?;

        Ok(format!(
            "
           {value}
           pop rax
           mov {id}, rax
        "
        ))
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.id())
            || !ctx.borrow_mut().has_dec_with_id(&self.id())
        {
            return Err(unknown_symbol(self.id().clone()));
        }

        let (assign_type, _) =
            self.value.borrow_mut().type_check(ctx.clone())?;
        let symbol = ctx.borrow_mut().get_dec_from_id(&self.id()).clone();
        if symbol.is_type {
            return Err(type_error(format!(
                "'{}' is a type and cannot be assigned to",
                self.id()
            ))
            .set_interval(self.identifier.interval()));
        }
        if symbol.is_constant {
            return Err(type_error(format!(
                "Expected mutable variable, found constant '{}'",
                self.id()
            ))
            .set_interval((self.pos().0, self.value.borrow_mut().pos().0)));
        }
        if !symbol.type_.borrow().contains(assign_type.clone()) {
            return Err(mismatched_types(
                symbol.type_.clone(),
                assign_type.clone(),
            )
            .set_interval(self.value.borrow_mut().pos()));
        }
        Ok((get_type!(ctx, "Void"), None))
    }

    fn pos(&self) -> Interval {
        (
            self.identifier.start.clone(),
            self.value.borrow_mut().pos().1,
        )
    }
}
