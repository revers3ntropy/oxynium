use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<Box<dyn Node>>
}

impl Node for StatementsNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let mut asm = String::new();
        for statement in self.statements.iter_mut() {
            let stmt = statement.asm(ctx)?;
            if stmt.is_empty() {
                continue;
            }
            // asm.push('\n');
            // asm.push_str("call clear_stack");
            asm.push('\n');
            asm.push_str(&stmt.clone());
        };
        Ok(asm)
    }
}