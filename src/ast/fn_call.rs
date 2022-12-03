use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, type_error, unknown_symbol};

#[derive(Debug)]
pub struct FnCallNode {
    pub identifier: String,
    pub args: Vec<Box<dyn Node>>
}

impl Node for FnCallNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let mut asm = String::new();

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.asm(ctx)?);
            asm.push_str("\n");
        }

        asm.push_str(&format!("
            call {}
        ", self.identifier));

        Ok(asm)
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        let mut args = Vec::new();
        for arg in self.args.iter_mut() {
            args.push(arg.type_check(ctx)?);
        }

        if !ctx.has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("undefined function {}", self.identifier)));
        }

        let mut call_signature_children = vec![ctx.get_dec_from_id("Void").type_.clone()];
        call_signature_children.append(&mut args);
        let call_signature_type = Box::new(Type {
            id: ctx.get_type_id(),
            name: "Fn".to_string(),
            children: call_signature_children
        });

        let fn_type = ctx.get_dec_from_id(&self.identifier).type_.clone();
        if !fn_type.contains(&call_signature_type) {
            return Err(type_error(fn_type.as_ref(), call_signature_type.as_ref()));
        }
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}