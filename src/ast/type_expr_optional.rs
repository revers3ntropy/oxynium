use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;
use std::collections::HashMap;

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

        let generics = HashMap::from([(generic_arg_name, t)]);

        let type_res = option.borrow().concrete(&generics, &mut HashMap::new())?;

        Ok(TypeCheckRes::from(type_res, unknowns))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
