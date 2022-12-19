use crate::ast::class_declaration::method_id;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, type_error, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::function::{FnParamType, FnType};
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct ClassMethodCallNode {
    pub base: MutRc<dyn Node>,
    pub name: Token,
    pub args: Vec<MutRc<dyn Node>>,
    pub position: Interval,
    pub use_return_value: bool,
}

impl ClassMethodCallNode {
    fn class_id(&self, ctx: MutRc<Context>) -> String {
        let t = self.base.borrow_mut().type_check(ctx.clone()).unwrap().0;
        if let Some(class) = t.borrow().as_class() {
            return class.name.clone();
        }
        if let Some(type_type) = t.borrow().as_type_type() {
            return type_type
                .instance_type
                .borrow()
                .as_class()
                .unwrap()
                .name
                .clone();
        }

        panic!("invalid type on class method call");
    }
}

impl Node for ClassMethodCallNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        asm.push_str(&self.base.borrow_mut().asm(ctx.clone())?);
        asm.push_str("\n");

        asm.push_str(&format!(
            "
            call {}
            {}
            {}
        ",
            method_id(
                self.class_id(ctx.clone()),
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
        let mut accessing_method_from_instance = true;

        let (base_type_any, _) =
            self.base.borrow_mut().type_check(ctx.clone())?;
        let mut base_type = base_type_any.borrow().as_class();
        if base_type.is_none() {
            if let Some(type_type) = base_type_any.borrow().as_type_type() {
                let class_type = type_type.instance_type.borrow().as_class();

                if class_type.is_none() {
                    return Err(type_error(format!(
                        "Cannot access methods statically for type '{}'",
                        type_type.str()
                    ))
                    .set_interval(self.position.clone()));
                }

                // if the type is a TypeType,
                // is means we are doing something like this:
                //      class C { ... }
                //      C.method()
                // and accessing the method statically rather than through
                // an instance, which we want to allow but must be dealt with
                // explicitly, and unwrap the type before accessing methods.
                base_type = Some(class_type.unwrap());
                accessing_method_from_instance = false;
            } else {
                return Err(type_error(format!(
                    "Cannot access method of non-class type '{}'",
                    base_type_any.borrow_mut().str()
                ))
                .set_interval(self.position.clone()));
            }
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

        // deal with arguments
        let mut args: Vec<FnParamType> = Vec::new();

        if accessing_method_from_instance {
            args.push(FnParamType {
                name: "self".to_string(),
                type_: base_type_any,
                default_value: None,
            });
        }

        for arg in self.args.iter_mut() {
            let (arg_type, _) = arg.borrow_mut().type_check(ctx.clone())?;
            args.push(FnParamType {
                // calling the function, so parameter name is not known
                name: "".to_string(),
                type_: arg_type,
                default_value: None,
            });
        }

        // compare call signatures of declaration to callee
        let call_signature_type = new_mut_rc(FnType {
            name: self.name.clone().literal.unwrap(),
            ret_type: method_type.ret_type.clone(),
            parameters: args,
        });

        if !method_type.contains(call_signature_type.clone()) {
            return Err(mismatched_types(
                new_mut_rc(method_type),
                call_signature_type.clone(),
            ).set_interval(self.pos()));
        }

        // fill out default arguments
        let num_args = if accessing_method_from_instance {
            // Arg is not added to self.args, rather inserted when we know we are calling on
            // an instance rather than statically,
            // So that it doesn't try to fill out the default value for whatever
            // the last argument is.
            self.args.len() + 1
        } else {
            self.args.len()
        };
        for i in num_args..method_type.parameters.len() {
            // add to end of vec
            self.args.insert(
                self.args.len(),
                method_type.parameters[i].default_value.clone().unwrap(),
            );
        }

        self.use_return_value = !method_type
            .ret_type
            .borrow()
            .contains(get_type!(ctx, "Void"));

        Ok((method_type.ret_type.clone(), None))
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
