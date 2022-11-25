use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct BreakNode {

}

impl BreakNode {
    pub fn new() -> BreakNode {
        BreakNode {

        }
    }
}

impl Node for BreakNode {
    fn asm(&mut self, _: &mut Context) -> Result<String, Error> {
        Ok(format!("
            jmp loop_end
        "))
    }
}