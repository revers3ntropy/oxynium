use crate::ast::Node;
use crate::ast::types::Type;
use crate::context::Context;
use crate::error::Error;

const STD_ASM: &str = include_str!("../../std/std.asm");

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: Box<dyn Node>
}

impl Node for ExecRootNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        // println!("Generating assembly for program: {:?}", self.statement);

        let res = self.statements.asm(ctx)?;
        let decls = ctx.get_global_vars().iter().map(|k| {
            format!("{} {}", k.name, k.data.as_ref().unwrap())
        }).collect::<Vec<String>>().join("\n");

        if ctx.exec_mode == 2 {
            return Ok(format!("
                section .data
                    {decls}
            "));
        }

        let end_statements: &str;
        if ctx.exec_mode == 1 {
            end_statements = "
                call print_stack
                call print_nl
            ";
        } else {
            end_statements = "";
        }

        Ok(format!("
            %include \"std.asm\"
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
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        self.statements.type_check(ctx)?;
        Ok(ctx.get_from_id("Void").type_.clone())
    }
}

#[derive(Debug)]
pub struct EmptyExecRootNode {}

impl Node for EmptyExecRootNode {
    fn asm(&mut self, ctx: &mut Context) -> Result<String, Error> {
        if ctx.exec_mode == 2 {
            Ok(format!("
                section .text
            "))
        } else {
            Ok(format!("
                section .text
                    global _start
                    {STD_ASM}
                _start:
                    call exit
            "))
        }
    }

    fn type_check(&mut self, ctx: &mut Context) -> Result<Box<Type>, Error> {
        Ok(ctx.get_from_id("Void").type_.clone())
    }
}