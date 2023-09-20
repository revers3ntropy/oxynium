use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::types::function::{FnParamType, FnType};
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug, Clone)]
pub struct FnTypeNode {
    pub parameters: Vec<MutRc<dyn AstNode>>,
    pub ret_type: MutRc<dyn AstNode>,
    pub position: Interval,
    pub fn_tok_pos: Interval,
}

impl AstNode for FnTypeNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        for arg in &mut self.parameters {
            arg.borrow_mut().setup(ctx.clone())?;
        }
        self.ret_type.borrow_mut().setup(ctx)
    }

    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;

        let mut parameters = vec![];

        for param in self.parameters.iter() {
            let position = param.borrow().pos();
            let param_type_res = param.borrow().type_check(ctx.clone())?;
            unknowns += param_type_res.unknowns;
            parameters.push(FnParamType {
                name: format!("[anon]"),
                type_: param_type_res.t,
                default_value: None,
                position,
            });
        }

        let ret_type_res = self.ret_type.borrow().type_check(ctx.clone())?;
        unknowns += ret_type_res.unknowns;

        Ok(TypeCheckRes::from(
            new_mut_rc(FnType {
                id: ctx.borrow_mut().get_id(),
                name: format!(""),
                ret_type: ret_type_res.t,
                parameters,
                generic_args: Default::default(),
                generic_params_order: vec![],
            }),
            unknowns,
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
