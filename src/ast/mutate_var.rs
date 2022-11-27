use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct MutateVar {
    pub identifier: String,
    pub value: Box<dyn Node>
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        Ok(format!("
           {}
           pop rax
           mov rbx, {}
           mov rax, [rax]
           mov [rbx], rax
        ",
           self.value.asm(ctx)?,
           self.identifier
        ))
    }
}