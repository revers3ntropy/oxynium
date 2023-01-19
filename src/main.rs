use crate::args::{
    get_args_cmd, get_cli_args, Args, ExecMode,
};
use crate::context::Context;
use crate::error::{arg_error, io_error, Error};
use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::position::Position;
use crate::post_process::format_asm::post_process;
use crate::util::MutRc;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command as Exec;
use std::time::Instant;

mod args;
mod ast;
mod context;
mod error;
mod log;
mod oxy_std;
mod parse;
mod position;
mod post_process;
mod symbols;
mod types;
mod util;

const STD_DOXY: &str = include_str!("../std/std.doxy");

fn setup_ctx_with_doxy(
    ctx: MutRc<Context>,
) -> Result<MutRc<Context>, Error> {
    let start = Instant::now();

    if ctx.borrow().cli_args.exec_mode == ExecMode::Lib {
        ctx.borrow_mut().set_ignoring_definitions(true);
    }

    let mut lexer = Lexer::new(
        STD_DOXY.to_owned(),
        "std.doxy".to_owned(),
        ctx.borrow().cli_args.clone(),
    );
    let tokens = lexer.lex();
    if tokens.is_err() {
        return Err(tokens.err().unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Lexed STD");
    let start = Instant::now();

    let mut parser = Parser::new(
        ctx.borrow().cli_args.clone(),
        tokens.unwrap(),
    );
    let ast = parser.parse();
    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Parsed STD");
    let start = Instant::now();

    let node = ast.node.unwrap();
    let type_check_res =
        node.borrow_mut().type_check(ctx.clone());
    if type_check_res.is_err() {
        return Err(type_check_res.err().unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Type-checked STD");
    let start = Instant::now();

    let asm_error = node.borrow_mut().asm(ctx.clone());
    if asm_error.is_err() {
        return Err(asm_error.err().unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Compiled STD");
    let start = Instant::now();

    ctx.borrow_mut().reset();

    perf!(ctx.borrow().cli_args, start, "Reset context");

    Ok(ctx)
}

fn compile(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(String, MutRc<Context>), Error> {
    let ctx = Context::new(args.clone());
    let ctx = setup_ctx_with_doxy(ctx)?;
    ctx.borrow_mut().std_asm_path = args.std_path.clone();
    ctx.borrow_mut().exec_mode = args.exec_mode;

    let start = Instant::now();

    let mut lexer =
        Lexer::new(input.clone(), file_name, args.clone());
    let tokens = lexer.lex()?;

    perf!(ctx.borrow().cli_args, start, "Lexed");
    let start = Instant::now();

    let mut parser = Parser::new(args.clone(), tokens);
    let ast = parser.parse();
    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Parsed");
    let start = Instant::now();

    let root_node = ast.node.unwrap();
    root_node.borrow_mut().type_check(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Type-checked");
    let start = Instant::now();

    let compile_res =
        root_node.borrow_mut().asm(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Compiled");
    let start = Instant::now();

    let asm = post_process(compile_res, args);

    perf!(ctx.borrow().cli_args, start, "Post Processed");

    Ok((asm, ctx.clone()))
}

fn compile_and_assemble(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(), Error> {
    let start_pos = Position {
        file: file_name.clone(),
        idx: 0,
        line: 0,
        col: 0,
    };
    let compile_res =
        compile(input, file_name.clone(), args)?;

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

    if !args.stop_after_asm {
        let nasm_out = Exec::new("nasm")
            .arg("-f")
            .arg("elf64")
            .arg(asm_out_file.clone().as_str())
            .arg("-o")
            .arg(o_out_file.clone().as_str())
            .output()
            .expect("Could not assemble");
        if !nasm_out.status.success() {
            return Err(io_error(
                String::from_utf8(nasm_out.stderr).unwrap(),
            )
            .set_pos(
                start_pos.clone(),
                Position::unknown(),
            ));
        }

        if args.exec_mode == ExecMode::Bin {
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
                return Err(io_error(
                    String::from_utf8(ls_out.stderr)
                        .unwrap(),
                )
                .set_pos(
                    start_pos.clone(),
                    Position::unknown(),
                ));
            }
        }

        if !args.keep {
            fs::remove_file(asm_out_file)
                .expect("Could not remove assembly file");
            fs::remove_file(o_out_file)
                .expect("Could not remove object file");
        }
    }

    Ok(())
}

fn print_usage() {
    println!("{}", get_args_cmd().render_usage());
}

fn main() -> std::io::Result<()> {
    let args = get_cli_args();

    if !args.input.is_empty() && !args.eval.is_empty() {
        arg_error("Cannot specify both 'input' and 'eval' options\n")
            .print_stderr();
        return Ok(());
    }
    if args.exec_mode != ExecMode::Lib
        && !Path::new(&args.std_path).exists()
    {
        io_error(format!(
            "STD file '{}' does not exist or is not accessible\n",
            args.std_path
        ))
        .print_stderr();
        return Ok(());
    }

    if !args.eval.is_empty() {
        let args_ = args.clone();
        let res = compile_and_assemble(
            args.eval.clone(),
            "CLI".to_owned(),
            &args_,
        );
        if res.is_err() {
            res.err().unwrap().pretty_print_stderr(
                args.eval,
                "CLI".to_string(),
            )
        }
        return Ok(());
    }

    if !args.input.is_empty() {
        if !Path::new(args.input.as_str()).exists() {
            io_error(format!(
                "Path '{}' doesn't exist\n",
                args.input
            ))
            .print_stderr();
            return Ok(());
        }

        let mut input_file =
            File::open(args.input.clone())?;
        let mut input = String::new();
        input_file.read_to_string(&mut input)?;

        let res = compile_and_assemble(
            input.clone(),
            args.input.clone(),
            &args,
        );
        if res.is_err() {
            res.err().unwrap().pretty_print_stderr(
                input,
                args.input.clone(),
            )
        }
        return Ok(());
    }

    print_usage();
    Ok(())
}
