use crate::ast::{Node, TypeCheckRes};
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
                jmp {}
            ", value.asm(ctx)?, frame.ret_lbl));
        }
        Ok(format!("
            jmp {}
        ", frame.ret_lbl))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        if let Some(ref mut value) = self.value {
            let (t, _) = value.type_check(ctx)?;
            Ok((t.clone(), Some(t)))
        } else {
            let void = ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone();
            Ok((void.clone(), Some(void)))
        }
    }
}