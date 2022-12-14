use crate::ast::{Node, TypeCheckRes};
use crate::context::Context;
use crate::util::MutRc;
use crate::error::Error;
use crate::ast::STD_ASM;
use crate::position::Interval;

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: MutRc<dyn Node>,
}

impl Node for ExecRootNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let res = self.statements.borrow_mut().asm(ctx.clone())?;
        let mut_ref = ctx.borrow_mut();
        let (data_decls, text_decls) = mut_ref.get_definitions();
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
            {STD_ASM}
            {text}
            main:
                mov rbp, rsp
                {res}
                mov rsp, rbp
                push 0
                call exit
        ", mut_ref.std_asm_path))
    }

    fn type_check(&mut self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        self.statements.borrow_mut().type_check(ctx.clone())
    }

    fn pos(&mut self) -> Interval {
        self.statements.borrow_mut().pos()
    }
}