use crate::context::Context;

pub(crate) trait Node {
    fn asm(&self, ctx: &mut Context) -> String;
}

pub(crate) struct IntNode {
    value: i32
}

impl IntNode {
    pub fn new(value: i32) -> IntNode {
        IntNode {
            value
        }
    }
}

impl Node for IntNode {
    fn asm(&self, ctx: &mut Context) -> String {
        let data = format!("dw \"{}\"", self.value.to_string().to_owned());
        let reference = ctx.declare(data);
        format!("push {}", reference)
    }
}

pub(crate) struct ProgramNode {
    statement: Box<dyn Node>
}

impl ProgramNode {
    pub fn new(statement: Box<dyn Node>) -> ProgramNode {
        ProgramNode {
            statement
        }
    }
}

impl Node for ProgramNode {
    fn asm(&self, ctx: &mut Context) -> String {
        let res = self.statement.asm(ctx);
        let decls = &ctx.declarations.iter().map(|(k, v)| {
            format!("{} {}", k, v)
        }).collect::<Vec<String>>().join("\n");

        format!("
            section .data
                {}
            section .text
            global main
            global _start
            main:
            _start: ; entry point to program
                push 2

                {}

                mov rax, 1
                mov rdi, 1
                pop rsi
                pop rdx
                syscall

                mov rax, 60       ; exit(
                mov rdi, 0        ;   EXIT_SUCCESS
                syscall           ; );

        ", decls, res)
    }
}