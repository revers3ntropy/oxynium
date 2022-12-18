use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct LoopNode {
    pub statements: MutRc<dyn Node>,
    pub position: Interval,
}

impl Node for LoopNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let start_lbl = ctx.borrow_mut().get_anon_label();
        let end_lbl = ctx.borrow_mut().get_anon_label();

        ctx.borrow_mut()
            .loop_labels_push(start_lbl.clone(), end_lbl.clone());
        let body = self.statements.borrow_mut().asm(ctx.clone())?;
        ctx.borrow_mut().loop_labels_pop();

        Ok(format!(
            "
            {start_lbl}:
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
        self.statements.borrow_mut().type_check(ctx.clone())
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}