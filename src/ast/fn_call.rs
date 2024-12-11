use crate::ast::class_declaration::method_id;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::Interval;
use crate::types::function::{FnParamType, FnType};
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;

struct CalleeType {
    fn_type: Option<FnType>,
    calling_through_instance: bool,
    base_type: Option<MutRc<dyn Type>>,
    unknowns: usize,
    dec_id: String,
}

#[derive(Debug)]
pub struct FnCallNode {
    pub object: Option<MutRc<dyn AstNode>>,
    pub identifier: Token,
    pub args: Vec<MutRc<dyn AstNode>>,
    pub generic_args: Vec<MutRc<dyn AstNode>>,
    pub position: Interval,
}

impl FnCallNode {
    fn class_id(&self, object_type_check_res: TypeCheckRes) -> Result<String, Error> {
        let t = object_type_check_res.t;
        if let Some(class) = t.borrow().as_class() {
            return Ok(class.name.clone());
        }
        if let Some(type_type) = t.borrow().as_type_type() {
            return Ok(type_type
                .instance_type
                .borrow()
                .as_class()
                .unwrap()
                .name
                .clone());
        }

        Err(type_error(format!(
            "cannot access methods statically for type '{}'",
            t.borrow().str()
        )))
    }

    fn declaration_id(&self, object_type_check_res: TypeCheckRes) -> Result<String, Error> {
        Ok(method_id(
            self.class_id(object_type_check_res)?,
            self.identifier.clone().literal.unwrap(),
        ))
    }

