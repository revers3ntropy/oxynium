use crate::ast::Node;
use crate::context::Context;
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct BreakNode {

}

impl Node for BreakNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        let labels = ctx.loop_labels_peak();
        if labels.is_none() {
            return Err(syntax_error("break statement outside of loop".to_string()));
        }
        Ok(format!("
            jmp {}
        ", labels.unwrap().1))
    }
}