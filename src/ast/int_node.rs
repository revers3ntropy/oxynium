use crate::ast::node::Node;
use crate::context::Context;

#[derive(Debug)]
pub(crate) struct IntNode {
    value: i32
}

impl IntNode {
    pub fn new(value: i32) -> IntNode {
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