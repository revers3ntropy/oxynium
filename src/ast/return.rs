use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Ctx;
use crate::error::{Error, syntax_error};

#[derive(Debug)]
pub struct ReturnNode {
    pub value: Option<Box<dyn Node>>
}

impl Node for ReturnNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let frame = ctx.borrow_mut().stack_frame_peak();
        if frame.is_none() {
            return Err(syntax_error("'return' statement outside of function".to_string()));
        }
        let frame = frame.unwrap();

        if let Some(mut value) = self.value.take() {
            let ret_offset = 8 * (frame.params.len() + 2);
            return Ok(format!("
                {}
                pop rax
                mov qword [rbp+{ret_offset}], rax
                jmp _$_{}_end
            ", value.asm(ctx)?, frame.name));
        }
        Ok(format!("
            jmp _$_{}_end
        ", frame.name))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<Box<Type>, Error> {
        Ok(ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone())
    }
}