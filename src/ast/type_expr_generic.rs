use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::position::Interval;
use crate::types::r#type::TypeType;
use crate::types::Type;
use crate::util::{mut_rc, MutRc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GenericTypeNode {
    pub base: MutRc<dyn AstNode>,
    pub generic_args: Vec<MutRc<dyn AstNode>>,
    pub position: Interval,
}

impl AstNode for GenericTypeNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        for arg in &mut self.generic_args {
            arg.borrow_mut().setup(ctx.clone())?;
        }
        self.base.borrow_mut().setup(ctx)
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        let raw_type = self.base.borrow().type_check(ctx.clone())?.t;
        if raw_type.borrow().is_unknown() {
            if ctx.borrow().throw_on_unknowns() {
                return Err(
                    unknown_symbol(format!("Generic '{}'", raw_type.borrow().str(),))
                        .set_interval(self.pos()),
                );
            }
            for arg in self.generic_args.clone() {
                let field_type_res = arg.borrow().type_check(ctx.clone())?;
                unknowns += field_type_res.unknowns;
            }
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }

        let mut is_type_type = false;

        let mut class_type = raw_type.clone().borrow().as_class();
        if class_type.is_none() {
            let type_type = raw_type.clone().borrow().as_type_type();
            if let Some(type_type) = type_type {
                class_type = type_type.instance_type.borrow().as_class();
                is_type_type = true;
            }
        }
        if class_type.is_none() {
            return Err(type_error(format!(
                "expected class type, found `{}`",
                self.base.borrow().type_check(ctx.clone())?.t.borrow().str()
            ))
            .set_interval(self.position.clone()));
        }
        let class_type = class_type.unwrap();

        if self.generic_args.len() != class_type.generic_params_order.len() {
            return Err(type_error(format!(
                "expected {} generic arguments, found {}",
                class_type.generic_params_order.len(),
                self.generic_args.len()
            ))
            .set_interval(self.position.clone()));
        }

        let mut generics = HashMap::new();

        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res = arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name = class_type.generic_params_order[i].clone();
            generics.insert(name.literal.unwrap().clone(), arg_type_res.t);
            i += 1;
        }
        let mut class_type = class_type.clone();
        class_type.generic_args = generics.clone();

        let res_type = mut_rc(
            class_type
                .concrete(&HashMap::new(), &mut HashMap::new())?
                .borrow()
                .as_class()
                .unwrap(),
        );

        if is_type_type {
            // preserve wrapper
            Ok(TypeCheckRes::from(
                mut_rc(TypeType {
                    instance_type: res_type,
                }),
                unknowns,
            ))
        } else {
            Ok(TypeCheckRes::from(res_type, unknowns))
        }
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
