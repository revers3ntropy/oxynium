use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct StrNode {
    pub value: String
}

impl Node for StrNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq \"{}\", 0", self.value);
        let reference = ctx.declare_anon_data(data, true);
        Ok(format!("push {}", reference))
    }
}