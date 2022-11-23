use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct StrNode {
    value: String
}

impl StrNode {
    pub fn new(value: String) -> StrNode {
        StrNode {
            value
        }
    }
}

impl Node for StrNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq \"{}\", 0", self.value);
        let reference = ctx.declare(data);
        Ok(format!("push {}", reference))
    }
}