use crate::ast::{AstNode, TypeCheckRes};
use crate::context::scope::Scope;
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::symbols::SymbolDec;
use crate::util::MutRc;

#[derive(Debug, Clone)]
pub struct OptionalTypeNode {
    pub value: MutRc<dyn AstNode>,
    pub position: Interval,
}

impl AstNode for OptionalTypeNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.value.borrow_mut().setup(ctx)
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let root_ctx = ctx.clone().borrow().global_scope();

        let option = root_ctx.borrow().get_dec_from_id("Option").type_;

        let generics_ctx = Scope::new_local(root_ctx);

        let generic_arg_name = option
            .clone()
            .borrow()
            .as_class()
            .unwrap()
            .generic_params_order
            .first()
            .unwrap()
            .clone()
            .literal
            .unwrap();

        let TypeCheckRes { unknowns, t, .. } = self.value.borrow().type_check(ctx.clone())?;

        generics_ctx.borrow_mut().declare(
            SymbolDec {
                name: generic_arg_name.clone(),
                id: generic_arg_name,
                is_constant: false,
                is_type: true,
                is_func: false,
                type_: t,
                require_init: false,
                is_defined: false,
                is_param: false,
                position: self.position.clone(),
            },
            self.position.clone(),
        )?;

        let type_res = option.borrow().concrete(generics_ctx)?;

        Ok(TypeCheckRes::from(type_res, unknowns))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
