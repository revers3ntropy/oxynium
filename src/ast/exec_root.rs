use crate::ast::{Node, TypeCheckRes};
use crate::context::Ctx;
use crate::error::Error;

const STD_ASM: &str = include_str!("../../std/std.asm");

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: Box<dyn Node>
}

impl Node for ExecRootNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        let res = self.statements.asm(ctx.clone())?;
        let mut mut_ref = ctx.borrow_mut();
        let (data_decls, text_decls) = mut_ref.get_global_definitions();
        let data = data_decls.iter().map(|k| {
            format!("{} {}", k.name, k.data.as_ref().unwrap())
        }).collect::<Vec<String>>().join("\n");

        let text = text_decls.iter().map(|k| {
            format!("{}: \n{}", k.name, k.text.as_ref().unwrap())
        }).collect::<Vec<String>>().join("\n");

        if mut_ref.exec_mode == 1 {
            return Ok(format!("
                section	.note.GNU-stack
                section .data
                    {data}
                section .text
                    {text}
            "));
        }

        Ok(format!("
            %include \"{}\"
            section	.note.GNU-stack
            section .data
                {data}
            section .text
                global main
                extern malloc
            {STD_ASM}
            {text}
            main:
                mov rbp, rsp
                {res}
                mov rsp, rbp
                call exit
        ", mut_ref.std_asm_path))
    }

    fn type_check(&mut self, ctx: Ctx) -> Result<TypeCheckRes, Error> {
        self.statements.type_check(ctx.clone())
    }
}

#[derive(Debug)]
pub struct EmptyExecRootNode {}

impl Node for EmptyExecRootNode {
    fn asm(&mut self, ctx: Ctx) -> Result<String, Error> {
        if ctx.borrow_mut().exec_mode == 1 {
            Ok(format!("
                section	.note.GNU-stack
            "))
        } else {
            Ok(format!("
                section	.note.GNU-stack
                section .text
                    global main
                    {STD_ASM}
                main:
                    call exit
            "))
        }
    }
}