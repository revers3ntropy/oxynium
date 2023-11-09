use crate::args::ExecMode;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::Error;
use crate::position::Interval;
use crate::util::MutRc;

#[derive(Debug)]
pub struct EmptyExecRootNode {
    pub position: Interval,
}

impl AstNode for EmptyExecRootNode {
    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        if ctx.borrow_mut().exec_mode() == ExecMode::Lib {
            Ok(format!(
                "
                    section	.text
                "
            ))
        } else {
            Ok(format!(
                "
                    section .text
                        global main
                        main:
                            mov rax, 60
                            mov rdi, 0
                            syscall
                "
            ))
        }
    }
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
