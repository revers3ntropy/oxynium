use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct ForLoopNode {
    pub statements: Box<dyn Node>
}

impl Node for ForLoopNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let body = self.statements.asm(ctx)?;
        Ok(format!("
            loop_start:
                {body}
                jmp loop_start
            loop_end:
        "))
    }
}