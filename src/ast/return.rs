use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::error::{syntax_error, Error};
use crate::get_type;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ReturnNode {
    pub value: Option<MutRc<dyn Node>>,
    pub position: Interval,
}

impl Node for ReturnNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let frame = ctx.borrow_mut().stack_frame_peak();
        if frame.is_none() {
            return Err(syntax_error(
                "'return' statement outside of function".to_string(),
            )
            .set_interval(self.pos()));
        }
        let frame = frame.unwrap();

        if let Some(value) = self.value.take() {
            return Ok(format!(
                "
                {}
                pop rax
                jmp {}
            ",
                value.borrow_mut().asm(ctx)?,
                frame.ret_lbl
            ));
        }
        Ok(format!(
            "
            jmp {}
        ",
            frame.ret_lbl
        ))
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        if let Some(ref value) = self.value {
            let ret_tr = value.borrow().type_check(ctx.clone())?;
            Ok(TypeCheckRes::returns(true, ret_tr.t, ret_tr.unknowns))
        } else {
            Ok(TypeCheckRes::returns(true, get_type!(ctx, "Void"), 0))
        }
    }

    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
