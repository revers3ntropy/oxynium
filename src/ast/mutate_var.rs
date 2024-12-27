use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::is_valid_identifier;
use crate::util::MutRc;

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: Token,
    pub value: MutRc<dyn AstNode>,
}

impl MutateVar {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl AstNode for MutateVar {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.value.borrow_mut().setup(ctx.clone())
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.id()) || !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            return Err(unknown_symbol(format!("Variable {}", self.id()))
                .set_interval(self.identifier.interval()));
        }

        let TypeCheckRes {
            t: assign_type,
            unknowns,
            ..
        } = self.value.borrow().type_check(ctx.clone())?;
        let symbol = ctx.borrow().get_dec_from_id(&self.id()).clone();
        if symbol.is_type {
            return Err(type_error(format!(
                "'{}' is a type and cannot be assigned to",
                self.id()
            ))
            .set_interval(self.identifier.interval()));
        }
        if symbol.is_constant {
            return Err(type_error(format!(
                "expected mutable variable, found constant '{}'",
                self.id()
            ))
            .set_interval((self.pos().0, self.value.borrow_mut().pos().0)));
        }
        if !symbol.type_.borrow().contains(assign_type.clone()) {
            return Err(mismatched_types(symbol.type_.clone(), assign_type.clone())
                .set_interval(self.value.borrow_mut().pos()));
        }
        Ok(TypeCheckRes::from_type_in_ctx(&ctx, "Void", unknowns, true))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
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

    fn pos(&self) -> Interval {
        (
            self.identifier.start.clone(),
            self.value.borrow_mut().pos().1,
        )
    }
}
