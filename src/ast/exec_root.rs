use crate::ast::{Node, TypeCheckRes};
use crate::ast::{STD_ASM, STD_DATA_ASM};
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::position::Interval;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::MutRc;

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: MutRc<dyn Node>,
}

impl Node for ExecRootNode {
    fn asm(&mut self, ctx: MutRc<Context>) -> Result<String, Error> {
        let mut res = self.statements.borrow_mut().asm(ctx.clone())?;
        let ctx_ref = ctx.borrow_mut();

        let (data_decls, text_decls) = ctx_ref.get_definitions();
        let data = data_decls
            .iter()
            .map(|k| format!("{} {}", k.name, k.data.as_ref().unwrap()))
            .collect::<Vec<String>>()
            .join("\n");

        let text = text_decls
            .iter()
            .map(|k| {
                if k.name == "main" {
                    format!("_$_oxy_main: \n{}", k.text.as_ref().unwrap())
                } else {
                    format!("{}: \n{}", k.name, k.text.as_ref().unwrap())
                }
            })
            .collect::<Vec<String>>()
            .join("\n");

        if ctx_ref.exec_mode == 1 {
            return Ok(format!(
                "
                section	.note.GNU-stack
                section .data
                    {data}
                section .text
                    {text}
            "
            ));
        }

        let main_decl_option = text_decls.iter().find(|k| k.name == "main");
        let has_main = main_decl_option.is_some();

        if has_main && res != "" {
            return Err(syntax_error(format!(
                "Cannot have top level statements and 'main' function"
            ))
            .set_interval(ctx_ref.get_dec_from_id("main").position.clone()));
        }

        if has_main {
            res = "call _$_oxy_main".to_string();

            let main_decl = ctx_ref.get_dec_from_id("main");
            let main_type = main_decl.type_.clone();
            let main_signature = FnType {
                name: "main".to_string(),
                ret_type: ctx_ref.get_dec_from_id("Void").type_,
                parameters: vec![],
            };

            if !main_signature.contains(main_type) {
                return Err(type_error(format!(
                    "main function must have signature 'main(): Void'"
                ))
                .set_interval(main_decl.position.clone()));
            }
        }

        if ctx_ref.has_dec_with_id("main") {
            if !has_main {
                return Err(syntax_error(format!(
                    "if main function is declared it must be defined"
                ))
                .set_interval(
                    ctx_ref.get_dec_from_id("main").position.clone(),
                ));
            }
        }

        Ok(format!(
            "
            %include \"{}\"
            section	.note.GNU-stack
            section .data
                {STD_DATA_ASM}
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
        ",
            ctx_ref.std_asm_path
        ))
    }

    fn type_check(&self, ctx: MutRc<Context>) -> Result<TypeCheckRes, Error> {
        self.statements.borrow_mut().type_check(ctx.clone())
    }

    fn pos(&self) -> Interval {
        self.statements.borrow_mut().pos()
    }
}
