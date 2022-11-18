use std::fmt::Debug;
use crate::context::Context;

const STD_ASM: &str = include_str!("./std.asm");

pub(crate) trait Node: Debug {
    fn asm(&self, ctx: &mut Context) -> String;
}

#[derive(Debug)]
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
        let data = format!("dw {}", self.value.to_string().to_owned());
        let reference = ctx.declare(data);
        format!("push {}", reference)
    }
}

#[derive(Debug)]
pub(crate) struct BinOpNode {
    lhs: Box<dyn Node>,
    operator: String,
    rhs: Box<dyn Node>
}

impl BinOpNode {
    pub fn new(lhs: Box<dyn Node>, operator: String, rhs: Box<dyn Node>) -> BinOpNode {
        BinOpNode {
            lhs,
            operator,
            rhs
        }
    }
}

impl Node for BinOpNode {
    fn asm(&self, ctx: &mut Context) -> String {
        format!(
            "{}\n{}\n   pop rax\n   pop rbx\n   mov rdx, [rbx]\n   {} [rax], rdx\n   push rax",
            self.lhs.asm(ctx),
            self.rhs.asm(ctx),
            self.operator
        )
    }
}

#[derive(Debug)]
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
        println!("Generating assembly for program: {:?}", self);
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

            {}

            main:
            _start:
                {}
                call print_stack
                call exit

        ", decls, STD_ASM, res)
    }
}