use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct MutateVar {
    identifier: String,
    value: Box<dyn Node>
}

impl MutateVar {
    pub fn new(identifier: String, value: Box<dyn Node>) -> MutateVar {
        MutateVar {
            identifier,
            value
        }
    }
}

impl Node for MutateVar {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        Ok(format!("
           {0}
           pop rax
           mov rbx, {1}
           mov rax, [rax]
           mov [rbx], rax
        ",
           self.value.asm(ctx)?,
           self.identifier
        ))
    }
}