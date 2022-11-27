use crate::ast::Node;
use crate::ast::types::built_in::INT;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct IntNode {
    pub value: i64
}

impl Node for IntNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let data = format!("dq {}", self.value);
        let reference = ctx.declare_anon_data(data, true, Box::new(INT));
        Ok(format!("push {}", reference))
    }

    fn type_check(&mut self, _: &mut Context) -> Result<Box<Type>, Error> {
        Ok(Box::new(INT))
    }
}