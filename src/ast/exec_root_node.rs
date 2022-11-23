use crate::ast::Node;
use crate::context::Context;
use crate::error::Error;

const STD_ASM: &str = include_str!("../../std/std.asm");

#[derive(Debug)]
pub struct ExecRootNode {
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
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        println!("Generating assembly for program: {:?}", self);
        if let Some(statement) = &mut self.statement {

            let res = statement.asm(ctx)?;

            let decls = &ctx.declarations.iter().map(|(k, v)| {
                format!("{k} {v}")
            }).collect::<Vec<String>>().join("\n");

            let end_statements: &str;
            if ctx.exec_mode {
                end_statements = "
                    call print_stack
                    call print_nl
                ";
            } else {
                end_statements = "";
            }

            Ok(format!("
                section .data
                    {decls}
                section .text
                    global _start

                {STD_ASM}

                _start:
                    mov rbp, rsp
                    {res}
                    {end_statements}
                    call clear_stack
                    call exit
            "))

        } else {
            Ok(format!("
                section .data
                section .text
                    global _start
                    {STD_ASM}
                _start:
                    call exit
            "))
        }
    }
}