use std::rc::Rc;
use crate::ast::{Node, TypeCheckRes};
use crate::context::{Ctx};
use crate::error::Error;

#[derive(Debug)]
pub struct ForLoopNode {
    pub statements: Box<dyn Node>
}

impl Node for ForLoopNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let start_lbl = ctx.borrow_mut().get_anon_label();
        let end_lbl = ctx.borrow_mut().get_anon_label();

        ctx.borrow_mut().loop_labels_push(start_lbl.clone(), end_lbl.clone());
        let body = self.statements.asm(Rc::clone(&ctx))?;
        ctx.borrow_mut().loop_labels_pop();

        Ok(format!("
            {start_lbl}:
                {body}
                jmp {start_lbl}
            {end_lbl}:
        "))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        self.statements.type_check(Rc::clone(&ctx))
    }
}