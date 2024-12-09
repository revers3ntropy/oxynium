use crate::ast::fn_call::{BaseTypeInfo, CalleeTypeInfo, FnCallNode};
use crate::ast::{AstNode, CallableType, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::position::Interval;
use crate::types::generic_function::GenericFnType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;
use std::ops::Deref;

struct GenericCalleeTypeInfo {
    fn_type: Option<GenericFnType>,
    calling_through_instance: bool,
    base_type: Option<MutRc<dyn Type>>,
    unknowns: usize,
    dec_id: String,
}

#[derive(Debug)]
pub struct GenericFnCallNode {
    pub fn_call_node: FnCallNode,
    pub generic_args: Vec<MutRc<dyn AstNode>>,
}

impl GenericFnCallNode {
    fn get_callee_type_on_base(
        &self,
        ctx: MutRc<dyn Context>,
        base: MutRc<dyn AstNode>,
    ) -> Result<GenericCalleeTypeInfo, Error> {
        let BaseTypeInfo {
            base_type,
            unknowns,
            dec_id,
            calling_through_instance,
        } = self.fn_call_node.get_base_type(ctx.clone(), base.clone())?;

        if base_type.is_none() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "cannot find type of '{}'",
                    base.borrow().type_check(ctx.clone())?.t.borrow().str()
                ))
                .set_interval(self.fn_call_node.position.clone()));
            }
            return Ok(GenericCalleeTypeInfo {
                fn_type: None,
                calling_through_instance: false,
                base_type: None,
                unknowns,
                dec_id,
            });
        }
        let base_type = base_type.unwrap();

        let method_type =
            base_type.method_type(&self.fn_call_node.identifier.clone().literal.unwrap());
        if method_type.is_none() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(type_error(format!(
                    "class '{}' does not have method '{}'",
                    base_type.str(),
                    self.fn_call_node.identifier.clone().literal.unwrap(),
                ))
                .set_interval(self.fn_call_node.position.clone()));
            }
            return Ok(GenericCalleeTypeInfo {
                fn_type: None,
                calling_through_instance: false,
                base_type: None,
                unknowns,
                dec_id,
            });
        }
        match method_type.unwrap().borrow().deref() {
            CallableType::Fn(_) => Err(type_error(format!(
                "cannot call generic method '{}' directly",
                self.fn_call_node.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.fn_call_node.position.clone())),
            CallableType::GenericFn(fn_type) => Ok(GenericCalleeTypeInfo {
                fn_type: Some(fn_type.clone()),
                calling_through_instance,
                base_type: Some(mut_rc(base_type.clone())),
                unknowns,
                dec_id,
            }),
        }
    }

    fn get_callee_type(&self, ctx: MutRc<dyn Context>) -> Result<GenericCalleeTypeInfo, Error> {
        if let Some(obj) = self.fn_call_node.base.clone() {
            return self.get_callee_type_on_base(ctx.clone(), obj);
        }

        // getting function type on normal function call
        if !ctx
            .borrow_mut()
            .has_dec_with_id(&self.fn_call_node.identifier.clone().literal.unwrap())
        {
            return Ok(GenericCalleeTypeInfo {
                fn_type: None,
                calling_through_instance: false,
                base_type: None,
                unknowns: 0,
                dec_id: "".to_string(),
            });
        }

        let fn_dec = ctx
            .borrow_mut()
            .get_dec_from_id(&self.fn_call_node.identifier.clone().literal.unwrap());

        let fn_type_option = fn_dec.type_.clone();
        let fn_type_as_fn = fn_type_option.borrow().as_generic_fn();
        if fn_type_as_fn.is_none() {
            return Err(unknown_symbol(format!(
                "'{}' is not a function",
                self.fn_call_node.identifier.clone().literal.unwrap()
            ))
            .set_interval(self.fn_call_node.identifier.interval()));
        }
        Ok(GenericCalleeTypeInfo {
            fn_type: Some(fn_type_as_fn.unwrap()),
            calling_through_instance: false,
            base_type: None,
            unknowns: 0,
            dec_id: fn_dec.id,
        })
    }

    fn generic_callee_type_info_to_concrete(
        &self,
        ctx: MutRc<dyn Context>,
        info: GenericCalleeTypeInfo,
    ) -> Result<CalleeTypeInfo, Error> {
        if info.fn_type.is_none() {
            return Ok(CalleeTypeInfo {
                fn_type: None,
                calling_through_instance: false,
                base_type: None,
                unknowns: info.unknowns,
                dec_id: info.dec_id,
            });
        }
        let fn_type = info.fn_type.unwrap();

        let mut unknowns = 0;
        let mut generics = HashMap::new();
        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name = fn_type.generic_params_order[i].clone().literal.unwrap();
            generics.insert(name, arg_type_res.t.borrow().concrete(&HashMap::new())?);
            i += 1;
        }

        let fn_type_concrete = fn_type.concrete(&generics)?.borrow().as_fn().unwrap();

        Ok(CalleeTypeInfo {
            fn_type: Some(fn_type_concrete.clone()),
            calling_through_instance: info.calling_through_instance,
            base_type: info.base_type,
            unknowns,
            dec_id: info.dec_id,
        })
    }
}

impl AstNode for GenericFnCallNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.fn_call_node.setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let GenericCalleeTypeInfo {
            fn_type,
            calling_through_instance,
            base_type,
            mut unknowns,
            ..
        } = self.get_callee_type(ctx.clone())?;

        if fn_type.is_none() {
            return self
                .fn_call_node
                .definition_not_found(ctx.clone(), unknowns, base_type);
        }
        let fn_type = fn_type.unwrap();

        if self.generic_args.len() > fn_type.generic_params_order.len() {
            return Err(type_error(format!(
                "too many generic arguments for function `{}`",
                fn_type.str()
            ))
            .set_interval(self.fn_call_node.position.clone())
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
            .set_interval(self.fn_call_node.position.clone())
            .hint(format!(
                "function `{}` requires {} generic arguments",
                fn_type.str(),
                fn_type.generic_params_order.len()
            )));
        }

        let mut generics = HashMap::new();
        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name = fn_type.generic_params_order[i].clone().literal.unwrap();
            generics.insert(name, arg_type_res.t.borrow().concrete(&HashMap::new())?);
            i += 1;
        }

        let fn_type = fn_type.concrete(&generics)?.borrow().as_fn().unwrap();

        self.fn_call_node.type_check_with_concrete_callee_type(
            ctx.clone(),
            fn_type,
            unknowns,
            if calling_through_instance {
                Some(base_type.unwrap())
            } else {
                None
            },
        )
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        self.fn_call_node.generate_asm_using_callee_type(
            ctx.clone(),
            self.generic_callee_type_info_to_concrete(
                ctx.clone(),
                self.get_callee_type(ctx.clone())?,
            )?,
        )
    }

    fn pos(&self) -> Interval {
        self.fn_call_node.pos()
    }
}
