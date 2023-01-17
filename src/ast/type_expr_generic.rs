use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, unknown_symbol, Error};
use crate::parse::token::Token;
use crate::position::Interval;
use crate::symbols::SymbolDec;
use crate::types::Type;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct GenericTypeNode {
    pub identifier: Token,
    pub generic_args: Vec<MutRc<dyn Node>>,
}

impl GenericTypeNode {
    fn id(&self) -> String {
        self.identifier.literal.as_ref().unwrap().clone()
    }
}

impl Node for GenericTypeNode {
    fn type_check(
        &self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        if !ctx.borrow().has_dec_with_id(&self.id()) {
            if ctx.borrow().throw_on_unknowns() {
                return Err(unknown_symbol(format!(
                    "Generic '{}'",
                    self.id(),
                ))
                .set_interval(self.identifier.interval()));
            }
            for arg in self.generic_args.clone() {
                let field_type_res =
                    arg.borrow().type_check(ctx.clone())?;
                unknowns += field_type_res.unknowns;
            }
            return Ok(TypeCheckRes::unknown_and(unknowns));
        }
        if !ctx.borrow().get_dec_from_id(&self.id()).is_type
        {
            return Err(type_error(format!(
                "'{}' cannot be used as a type",
                self.id()
            ))
            .set_interval(self.pos()));
        }
        let class_type = ctx
            .borrow()
            .get_dec_from_id(
                &self.identifier.clone().literal.unwrap(),
            )
            .type_
            .clone()
            .borrow()
            .as_class()
            .unwrap();

        let generics_ctx =
            Context::new(ctx.borrow().cli_args.clone());

        let mut i = 0;
        for arg in self.generic_args.clone() {
            let arg_type_res =
                arg.borrow().type_check(ctx.clone())?;
            unknowns += arg_type_res.unknowns;
            let name =
                class_type.generic_params_order[i].clone();
            generics_ctx.borrow_mut().declare(
                SymbolDec {
                    name: name.clone(),
                    id: name,
                    is_constant: true,
                    is_type: true,
                    type_: arg_type_res.t,
                    require_init: false,
                    is_defined: true,
                    is_param: true,
                    position: arg.borrow().pos(),
                },
                arg.borrow().pos(),
            )?;
            i += 1;
        }

        generics_ctx.borrow_mut().set_parent(ctx.clone());

        Ok(TypeCheckRes::from(
            new_mut_rc(
                class_type
                    .concrete(generics_ctx)?
                    .borrow()
                    .as_class()
                    .unwrap(),
            ),
            unknowns,
        ))
    }

    fn pos(&self) -> Interval {
        self.identifier.interval()
    }
}
