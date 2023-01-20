use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{
    syntax_error, type_error, unknown_symbol, Error,
};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::is_valid_identifier;
use crate::types::r#type::TypeType;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct SymbolAccess {
    pub identifier: Token,
}

impl SymbolAccess {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl AstNode for SymbolAccess {
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.id()) {
            return Err(syntax_error(format!(
                "Invalid identifier '{}'",
                self.id()
            )));
        }
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "Symbol '{}' does not exist",
                    self.id()
                ))
                .set_interval(self.pos()));
            }
            return Ok(TypeCheckRes::unknown());
        }
        if ctx
            .borrow_mut()
            .get_dec_from_id(&self.id())
            .is_type
        {
            return Ok(TypeCheckRes::from(
                new_mut_rc(TypeType {
                    instance_type: ctx
                        .borrow_mut()
                        .get_dec_from_id(&self.id())
                        .type_
                        .clone(),
                }),
                0,
            ));
        }

        Ok(TypeCheckRes::from_ctx(&ctx, &self.id(), 0))
    }

    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let decl =
            ctx.borrow_mut().get_dec_from_id(&self.id());
        if decl.require_init && !decl.is_defined {
            return Err(type_error(format!(
                "Cannot use uninitialized variable '{}'",
                self.id()
            ))
            .set_interval(self.pos()));
        }

        if decl.is_type {
            return Ok("".to_string());
        }

        Ok(format!(
            "
            push {}
        ",
            ctx.borrow_mut().get_dec_from_id(&self.id()).id
        ))
    }

    fn pos(&self) -> Interval {
        self.identifier.interval()
    }

    fn as_symbol_access(&self) -> Option<SymbolAccess> {
        Some(self.clone())
    }
}
