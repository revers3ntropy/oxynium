use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub struct StatementsNode {
    statements: Vec<Box<dyn Node>>
}

impl StatementsNode {
    pub fn new(statements: Vec<Box<dyn Node>>) -> StatementsNode {
        StatementsNode {
            statements
        }
    }
}

impl Node for StatementsNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        let mut asm = String::new();
        for statement in self.statements.iter_mut() {
            asm.push_str(&statement.asm(ctx));
            asm.push('\n');
        };
        asm
    }
}