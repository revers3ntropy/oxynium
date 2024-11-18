use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::r#type::TypeType;
use crate::util::{mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct SymbolAccessNode {
    pub identifier: Token,
}

impl SymbolAccessNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl AstNode for SymbolAccessNode {
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if !ctx.borrow_mut().has_dec_with_id(&self.id()) {
            if ctx.borrow().throw_on_unknowns() {
                return Err(
                    unknown_symbol(format!("symbol '{}' does not exist", self.id()))
                        .set_interval(self.pos()),
                );
            }
            return Ok(TypeCheckRes::unknown());
        }
        if ctx.borrow_mut().get_dec_from_id(&self.id()).is_type {
            return Ok(TypeCheckRes::from(
                mut_rc(TypeType {
                    instance_type: ctx.borrow_mut().get_dec_from_id(&self.id()).type_.clone(),
                }),
                0,
            ));
        }

        Ok(TypeCheckRes::from_ctx(&ctx, &self.id(), 0, false))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let decl = ctx.borrow_mut().get_dec_from_id(&self.id());
        if decl.require_init && !decl.is_defined {
            return Err(
                type_error(format!("cannot use uninitialized variable '{}'", self.id()))
                    .set_interval(self.pos()),
            );
        }

        if decl.is_type {
            return Ok("".to_string());
        }

        let dec = ctx.borrow_mut().get_dec_from_id(&self.id());

        Ok(
            // TODO fix this mess
            if decl.type_.borrow().is_ptr() && !dec.id.contains("qword [") {
                format!(
                    "lea rax, [rel {}]
                    push rax\n",
                    dec.id
                )
            } else {
                format!("push {}\n", dec.id)
            },
        )
    }

    fn pos(&self) -> Interval {
        self.identifier.interval()
    }
}
