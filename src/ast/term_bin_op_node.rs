use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub(crate) struct TermBinOpNode {
    lhs: Box<dyn Node>,
    operator: String,
    rhs: Box<dyn Node>
}

impl TermBinOpNode {
    pub fn new(lhs: Box<dyn Node>, operator: String, rhs: Box<dyn Node>) -> TermBinOpNode {
        TermBinOpNode {
            lhs,
            operator,
            rhs
        }
    }
}

impl Node for TermBinOpNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        format!("
                {}
                {}
                pop rcx
                pop rbx
                mov rbx, [rbx]
                mov rax, [rcx]
                {} rbx
                mov [rcx], rax
                push rcx
            ",
                self.rhs.asm(ctx),
                self.lhs.asm(ctx),
                self.operator
        )
    }
}