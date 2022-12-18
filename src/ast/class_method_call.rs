use crate::ast::class_declaration::method_id;
use crate::ast::types::Type;
use crate::ast::{Node, TypeCheckRes};
use crate::ast::types::function::{FnParamType, FnType};
use crate::context::Context;
use crate::error::{type_error, Error, mismatched_types};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::util::{MutRc, new_mut_rc};

#[derive(Debug)]
pub struct ClassMethodCallNode {
    pub base: MutRc<dyn Node>,
    pub name: Token,
    pub args: Vec<MutRc<dyn Node>>,
    pub position: Interval,
    pub use_return_value: bool,
}

impl Node for ClassMethodCallNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        let (base_type_any, _) =
            self.base.borrow_mut().type_check(ctx.clone())?;
        let base_type = base_type_any.borrow_mut().as_class().unwrap();

        asm.push_str(&format!(
            "
            call {}
            {}
            {}
        ",
            method_id(
                base_type.name.clone(),
                self.name.clone().literal.unwrap()
            ),
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

        // put 'self' as first argument
        self.args.insert(0, self.base.clone());

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

        let (base_type_any, _) =
            self.base.borrow_mut().type_check(ctx.clone())?;
        let base_type = base_type_any.borrow().as_class();
        if base_type.is_none() {
            return Err(type_error(format!(
                "Cannot access method of non-class type '{}'",
                base_type_any.borrow_mut().str()
            ))
            .set_interval(self.position.clone()));
        }
        let base_type = base_type.unwrap();

        let method_type =
            base_type.method_type(&self.name.clone().literal.unwrap());
        if method_type.is_none() {
            return Err(type_error(format!(
                "Class '{}' does not have method '{}'",
                base_type.str(),
                self.name.clone().literal.unwrap(),
            ))
            .set_interval(self.position.clone()));
        }
        let method_type = method_type.unwrap().borrow().as_fn().unwrap();

        let call_signature_type = new_mut_rc(FnType {
            name: self.name.clone().literal.unwrap(),
            ret_type: method_type.ret_type.clone(),
            parameters: args,
        });

        if !method_type.contains(call_signature_type.clone()) {
            return Err(mismatched_types(
                new_mut_rc(method_type),
                call_signature_type.clone(),
            ));
        }

        // fill out default arguments
        for i in self.args.len()..method_type.parameters.len() {
            // add to end of vec
            self.args.insert(
                self.args.len(),
                method_type.parameters[i].default_value.clone().unwrap(),
            );
        }

        self.use_return_value = !method_type
            .ret_type
            .borrow()
            .contains(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone());

        Ok((method_type.ret_type.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
