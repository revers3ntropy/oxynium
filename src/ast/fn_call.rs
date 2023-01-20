use crate::ast::class_declaration::method_id;
use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::get_type;
use crate::parse::token::Token;
use crate::position::{Interval, Position};
use crate::types::function::{FnParamType, FnType};
use crate::types::unknown::UnknownType;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct FnCallNode {
    pub object: Option<MutRc<dyn AstNode>>,
    pub identifier: Token,
    pub args: Vec<MutRc<dyn AstNode>>,
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
            .t;
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

    fn get_callee_type(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<
        (
            Option<FnType>,
            bool,
            Option<MutRc<dyn Type>>,
            usize,
        ),
        Error,
    > {
        if let Some(obj) = self.object.clone() {
            let mut calling_through_instance = true;
            // getting function type on method call
            let TypeCheckRes {
                t: base_type_any,
                unknowns,
                ..
            } = obj.borrow_mut().type_check(ctx.clone())?;
            if base_type_any.borrow().is_unknown() {
                return Ok((None, false, None, unknowns));
            }
            let mut base_type =
                base_type_any.borrow().as_class();
            if base_type.is_none() {
                if let Some(type_type) =
                    base_type_any.borrow().as_type_type()
                {
                    let class_type = type_type
                        .instance_type
                        .borrow()
                        .as_class();

                    if class_type.is_none() {
                        return Err(type_error(format!(
                            "Cannot access methods statically for type '{}'",
                            type_type.str()
                        ))
                        .set_interval(self.position.clone()));
                    }

                    let type_type = type_type
                        .instance_type
                        .borrow()
                        .as_class()
                        .unwrap();
                    if type_type.generic_params_order.len()
                        > 0
                    {
                        for (_, type_) in
                            type_type.generic_args.iter()
                        {
                            if type_
                                .borrow()
                                .as_generic()
                                .is_some()
                            {
                                return Err(
                                    type_error(format!(
                                        "Can't call method on generic type `{}`",
                                        type_type.str()
                                    ))
                                        .set_interval(self.position.clone())
                                        .hint(format!(
                                            "Type `{}` requires {} generic arguments",
                                            type_type.str(),
                                            type_type.generic_params_order.len()
                                        )),
                                );
                            }
                        }
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
                        base_type_any.borrow().str()
                    ))
                    .set_interval(self.position.clone()));
                }
            }
            let base_type =
                base_type.unwrap().as_class().unwrap();

            let method_type = base_type.method_type(
                &self.identifier.clone().literal.unwrap(),
            );
            if method_type.is_none() {
                if ctx.borrow().throw_on_unknowns() {
                    return Err(type_error(format!(
                        "Class '{}' does not have method '{}'",
                        base_type.str(),
                        self.identifier
                            .clone()
                            .literal
                            .unwrap(),
                    ))
                        .set_interval(self.position.clone()));
                }
                return Ok((None, false, None, unknowns));
            }

            return Ok((
                Some(
                    method_type
                        .unwrap()
                        .borrow()
                        .as_fn()
                        .unwrap(),
                ),
                calling_through_instance,
                Some(base_type_any.clone()),
                unknowns,
            ));
        }
        // getting function type on normal function call
        if !ctx.borrow_mut().has_dec_with_id(
            &self.identifier.clone().literal.unwrap(),
        ) {
            return Ok((None, false, None, 0));
        }

        let fn_type_option = ctx
            .borrow_mut()
            .get_dec_from_id(
                &self.identifier.clone().literal.unwrap(),
            )
            .type_
            .clone()
            .borrow()
            .as_fn();
        if fn_type_option.is_none() {
            return Err(unknown_symbol(format!(
                "'{}' is not a function",
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        Ok((Some(fn_type_option.unwrap()), false, None, 0))
    }
}

impl AstNode for FnCallNode {
    fn setup(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<(), Error> {
        if let Some(obj) = self.object.clone() {
            obj.borrow_mut().setup(ctx.clone())?;
        }
        for arg in self.args.iter_mut() {
            arg.borrow_mut().setup(ctx.clone())?;
        }
        Ok(())
    }
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut args: Vec<FnParamType> = Vec::new();

        let (
            fn_type,
            is_method_call,
            base_type,
            mut unknowns,
        ) = self.get_callee_type(ctx.clone())?;

        if fn_type.is_none() {
            if !ctx.borrow().throw_on_unknowns() {
                return Ok(TypeCheckRes::unknown_and(
                    unknowns,
                ));
            }
            return Err(unknown_symbol(format!(
                "Can't find {} `{}`",
                if base_type.is_some() {
                    format!(
                        "method on type `{}`",
                        base_type.unwrap().borrow().str()
                    )
                } else {
                    "function".to_string()
                },
                self.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.identifier.interval()));
        }
        let fn_type = fn_type.unwrap();

        if is_method_call {
            args.push(FnParamType {
                name: "self".to_string(),
                type_: base_type.unwrap(),
                default_value: None,
                position: Position::unknown_interval(),
            });
        }

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
                "Too few arguments to function '{}', expected {} but found only {}",
                self.identifier.clone().literal.unwrap(),
                required_params,
                args.len()
            ))
                .set_interval(self.identifier.interval()));
        }
        if args.len() > fn_type.parameters.len() {
            return Err(type_error(format!(
                "Too many arguments to function '{}', expected only {} but found {}",
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
                let expected = fn_type.parameters[i]
                    .type_
                    .borrow()
                    .str();
                let found = args[i].type_.borrow().str();
                return Err(type_error(format!(
                    "Expected argument {} to function '{}' to be of type '{expected}' but found type '{found}'",
                    i + 1,
                    self.identifier.clone().literal.unwrap(),
                ))
                    .set_interval(args[i].position.clone()));
            }
        }

        if fn_type.ret_type.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "Unknown return type for `{}`",
                    fn_type.str()
                ))
                .set_interval(self.identifier.interval()));
            }
            unknowns += 1;
        }
        Ok(TypeCheckRes::from(
            fn_type.ret_type.clone(),
            unknowns,
        ))
    }

    fn asm(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<String, Error> {
        let mut asm = format!("");

        let (fn_type, calling_through_instance, _, _) =
            self.get_callee_type(ctx.clone())?;

        // logically impossible for this to not exist -
        // would require unresolved unknowns
        let fn_type = fn_type.expect(
            format!("function type not found: {:#?}\n\nctx:{}\n",
                    self,
                    ctx.borrow().str()
            )
                .as_str(),
        );

        let mut args = self.args.clone();

        let num_args = self.args.len()
            + if calling_through_instance { 1 } else { 0 };
        let num_params = fn_type.parameters.len();

        // fill out default arguments
        for i in num_args..fn_type.parameters.len() {
            // add to end of vec
            args.insert(
                args.len(),
                fn_type.parameters[i]
                    .default_value
                    .clone()
                    .unwrap(),
            );
        }

        for arg in args.iter_mut().rev() {
            asm.push_str(
                &arg.borrow_mut().asm(ctx.clone())?,
            );
            asm.push_str("\n");
        }

        if let Some(obj) = self.object.clone() {
            asm.push_str(
                &obj.borrow_mut().asm(ctx.clone())?,
            );
            asm.push_str("\n");
        }

        let use_return_value = !get_type!(ctx, "Void")
            .borrow()
            .contains(fn_type.ret_type);

        asm.push_str(&format!(
            "
            call {}
            add rsp, {}
            {}
        ",
            if self.object.is_some() {
                method_id(
                    self.class_id(ctx.clone()),
                    self.identifier
                        .clone()
                        .literal
                        .unwrap(),
                )
            } else {
                self.identifier.clone().literal.unwrap()
            },
            num_params * 8,
            if use_return_value {
                "
                push rax"
            } else {
                ""
            }
        ));

        Ok(asm)
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
