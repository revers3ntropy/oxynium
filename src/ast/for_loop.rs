use crate::ast::Node;
use crate::ast::types::built_in::VOID;
use crate::ast::types::Type;
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

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        self.statements.type_check(ctx)?;
        Ok(Box::new(VOID))
    }
}