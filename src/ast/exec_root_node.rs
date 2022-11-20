use crate::ast::Node;
use crate::context::Context;

const STD_ASM: &str = include_str!("../../std/std.asm");

#[derive(Debug)]
pub(crate) struct ExecRootNode {
    statement: Option<Box<dyn Node>>
}

impl ExecRootNode {
    pub fn new(statement: Option<Box<dyn Node>>) -> ExecRootNode {
        ExecRootNode {
            statement
        }
    }
}

impl Node for ExecRootNode {
    fn asm(&mut self, ctx: &mut Context) -> String {
        println!("Generating assembly for program: {:?}", self);
        if let Some(statement) = &mut self.statement {

            let res = statement.asm(ctx);

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

        } else {
            format!("
                section .data
                section .text
                global main
                global _start
                    {}
                main:
                _start:
                    call exit
            ", STD_ASM)
        }
    }
}