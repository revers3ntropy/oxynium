use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{type_error, Error};
use crate::get_type;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct WhileLoopNode {
    pub condition: Option<MutRc<dyn Node>>,
    pub statements: MutRc<dyn Node>,
    pub position: Interval,
}

impl Node for WhileLoopNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
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

    fn type_check(
        &mut self,
        ctx: MutRc<Context>,
    ) -> Result<TypeCheckRes, Error> {
        if let Some(condition) = &self.condition {
            let (cond_type, _) =
                condition.borrow_mut().type_check(ctx.clone())?;
            if !get_type!(ctx, "Bool").borrow().contains(cond_type) {
                return Err(type_error(
                    "while loop condition must be a bool".to_string(),
                )
                .set_interval(condition.borrow_mut().pos()));
            }
        }
        self.statements.borrow_mut().type_check(ctx.clone())
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}
