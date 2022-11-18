use crate::ast::node::Node;
use crate::context::Context;

#[derive(Debug)]
pub(crate) struct ArithmeticBinOpNode {
    lhs: Box<dyn Node>,
    operator: String,
    rhs: Box<dyn Node>
}

impl ArithmeticBinOpNode {
    pub fn new(lhs: Box<dyn Node>, operator: String, rhs: Box<dyn Node>) -> ArithmeticBinOpNode {
        ArithmeticBinOpNode {
            lhs,
            operator,
            rhs
        }
    }
}

impl Node for ArithmeticBinOpNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        format!(
            "{}\n{}\n   pop rax\n   pop rbx\n   mov rdx, [rbx]\n   {} [rax], rdx\n   push rax",
            self.rhs.asm(ctx),
            self.lhs.asm(ctx),
            self.operator
        )
    }
}