use crate::args::{get_args_cmd, get_cli_args, Args};
use crate::ast::types::atomic::AtomicType;
use crate::context::Context;
use crate::error::{io_error, Error};
use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::post_process::format_asm::post_process;
use crate::symbols::{SymbolDec, SymbolDef};
use crate::util::MutRc;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command as Exec;
use std::rc::Rc;

mod args;
mod ast;
mod context;
mod error;
mod parse;
mod position;
mod post_process;
mod symbols;
mod util;

const STD_DOXY: &str = include_str!("../std/std.doxy");

fn setup_ctx_with_doxy(ctx: MutRc<Context>) -> Result<MutRc<Context>, Error> {
    // declare the built in types
    ctx.borrow_mut().declare(SymbolDec {
        name: "Int".to_string(),
        id: "Int".to_string(),
        is_constant: true,
        is_type: true,
        require_init: false,
        is_defined: true,
        is_param: false,
        type_: Rc::new(AtomicType {
            id: 0,
            name: "Int".to_string(),
            is_ptr: false,
        }),
    })?;
    ctx.borrow_mut().declare(SymbolDec {
        name: "Bool".to_string(),
        id: "Bool".to_string(),
        is_constant: true,
        is_type: true,
        require_init: false,
        is_defined: true,
        is_param: false,
        type_: Rc::new(AtomicType {
            id: 1,
            name: "Bool".to_string(),
            is_ptr: false,
        }),
    })?;
    ctx.borrow_mut().declare(SymbolDec {
        name: "Str".to_string(),
        id: "Str".to_string(),
        is_constant: true,
        is_type: true,
        require_init: false,
        is_defined: true,
        is_param: false,
        type_: Rc::new(AtomicType {
            id: 2,
            name: "Str".to_string(),
            is_ptr: true,
        }),
    })?;
    ctx.borrow_mut().declare(SymbolDec {
        name: "Void".to_string(),
        id: "Void".to_string(),
        is_constant: true,
        is_type: true,
        require_init: false,
        is_defined: true,
        is_param: false,
        type_: Rc::new(AtomicType {
            id: 3,
            name: "Void".to_string(),
            is_ptr: true,
        }),
    })?;

    let mut lexer = Lexer::new(STD_DOXY.to_owned(), "std.doxy".to_owned());
    let tokens = lexer.lex();
    if tokens.is_err() {
        return Err(tokens.err().unwrap());
    }

    let mut parser = Parser::new(tokens.unwrap());
    let ast = parser.parse();

    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    let node = ast.node.unwrap();
    let type_check_res = node.borrow_mut().type_check(ctx.clone());
    if type_check_res.is_err() {
        return Err(type_check_res.err().unwrap());
    }
    let asm_error = node.borrow_mut().asm(ctx.clone());
    if asm_error.is_err() {
        return Err(asm_error.err().unwrap());
    }

    ctx.borrow_mut().define(SymbolDef {
        name: "true".to_string(),
        data: Some("dq 1".to_string()),
        text: None
    })?;
    ctx.borrow_mut().define(SymbolDef {
        name: "false".to_string(),
        data: Some("dq 0".to_string()),
        text: None
    })?;

    Ok(ctx)
}

fn compile(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(String, MutRc<Context>), Error> {
    let ctx = setup_ctx_with_doxy(Context::new())?;
    ctx.borrow_mut().std_asm_path = args.std_path.clone();
    ctx.borrow_mut().exec_mode = args.exec_mode;

    let mut lexer = Lexer::new(input.clone(), file_name);
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    let root_node = ast.node.unwrap();
    root_node.borrow_mut().type_check(ctx.clone())?;
    let compile_res = root_node.borrow_mut().asm(ctx.clone())?;

    let asm = post_process(compile_res, args);
    Ok((asm, ctx.clone()))
}

fn compile_and_assemble(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(), Error> {
    let compile_res = compile(input, file_name, args)?;

    let asm_out_file = format!("{}.asm", args.out);
    let o_out_file = format!("{}.o", args.out);

    let file = File::create(asm_out_file.clone());
    if file.is_err() {
        return Err(io_error(format!(
            "Could not create assembly ('{asm_out_file}') file"
        )));
    }
    file.unwrap()
        .write_all(compile_res.0.as_bytes())
        .expect("Could not write assembly output");

    let nasm_out = Exec::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(asm_out_file.clone().as_str())
        .arg("-o")
        .arg(o_out_file.clone().as_str())
        .output()
        .expect("Could not assemble");
    if !nasm_out.status.success() {
        return Err(io_error(String::from_utf8(nasm_out.stderr).unwrap()));
    }

    if args.exec_mode == 0 {
        let ls_out = Exec::new("gcc")
            .arg("-Wall")
            .arg("-no-pie")
            .arg(o_out_file.clone().as_str())
            .arg("-e")
            .arg("main")
            .arg("-o")
            .arg(args.out.clone().as_str())
            .output()
            .expect("Could not assemble");
        if !ls_out.status.success() {
            return Err(io_error(String::from_utf8(ls_out.stderr).unwrap()));
        }
    }

    if !args.keep {
        fs::remove_file(asm_out_file).expect("Could not remove assembly file");
        fs::remove_file(o_out_file).expect("Could not remove object file");
    }

    Ok(())
}

fn print_usage() {
    println!("{}", get_args_cmd().render_usage());
}

fn main() -> std::io::Result<()> {
    let mut e = std::io::stderr();

    let args = get_cli_args();

    if !args.input.is_empty() && !args.eval.is_empty() {
        let _ = e.write(
            "Cannot specify both 'input' and 'eval' options\n".as_bytes(),
        );
        return Ok(());
    }
    if args.exec_mode != 1 && !Path::new(&args.std_path).exists() {
        let _ = e.write(
            format!(
                "STD file '{}' does not exist or is not accessible\n",
                args.std_path
            )
            .as_bytes(),
        );
        return Ok(());
    }

    if !args.eval.is_empty() {
        let args_ = args.clone();
        let res = compile_and_assemble(args.eval, "CLI".to_owned(), &args_);
        if res.is_err() {
            let _ =
                e.write(format!("{}\n", res.err().unwrap().str()).as_bytes());
        }
        return Ok(());
    }

    if !args.input.is_empty() {
        if !Path::new(args.input.as_str()).exists() {
            let _ = e.write(
                format!("Path '{}' doesn't exist\n", args.input).as_bytes(),
            );
            return Ok(());
        }

        let mut input_file = File::open(args.input.clone())?;
        let mut input = String::new();
        input_file.read_to_string(&mut input)?;

        let res = compile_and_assemble(input, args.input.clone(), &args);
        if res.is_err() {
            let _ =
                e.write(format!("{}\n", res.err().unwrap().str()).as_bytes());
        }
        return Ok(());
    }

    print_usage();
    Ok(())
}
