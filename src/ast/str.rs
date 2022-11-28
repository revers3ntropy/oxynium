use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct StrNode {
    pub value: String
}

impl Node for StrNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq \"{}\", 0", self.value);
        let reference = ctx.declare_anon_data(data, true, ctx.get_from_id("Str").type_.clone());
        Ok(format!("push {}", reference))
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_from_id("Str").type_.clone())
    }
}