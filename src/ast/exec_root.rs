use crate::args::ExecMode;
use crate::ast::STD_DATA_ASM;
use crate::ast::{std_asm, AstNode, TypeCheckRes};
use crate::backend::main_fn_id;
use crate::context::Context;
use crate::error::{syntax_error, type_error, Error};
use crate::position::{Interval, Position};
use crate::types::function::{FnParamType, FnType};
use crate::types::Type;
use crate::util::MutRc;
use std::collections::HashMap;

const LOG_TYPE_CHECK_PASSES: bool = false;

static ASM_TO_GENERATE_ARGS_LIST_FOR_MAIN_FN: &'static str = "
    pop rdi        ; argc
    mov rsi, rsp   ; argv

    ; set up 'args' array for main function
    push rbp
    mov rbp, rsp
    sub rsp, 16
    ; size of array is number of bytes,
    ; and each pointer is 8 bytes
    imul rdi, 8
    ;  create the List<Str> structure on the stack
    ; (will last for lifetime of program)
    mov qword [rbp - 8], rdi
    mov qword [rbp - 16], rsi
    ; push pointer to stack structure as first arg to oxy_main
    mov rax, rbp
    sub rax, 16
    push rax
";

#[derive(Debug)]
pub struct ExecRootNode {
    pub statements: MutRc<dyn AstNode>,
}

impl ExecRootNode {
    fn main_fn_signature_with_args(ctx: MutRc<dyn Context>) -> Result<FnType, Error> {
        // construct 'main' function signature:
        // `(fn main(args: List<Utf8Str>) Void) | (fn main() Void)`
        let list_type = ctx.borrow().get_dec_from_id("List").type_;
        let utf8str_type = ctx.borrow().get_dec_from_id("Utf8Str").type_;
        let generics_for_list = HashMap::from([("T".to_string(), utf8str_type.clone())]);
        let list_of_utf8str_type = list_type
            .borrow()
            .concrete(&generics_for_list, &mut HashMap::new())?;

        let main_id = ctx.borrow_mut().get_id();
        let void_type = ctx.borrow().get_dec_from_id("Void").type_;
        let main_signature = FnType {
            id: main_id,
            name: "main".to_string(),
            ret_type: void_type.clone(),
            parameters: vec![FnParamType {
                name: "args".to_string(),
                type_: list_of_utf8str_type,
                default_value: None,
                position: Position::unknown_interval(),
            }],
            generic_args: HashMap::new(),
            generic_params_order: vec![],
        };

        Ok(main_signature)
    }

    fn main_function_signature_without_args(ctx: MutRc<dyn Context>) -> Result<FnType, Error> {
        let main_id = ctx.borrow_mut().get_id();
        let void_type = ctx.borrow().get_dec_from_id("Void").type_;

        Ok(FnType {
            id: main_id,
            name: "main".to_string(),
            ret_type: void_type,
            parameters: vec![],
            generic_args: HashMap::new(),
            generic_params_order: vec![],
        })
    }
}

impl AstNode for ExecRootNode {
    fn setup(&mut self, ctx: MutRc<dyn Context>) -> Result<(), Error> {
        self.statements.borrow_mut().setup(ctx.clone())
    }
    fn type_check(&self, ctx: MutRc<dyn Context>) -> Result<TypeCheckRes, Error> {
        if ctx.borrow().is_frozen() {
            panic!("cannot type check a frozen context");
        }
        let TypeCheckRes { mut unknowns, .. } = self.statements.borrow().type_check(ctx.clone())?;

        // so that things aren't redeclared
        ctx.borrow_mut().freeze();

        if LOG_TYPE_CHECK_PASSES {
            println!("(Pass 0) Unknowns: {} ", unknowns);
        }
        let mut i = 0;
        while unknowns > 0 {
            i += 1;

            ctx.borrow_mut().clear_concrete_cache();
            let res = self.statements.borrow().type_check(ctx.clone())?;
            if LOG_TYPE_CHECK_PASSES {
                println!("(Pass {}) Unknowns: {} ", i, res.unknowns);
            }

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

        let ctx_ref = ctx.borrow_mut();
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

        let main_decl_option = text_defs.iter().find(|k| k.name == "main");
        let has_main = main_decl_option.is_some();

        drop(ctx_ref);

        let asm_include_directives = ctx
            .borrow()
            .get_included_asm_paths()
            .iter()
            .map(|path| format!("%include \"{}\"", path))
            .collect::<Vec<String>>()
            .join("\n");

        if ctx.borrow().exec_mode() == ExecMode::Lib {
            return Ok(format!(
                "
                    {asm_include_directives}
                    section .data
                        {data}
                    section .text
                        {text}
                "
            ));
        }

        if has_main && res != "" {
            return Err(syntax_error(format!(
                "cannot have top level statements and 'main' function"
            ))
            .set_interval(ctx.borrow().get_dec_from_id("main").position.clone()));
        }

        if has_main {
            res = "call _$_oxy_main".to_string();

            let main_decl = ctx.borrow().get_dec_from_id("main");
            let main_type = main_decl.type_.clone();

            if !ExecRootNode::main_function_signature_without_args(ctx.clone())?
                .contains(main_type.clone())
            {
                if ExecRootNode::main_fn_signature_with_args(ctx.clone())?
                    .contains(main_type.clone())
                {
                    // OPTIMISATION: only set up args array if the main function takes args
                    // as a parameter
                    res = format!("{ASM_TO_GENERATE_ARGS_LIST_FOR_MAIN_FN}\n{res}");
                } else {
                    return Err(type_error(format!(
                        "`main` function must have type `Fn (args: List<Utf8Str>) Void`"
                    ))
                    .set_interval(main_decl.position.clone()));
                }
            }
        }

        if ctx.borrow().has_dec_with_id("main") {
            if !has_main {
                return Err(syntax_error(format!(
                    "if `main` function is declared it must be defined"
                ))
                .set_interval(ctx.borrow().get_dec_from_id("main").position.clone()));
            }
        }

        let std_asm = std_asm(ctx.borrow().target());
        let main_fn_id = main_fn_id(ctx.borrow().target());
        Ok(format!(
            "
                bits 64
                %include \"{}\"
                {asm_include_directives}
                section .data
                    {STD_DATA_ASM}
                    {data}
                section .text
                    global {main_fn_id}
                {std_asm}
                {text}
                {main_fn_id}:
                    endbr64
                    {res}
                    push 0
                    call exit
            ",
            ctx.borrow().std_asm_path()
        ))
    }

    fn pos(&self) -> Interval {
        self.statements.borrow().pos()
    }
}
