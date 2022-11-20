use crate::ast::Node;
use crate::context::Context;

#[derive(Debug)]
pub(crate) enum ArithUnaryOp {
    Minus
}

#[derive(Debug)]
pub(crate) struct ArithmeticUnaryOpNode {
    operator: ArithUnaryOp,
    rhs: Box<dyn Node>
}

impl ArithmeticUnaryOpNode {
    pub fn new(operator: ArithUnaryOp, rhs: Box<dyn Node>) -> ArithmeticUnaryOpNode {
        ArithmeticUnaryOpNode {
            operator,
            rhs
        }
    }
}

impl Node for ArithmeticUnaryOpNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        match self.operator {
            ArithUnaryOp::Minus => {
                format!("
                    {}
                    pop rcx
                    mov rax, [rcx]
                    neg rax
                    mov [rcx], rax
                    push rcx
                ",
                    self.rhs.asm(ctx)
                )
            }
        }
    }
}