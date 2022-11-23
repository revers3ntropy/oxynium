use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct FnCallNode {
    identifier: String,
    args: Vec<Box<dyn Node>>
}

impl FnCallNode {
    pub fn new(identifier: String, args: Vec<Box<dyn Node>>) -> FnCallNode {
        FnCallNode {
            identifier,
            args
        }
    }
}

impl Node for FnCallNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let mut asm = String::new();

        for arg in self.args.iter_mut().rev() {
            asm.push_str(&arg.asm(ctx)?);
            asm.push_str("\n");
        }

        asm.push_str(&format!("
            call {}
        ", self.identifier));

        Ok(asm)
    }
}