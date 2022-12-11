use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::ast::types::function::FnType;
use crate::ast::types::Type;
use crate::context::Ctx;
use crate::error::{Error, mismatched_types, unknown_symbol};
use crate::symbols::is_valid_identifier;

#[derive(Debug)]
pub struct FnCallNode {
    pub identifier: String,
    pub args: Vec<Box<dyn Node>>,
    pub use_return_value: bool
}

impl Node for FnCallNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.asm(ctx.clone())?);
            asm.push_str("\n");
        }

        asm.push_str(&format!("
            call {}
            {}
            {}
        ",
            self.identifier,
            if self.args.len() > 0 {
                format!("times {} pop rcx", self.args.len())
            } else {
                "".to_string()
            },
            if self.use_return_value { "push rax" } else { "" }
        ));

        Ok(asm)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone()));
        }

        let mut args = Vec::new();
        for arg in self.args.iter_mut() {
            let (arg_type, _) = arg.type_check(ctx.clone())?;
            args.push(arg_type);
        }

        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!("undefined function {}", self.identifier)));
        }

        let fn_type = ctx.borrow_mut()
            .get_dec_from_id(&self.identifier)?.type_.clone()
            .as_fn();
        if fn_type.is_none() {
            return Err(unknown_symbol(format!("'{}' is not a function", self.identifier)));
        }
        let fn_type = fn_type.unwrap();

        let call_signature_type = Rc::new(FnType {
            name: self.identifier.clone(),
            ret_type: fn_type.ret_type.clone(),
            parameters: args,
        });

        if !fn_type.contains(call_signature_type.clone()) {
            return Err(mismatched_types(Rc::new(fn_type), call_signature_type.clone()));
        }

        if fn_type.ret_type.contains(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone()) {
            self.use_return_value = false;
        } else {
            self.use_return_value = true;
        }
        Ok((fn_type.ret_type.clone(), None))
    }
}