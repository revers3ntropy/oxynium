use crate::ast::Node;
use crate::context::Context;
use crate::util::MutRc;
use crate::error::Error;
use crate::ast::STD_ASM;
use crate::position::Interval;

#[derive(Debug)]
pub struct EmptyExecRootNode {
    pub(crate) position: Interval
}

impl Node for EmptyExecRootNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        if ctx.borrow_mut().exec_mode == 1 {
            Ok(format!("
                section	.note.GNU-stack
            "))
        } else {
            Ok(format!("
                section	.note.GNU-stack
                section .text
                    global main
                    {STD_ASM}
                main:
                    push 0
                    call exit
            "))
        }
    }
    fn pos(&mut self) -> Interval {
        self.position.clone()
    }
}