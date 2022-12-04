use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Ctx;
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct ReturnNode {}

impl Node for ReturnNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let frame = ctx.borrow_mut().stack_frame_pop();
        if frame.is_none() {
            return Err(syntax_error("'return' statement outside of function".to_string()));
        }
        Ok(format!("
            jmp _$_{}_end
        ", frame.unwrap().name))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}