    fn get_callee_type(&self, ctx: MutRc<dyn Context>) -> Result<CalleeType, Error> {
        if let Some(obj) = self.object.clone() {
            let mut calling_through_instance = true;
            // getting function type on method call
            let obj_tc_res = obj.borrow_mut().type_check(ctx.clone())?;
            if obj_tc_res.t.borrow().is_unknown() {
                return Ok(CalleeType {
                    fn_type: None,
                    calling_through_instance: false,
                    base_type: None,
                    unknowns: obj_tc_res.unknowns,
                    dec_id: "".to_string(),
                });
            }
            let mut base_type = obj_tc_res.t.borrow().as_class();
            if base_type.is_none() {
                if let Some(type_type) = obj_tc_res.t.borrow().as_type_type() {
                    let class_type = type_type.instance_type.borrow().as_class();

                    if class_type.is_none() {
                        return Err(type_error(format!(
                            "cannot access methods statically for type '{}'",
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
                        "cannot access method of non-class type '{}'",
                        obj_tc_res.t.borrow().str()
                    ))
                    .set_interval(self.position.clone()));
                }
            }
            let base_type = base_type.unwrap().as_class().unwrap();

            let method_type = base_type.method_type(&self.identifier.clone().literal.unwrap());
            if method_type.is_none() {
                if ctx.borrow().throw_on_unknowns() {
                    return Err(type_error(format!(
                        "class '{}' does not have method '{}'",
                        base_type.str(),
                        self.identifier.clone().literal.unwrap(),
                    ))
                    .set_interval(self.position.clone()));
                }
                return Ok(CalleeType {
                    fn_type: None,
                    calling_through_instance: false,
                    base_type: None,
                    unknowns: obj_tc_res.unknowns,
                    dec_id: "".to_string(),
                });
            }
            return Ok(CalleeType {
                fn_type: Some(method_type.unwrap().borrow().as_fn().unwrap()),
                calling_through_instance,
                base_type: Some(obj_tc_res.t.clone()),
                unknowns: obj_tc_res.unknowns,
                dec_id: self.declaration_id(obj_tc_res)?,
            });
        }

        // getting function type on normal function call
        if !ctx
            .borrow_mut()
            .has_dec_with_id(&self.identifier.clone().literal.unwrap())
        {
            return Ok(CalleeType {
                fn_type: None,
                calling_through_instance: false,
                base_type: None,
                unknowns: 0,
                dec_id: "".to_string(),
            });
        }

        let fn_dec = ctx
            .borrow_mut()
            .get_dec_from_id(&self.identifier.clone().literal.unwrap());

        let fn_type_option = fn_dec.type_.clone();
        let fn_type_option = fn_type_option.borrow().as_fn();
        if fn_type_option.is_none() {
            return Err(unknown_symbol(format!(
                "'{}' is not a function",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        return Ok(CalleeType {
            fn_type: Some(fn_type_option.unwrap()),
            calling_through_instance: false,
            base_type: None,
            unknowns: 0,
            dec_id: fn_dec.id,
        });
    }
}

impl AstNode for FnCallNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        if let Some(obj) = self.object.clone() {
            obj.borrow_mut().setup(ctx.clone())?;
        }
        for arg in self.args.iter_mut() {
            arg.borrow_mut().setup(ctx.clone())?;
        }
        Ok(())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut args: Vec<FnParamType> = Vec::new();

        let CalleeType {
            fn_type,
            calling_through_instance,
            base_type,
            mut unknowns,
            ..
        } = self.get_callee_type(ctx.clone())?;

        if fn_type.is_none() {
            if !ctx.borrow().throw_on_unknowns() {
                return Ok(TypeCheckRes::unknown_and(unknowns));
            }
            return Err(unknown_symbol(format!(
                "can't find {} `{}`",
                if base_type.is_some() {
                    format!("method on type `{}`", base_type.unwrap().borrow().str())
                } else {
                    "function".to_string()
                },
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        let fn_type = fn_type.unwrap();

        if self.generic_args.len() > fn_type.generic_params_order.len() {
            return Err(type_error(format!(
                "too many generic arguments for function `{}`",
                fn_type.str()
            ))
            .set_interval(self.position.clone())
            .hint(format!(
                "function `{}` requires {} generic arguments",
                fn_type.str(),
                fn_type.generic_params_order.len()
            )));
        }

        if self.generic_args.len() < fn_type.generic_params_order.len() {
            return Err(type_error(format!(
                "not enough generic arguments for function `{}`",
                fn_type.str()
            ))
            .set_interval(self.position.clone())
            .hint(format!(
                "function `{}` requires {} generic arguments",
                fn_type.str(),
                fn_type.generic_params_order.len()
            )));
        }

        if calling_through_instance {
            args.push(FnParamType {
                name: "self".to_string(),
                type_: base_type.unwrap(),
                default_value: None,
                position: self.identifier.interval(),
            });
        }

        let mut generics = HashMap::new();

        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name = fn_type.generic_params_order[i].clone().literal.unwrap();
            generics.insert(name, arg_type_res.t);
            i += 1;
        }

        let mut fn_type = fn_type.clone();
        fn_type.generic_args = generics;
        fn_type = fn_type
            .concrete(&HashMap::new(), &mut HashMap::new())?
            .borrow()
            .as_fn()
            .unwrap();

        for arg in self.args.iter() {
            let TypeCheckRes {
                t: arg_type,
                unknowns: arg_unknowns,
                ..
            } = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_unknowns;
            args.push(FnParamType {
                // calling the function, so parameter name is not known and doesn't matter
                name: "".to_string(),
                type_: arg_type,
                default_value: None,
                position: arg.borrow().pos(),
            });
        }

        let required_params = fn_type
            .parameters
            .iter()
            .filter(|a| a.default_value.is_none())
            .count();

        if args.len() < required_params {
            return Err(type_error(format!(
                "too few arguments to function `{}`, expected {} but found only {}",
                self.identifier.clone().literal.unwrap(),
                required_params,
                args.len()
            ))
            .set_interval(self.identifier.interval()));
        }
        if args.len() > fn_type.parameters.len() {
            return Err(type_error(format!(
                "too many arguments to function `{}`, expected only {} but found {}",
                self.identifier.clone().literal.unwrap(),
                fn_type.parameters.len(),
                args.len()
            ))
            .set_interval(self.identifier.interval()));
        }

        for i in 0..args.len() {
            if !fn_type.parameters[i]
                .type_
                .borrow()
                .contains(args[i].type_.clone())
            {
                let expected = fn_type.parameters[i].type_.borrow().str();
                let found = args[i].type_.borrow().str();
                return Err(type_error(format!(
                    "expected argument {} to function '{}' to be of type '{expected}' but found type '{found}'",
                    i + 1,
                    self.identifier.clone().literal.unwrap(),
                ))
                    .set_interval(args[i].position.clone()));
            }
        }

        if fn_type.ret_type.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(
                    unknown_symbol(format!("unknown return type for `{}`", fn_type.str()))
                        .set_interval(self.identifier.interval()),
                );
            }
            unknowns += 1;
        }
        Ok(TypeCheckRes::from(fn_type.ret_type.clone(), unknowns))
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let mut asm = format!("");

        let CalleeType {
            fn_type,
            calling_through_instance,
            dec_id,
            ..
        } = self.get_callee_type(ctx.clone())?;

        // would require unresolved unknowns to be None
        let fn_type = fn_type.expect(
            format!(
                "function not found: {:#?}\n\nctx:\n{}\n",
                self,
                ctx.borrow().str()
            )
            .as_str(),
        );

        let mut args = self.args.clone();

        let num_args = self.args.len() + if calling_through_instance { 1 } else { 0 };
        let num_params = fn_type.parameters.len();

        // fill out default arguments
        for i in num_args..fn_type.parameters.len() {
            // add to end of vec
            args.insert(
                args.len(),
                fn_type.parameters[i].default_value.clone().unwrap(),
            );
        }

        for arg in args.iter_mut().rev() {
            asm.push_str(&arg.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        if let Some(obj) = self.object.clone() {
            asm.push_str(&obj.borrow_mut().asm(ctx.clone())?);
            asm.push_str("\n");
        }

        let returns_void = get_type!(ctx, "Void").borrow().contains(fn_type.ret_type);

        asm.push_str(&format!(
            "
                call {dec_id}
                {}
                {}
            ",
            if num_params > 0 {
                format!("add rsp, {}", num_params * 8)
            } else {
                "".to_string()
            },
            if returns_void { "push 0" } else { "push rax" }
        ));

        Ok(asm)
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
