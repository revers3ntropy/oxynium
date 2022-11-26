use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct FnCallNode {
    pub identifier: String,
    pub args: Vec<Box<dyn Node>>
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