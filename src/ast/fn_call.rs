use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{CallStackFrame, Ctx};
use crate::error::{Error, mismatched_types, unknown_symbol};

#[derive(Debug)]
pub struct FnCallNode {
    pub identifier: String,
    pub args: Vec<Box<dyn Node>>
}

impl Node for FnCallNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let mut asm = format!("
            ; reserve space for return value
            mov rax, 0
            push rax
        ");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.asm(Rc::clone(&ctx))?);
            asm.push_str("\n");
        }

        asm.push_str(&format!("
            call {}
        ", self.identifier));

        Ok(asm)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        let mut args = Vec::new();
        for arg in self.args.iter_mut() {
            let arg_type = arg.type_check(Rc::clone(&ctx))?;
            args.push(arg_type);
        }

        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("undefined function {}", self.identifier)));
        }

        let mut call_signature_children = vec![
            ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone()
        ];
        call_signature_children.append(&mut args);
        let call_signature_type = Box::new(Type {
            id: ctx.borrow_mut().get_type_id(),
            name: "Fn".to_string(),
            children: call_signature_children
        });

        let fn_type = ctx.borrow_mut().get_dec_from_id(&self.identifier)?.type_.clone();
        if !fn_type.contains(&call_signature_type) {
            return Err(mismatched_types(fn_type.as_ref(), call_signature_type.as_ref()));
        }
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}