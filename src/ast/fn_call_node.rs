use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub struct FnCallNode {
    identifier: String
}

impl FnCallNode {
    pub fn new(identifier: String) -> FnCallNode {
        FnCallNode {
            identifier
        }
    }
}

impl Node for FnCallNode {
    fn asm(&mut self, _: &mut Context) -> String {
        format!("call {}", self.identifier)
    }
}