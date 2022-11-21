use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub struct TermBinOpNode {
    lhs: Box<dyn Node>,
    operator: String,
    rhs: Box<dyn Node>,
    output_register: String
}

impl TermBinOpNode {
    pub fn new(lhs: Box<dyn Node>, operator: String, rhs: Box<dyn Node>, output_register: String) -> TermBinOpNode {
        TermBinOpNode {
            lhs,
            operator,
            rhs,
            output_register
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
                cqo ; extend rax to rdx:rax
                {} rbx
                mov [rcx], {}
                push rcx
            ",
                self.rhs.asm(ctx),
                self.lhs.asm(ctx),
                self.operator,
                self.output_register
        )
    }
}