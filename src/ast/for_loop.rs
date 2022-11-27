use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct ForLoopNode {
    pub statements: Box<dyn Node>
}

impl Node for ForLoopNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let start_lbl = ctx.get_anon_label();
        let end_lbl = ctx.get_anon_label();

        ctx.loop_labels_push(start_lbl.clone(), end_lbl.clone());
        let body = self.statements.asm(ctx)?;
        ctx.loop_labels_pop();

        Ok(format!("
            {start_lbl}:
                {body}
                jmp {start_lbl}
            {end_lbl}:
        "))
    }
}