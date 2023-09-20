use crate::ast::{AstNode, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::get_type;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::util::{new_mut_rc, MutRc};

#[derive(Debug)]
pub struct WhileLoopNode {
    pub condition: Option<MutRc<dyn AstNode>>,
    pub statements: MutRc<dyn AstNode>,
    pub position: Interval,
}

impl AstNode for WhileLoopNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        if let Some(ref condition) = self.condition.clone() {
            condition.borrow_mut().setup(ctx.clone())?;
        }
        self.statements.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        let mut unknowns = 0;
        if let Some(condition) = &self.condition {
            let TypeCheckRes {
                t: cond_type,
                unknowns: condition_unknowns,
                ..
            } = condition.borrow_mut().type_check(ctx.clone())?;
            unknowns += condition_unknowns;

            if !get_type!(ctx, "Bool").borrow().contains(cond_type) {
                return Err(
                    type_error("while loop condition must be a bool".to_string())
                        .set_interval(condition.borrow_mut().pos()),
                );
            }
        }
        let mut statements_tr = self.statements.borrow_mut().type_check(ctx.clone())?;
        statements_tr.unknowns += unknowns;
        statements_tr.always_returns = statements_tr.always_returns && self.condition.is_none();
        Ok(statements_tr)
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let start_lbl = ctx.borrow_mut().get_anon_label();
        let end_lbl = ctx.borrow_mut().get_anon_label();

        ctx.borrow_mut()
            .loop_labels_push(start_lbl.clone(), end_lbl.clone());
        let body = self.statements.borrow_mut().asm(ctx.clone())?;
        ctx.borrow_mut().loop_labels_pop();

        if self.condition.is_none() {
            return Ok(format!(
                "
                {start_lbl}:
                    {body}
                    jmp {start_lbl}
                {end_lbl}:
            "
            ));
        }

        let cond = self
            .condition
            .take()
            .unwrap()
            .borrow_mut()
            .asm(ctx.clone())?;

        Ok(format!(
            "
            {start_lbl}:
                {cond}
                pop rax
                test rax, rax
                je {end_lbl}
                {body}
                jmp {start_lbl}
            {end_lbl}:
        "
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
