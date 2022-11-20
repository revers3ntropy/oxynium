use crate::ast::Node;
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
        format!("
                {}
                {}
                pop rax
                pop rbx
                mov rbx, [rbx]
                {} [rax], rbx
                push rax
            ",
            self.rhs.asm(ctx),
            self.lhs.asm(ctx),
            self.operator
        )
    }
}