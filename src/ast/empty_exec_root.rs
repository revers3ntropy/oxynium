use crate::ast::AstNode;
use crate::backend::main_fn_id;
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
        let main_id = main_fn_id(ctx.borrow().target());
        Ok(format!(
            "
                section .text
                    global {main_id}
                {main_id}:
                    mov rax, 60
                    mov rdi, 0
                    syscall
            "
        ))
    }
    fn pos(&self) -> Interval {
        self.position.clone()
    }
}
