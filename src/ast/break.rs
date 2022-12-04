use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Ctx;
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct BreakNode {}

impl Node for BreakNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let labels = ctx.borrow_mut().loop_label_peak();
        if labels.is_none() {
            return Err(syntax_error("'break' statement outside of loop".to_string()));
        }
        Ok(format!("
            jmp {}
        ", labels.unwrap().1))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}