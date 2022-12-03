use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct ContinueNode {}

impl Node for ContinueNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let labels = ctx.loop_labels_peak();
        if labels.is_none() {
            return Err(syntax_error("continue statement outside of loop".to_string()));
        }
        Ok(format!("
            jmp {}
        ", labels.unwrap().0))
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_dec_from_id("Void").type_.clone())
    }
}