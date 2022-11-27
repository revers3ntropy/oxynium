use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct IfNode {
    pub comparison: Box<dyn Node>,
    pub body: Box<dyn Node>
}

impl Node for IfNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let body = self.body.asm(ctx)?;
        let comp = self.comparison.asm(ctx)?;
        let after_lbl = ctx.get_anon_label();
        Ok(format!("
            {comp}
            pop rax
            mov rax, [rax]
            cmp rax, 0     ; if evaluates to false, don't do body
            je {after_lbl}
            {body}
            {after_lbl}:
        "))
    }
}