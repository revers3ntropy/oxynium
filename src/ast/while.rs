use crate::ast::{AstNode, TypeCheckRes};
use crate::context::{Context, LoopLabels};
use crate::error::{type_error, Error};
use crate::get_type;
use crate::position::Interval;
use crate::types::unknown::UnknownType;
use crate::util::{mut_rc, MutRc};

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
                    type_error("while loop condition must be of type Bool".to_string())
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
        let pre_body_lbl = ctx.borrow_mut().get_anon_label();
        let post_body_lbl = ctx.borrow_mut().get_anon_label();
        let post_loop_lbl = ctx.borrow_mut().get_anon_label();

        ctx.borrow_mut().loop_labels_push(LoopLabels {
            post_body: post_body_lbl.clone(),
            post_loop: post_loop_lbl.clone(),
        });

        // loop label exists on loop label stack just inside loop body
        let body = self.statements.borrow_mut().asm(ctx.clone())?;

        ctx.borrow_mut().loop_labels_pop();

        // `while {}` is forever loop
        if self.condition.is_none() {
            return Ok(format!(
                "
                {pre_body_lbl}:
                    {body}
                {post_body_lbl}:
                    jmp {pre_body_lbl}
                {post_loop_lbl}:
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
                {pre_body_lbl}:
                    {cond}
                    pop rax
                    test rax, rax
                    je {post_loop_lbl}
                    {body}
                {post_body_lbl}:
                    jmp {pre_body_lbl}
                {post_loop_lbl}:
            "
        ))
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
