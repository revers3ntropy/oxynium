use std::rc::Rc;
use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::{Ctx};
use crate::error::Error;

#[derive(Debug)]
pub struct StatementsNode {
    pub statements: Vec<Box<dyn Node>>
}

impl Node for StatementsNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let mut asm = String::new();
        for statement in self.statements.iter_mut() {
            let stmt = statement.asm(Rc::clone(&ctx))?;
            if !stmt.is_empty() {
                asm.push('\n');
                asm.push_str(&stmt.clone());
            }
        };
        Ok(asm)
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        for statement in self.statements.iter_mut() {
            statement.type_check(Rc::clone(&ctx))?;
        }
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}