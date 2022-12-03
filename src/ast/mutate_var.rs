use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, type_error, type_error_unstructured, unknown_symbol};

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: String,
    pub value: Box<dyn Node>
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        Ok(format!("
           {}
           pop rax
           mov rbx, {}
           mov rax, [rax]
           mov [rbx], rax
        ",
           self.value.asm(ctx)?,
           self.identifier
        ))
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {

        if !ctx.has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let assign_type = self.value.type_check(ctx)?;
        let symbol = ctx.get_dec_from_id(&self.identifier).clone();
        if symbol.is_constant {
            return Err(type_error(&"<var>", &"<const>"));
        }
        if symbol.is_type {
            return Err(type_error_unstructured(format!(
                "'{}' is a type and does not exist at runtime", self.identifier
            )));
        }
        if !symbol.type_.contains(assign_type.as_ref()) {
            return Err(type_error(symbol.type_.as_ref(), assign_type.as_ref()));
        }
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}