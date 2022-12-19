use crate::ast::class_declaration::method_id;
use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{mismatched_types, unknown_symbol, Error, type_error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::function::{FnParamType, FnType};
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct FnCallNode {
    pub object: Option<MutRc<dyn Node>>,
    pub identifier: Token,
    pub args: Vec<MutRc<dyn Node>>,
    pub use_return_value: bool,
    pub position: Interval,
}

impl FnCallNode {
    fn class_id(&self, ctx: MutRc<Context>) -> String {
        let t = self
            .object
            .clone()
            .unwrap()
            .borrow_mut()
            .type_check(ctx.clone())
            .unwrap()
            .0;
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

impl Node for FnCallNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut asm = format!("");

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        if let Some(obj) = self.object.clone() {
            asm.push_str(&obj.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        asm.push_str(&format!(
            "
            call {}
            {}
            {}
        ",
            if self.object.is_some() {
                method_id(
                    self.class_id(ctx.clone()),
                    self.identifier.clone().literal.unwrap()
                )
            } else {
                self.identifier.clone().literal.unwrap()
            },
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
        let mut args: Vec<FnParamType> = Vec::new();
        let mut num_args = self.args.len();
        let fn_type = if let Some(obj) = self.object.clone() {
            let mut calling_through_instance = true;
            // getting function type on method call
            let (base_type_any, _) = obj.borrow_mut().type_check(ctx.clone())?;
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
                    calling_through_instance = false;
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
                base_type.method_type(&self.identifier.clone().literal.unwrap());
            if method_type.is_none() {
                return Err(type_error(format!(
                    "Class '{}' does not have method '{}'",
                    base_type.str(),
                    self.identifier.clone().literal.unwrap(),
                ))
                    .set_interval(self.position.clone()));
            }

            if calling_through_instance {
                args.push(FnParamType {
                    name: "self".to_string(),
                    type_: base_type_any.clone(),
                    default_value: None,
                });
                num_args += 1;
            }

            method_type.unwrap().borrow().as_fn().unwrap()

        } else {
            // getting function type on normal function call
            if !ctx.borrow_mut().has_dec_with_id(&self.identifier.clone().literal.unwrap()) {
                return Err(unknown_symbol(format!(
                    "undefined function {}",
                    self.identifier.clone().literal.unwrap()
                ))
                    .set_interval(self.position.clone()));
            }

            let fn_type_option = ctx.borrow_mut()
                .get_dec_from_id(&self.identifier.clone().literal.unwrap())
                .type_.clone()
                .borrow().as_fn();
            if fn_type_option.is_none() {
                return Err(unknown_symbol(format!(
                    "'{}' is not a function",
                    self.identifier.clone().literal.unwrap()
                ))
                    .set_interval(self.position.clone()));
            }

            fn_type_option.unwrap()
        };

        for arg in self.args.iter_mut() {
            let (arg_type, _) = arg.borrow_mut().type_check(ctx.clone())?;
            args.push(FnParamType {
                // calling the function, so parameter name is not known
                name: "".to_string(),
                type_: arg_type,
                default_value: None,
            });
        }

        let call_signature_type = new_mut_rc(FnType {
            name: self.identifier.clone().literal.unwrap(),
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
        for i in num_args..fn_type.parameters.len() {
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
