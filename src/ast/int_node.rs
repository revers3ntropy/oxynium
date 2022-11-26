use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64
}

impl Node for IntNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        let reference = ctx.declare_anon_data(data, true);
        Ok(format!("push {}", reference))
    }
}