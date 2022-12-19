use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, unknown_symbol, Error};
use crate::get_type;
use crate::position::Interval;
use crate::symbols::is_valid_identifier;
use crate::types::function::{FnParamType, FnType};
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct FnCallNode {
    pub identifier: String,
    pub args: Vec<MutRc<dyn Node>>,
    pub use_return_value: bool,
    pub position: Interval,
}

impl Node for FnCallNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        asm.push_str(&format!(
            "
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
            if self.use_return_value {
                "push rax"
            } else {
                ""
            }
        ));

        Ok(asm)
    }

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if !is_valid_identifier(&self.identifier) {
            return Err(unknown_symbol(self.identifier.clone())
                .set_interval(self.position.clone()));
        }

        let mut args: Vec<FnParamType> = Vec::new();
        for arg in self.args.iter_mut() {
            let (arg_type, _) = arg.borrow_mut().type_check(ctx.clone())?;
            args.push(FnParamType {
                // calling the function, so parameter name is not known
                name: "".to_string(),
                type_: arg_type,
                default_value: None,
            });
        }

        if !ctx.borrow_mut().has_dec_with_id(&self.identifier) {
            return Err(unknown_symbol(format!(
                "undefined function {}",
                self.identifier
            ))
            .set_interval(self.position.clone()));
        }

        let fn_type = ctx
            .borrow_mut()
            .get_dec_from_id(&self.identifier)
            .type_
            .clone()
            .borrow()
            .as_fn();
        if fn_type.is_none() {
            return Err(unknown_symbol(format!(
                "'{}' is not a function",
                self.identifier
            ))
            .set_interval(self.position.clone()));
        }
        let fn_type = fn_type.unwrap();

        let call_signature_type = new_mut_rc(FnType {
            name: self.identifier.clone(),
            ret_type: fn_type.ret_type.clone(),
            parameters: args,
        });

        if !fn_type.contains(call_signature_type.clone()) {
            return Err(mismatched_types(
                new_mut_rc(fn_type),
                call_signature_type.clone(),
            )
            .set_interval(self.position.clone()));
        }

        // fill out default arguments
        for i in self.args.len()..fn_type.parameters.len() {
            // add to end of vec
            self.args.insert(
                self.args.len(),
                fn_type.parameters[i].default_value.clone().unwrap(),
            );
        }

        self.use_return_value =
            !fn_type.ret_type.borrow().contains(get_type!(ctx, "Void"));
        Ok((fn_type.ret_type.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
