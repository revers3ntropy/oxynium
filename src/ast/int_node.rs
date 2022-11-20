use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub(crate) struct IntNode {
    value: i64
}

impl IntNode {
    pub fn new(value: i64) -> IntNode {
        IntNode {
            value
        }
    }
}

impl Node for IntNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        let data = format!("dq {}", self.value.to_string());
        let reference = ctx.declare(data);
        format!("push {}", reference)
    }
}