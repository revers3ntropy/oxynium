use crate::ast::Node;
use crate::context::Ctx;
use crate::error::Error;
use crate::ast::STD_ASM;

#[derive(Debug)]
pub struct EmptyExecRootNode {}

impl Node for EmptyExecRootNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
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
                    call exit
            "))
        }
    }
}