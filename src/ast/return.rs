use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::{Error, syntax_error};
use crate::position::Interval;

#[derive(Debug)]
pub struct ReturnNode {
    pub value: Option<MutRc<dyn Node>>,
    pub position: Interval
}

impl Node for ReturnNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let frame = ctx.borrow_mut().stack_frame_peak();
        if frame.is_none() {
            return Err(syntax_error("'return' statement outside of function".to_string()));
        }
        let frame = frame.unwrap();

        if let Some(value) = self.value.take() {
            return Ok(format!("
                {}
                pop rax
                jmp {}
            ", value.borrow_mut().asm(ctx)?, frame.ret_lbl));
        }
        Ok(format!("
            jmp {}
        ", frame.ret_lbl))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if let Some(ref mut value) = self.value {
            let (t, _) = value.borrow_mut().type_check(ctx.clone())?;
            Ok((ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone(), Some(t)))
        } else {
            let void = ctx.borrow_mut().get_dec_from_id("Void")?.type_.clone();
            Ok((void.clone(), Some(void)))
        }
    }

    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}