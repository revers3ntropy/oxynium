use crate::args::ExecMode;
use crate::ast::{AstNode, TypeCheckRes};
use crate::ast::{STD_ASM, STD_DATA_ASM};
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::position::Interval;
use crate::types::function::FnType;
use crate::types::Type;
use crate::util::MutRc;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: MutRc<dyn AstNode>,
}

impl AstNode for ExecRootNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.statements.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if ctx.borrow().is_frozen() {
            panic!("Cannot type check a frozen context");
        }
        let TypeCheckRes { mut unknowns, .. } = self.statements.borrow().type_check(ctx.clone())?;

        // so that things aren't redeclared
        ctx.borrow_mut().freeze();

        // println!("(Pass 0) Unknowns: {} ", unknowns);
        #[allow(unused_variables)]
        let mut i = 0;
        while unknowns > 0 {
            i += 1;

            ctx.borrow_mut().clear_concrete_cache();
            let res = self.statements.borrow().type_check(ctx.clone())?;

            // println!(
            //     "(Pass {}) Unknowns: {} ",
            //     i, res.unknowns
            // );

            if res.unknowns >= unknowns {
                break;
            }
            unknowns = res.unknowns;
        }

        ctx.borrow_mut().finished_resolving_types();

        // especially while not stable, do this last check every time
        // but TODO: only run when there are still unknowns but no progress
        self.statements.borrow().type_check(ctx.clone())
    }

    fn asm(&mut self, ctx: MutRc<dyn Context>) -> Result<String, Error> {
        let mut res = self.statements.borrow_mut().asm(ctx.clone())?;
        let mut ctx_ref = ctx.borrow_mut();

        let (data_defs, text_defs) = ctx_ref.get_definitions();
        let data = data_defs
            .iter()
            .map(|k| format!("{} {}", k.name, k.data.as_ref().unwrap()))
            .collect::<Vec<String>>()
            .join("\n");

        let text = text_defs
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

        if ctx_ref.exec_mode() == ExecMode::Lib {
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

        let main_decl_option = text_defs.iter().find(|k| k.name == "main");
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
                id: ctx_ref.get_id(),
                name: "main".to_string(),
                ret_type: ctx_ref.get_dec_from_id("Void").type_,
                parameters: vec![],
                generic_args: HashMap::new(),
                generic_params_order: vec![],
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
                .set_interval(ctx_ref.get_dec_from_id("main").position.clone()));
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
                {res}
                push 0
                call exit
        ",
            ctx_ref.std_asm_path()
        ))
    }

    fn pos(&self) -> Interval {
        self.statements.borrow().pos()
    }
}
