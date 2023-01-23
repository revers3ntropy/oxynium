use crate::args::{Args, ExecMode};
use crate::ast::exec_root::ExecRootNode;
use crate::ast::AstNode;
use crate::context::Context;
use crate::error::{io_error, Error, ErrorSource};
use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::perf;
use crate::position::Position;
use crate::post_process::format_asm::post_process;
use crate::util::{string_to_static_str, MutRc};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use std::{env, fs};

const STD_DOXY: &str = include_str!("../std/std.doxy");

fn setup_ctx_with_doxy(
    ctx: MutRc<Context>,
) -> Result<MutRc<Context>, Error> {
    let start = Instant::now();

    if ctx.borrow().cli_args.exec_mode == ExecMode::Lib {
        ctx.borrow_mut().set_ignoring_definitions(true);
    }

    let mut node = ExecRootNode {
        statements: generate_ast(
            &ctx.borrow().cli_args,
            STD_DOXY.to_owned(),
            "std.doxy".to_owned(),
        )?,
    };

    node.setup(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Setup STD AST");
    let start = Instant::now();

    let type_check_res = node.type_check(ctx.clone());
    if type_check_res.is_err() {
        return Err(type_check_res.err().unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Type-checked STD");
    let start = Instant::now();

    let asm_error = node.asm(ctx.clone());
    if asm_error.is_err() {
        return Err(asm_error.err().unwrap());
    }

    perf!(ctx.borrow().cli_args, start, "Compiled STD");
    let start = Instant::now();

    ctx.borrow_mut().reset();

    perf!(ctx.borrow().cli_args, start, "Reset context");

    Ok(ctx)
}

pub fn generate_ast(
    args: &Args,
    input: String,
    file_name: String,
) -> Result<MutRc<dyn AstNode>, Error> {
    let start = Instant::now();

    let mut lexer =
        Lexer::new(input.clone(), file_name, args.clone());
    let tokens = lexer.lex()?;

    perf!(args, start, "Lexed");
    let start = Instant::now();

    let mut parser = Parser::new(args.clone(), tokens);
    let ast = parser.parse();
    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    perf!(args, start, "Parsed");

    let ast = ast.node.unwrap();

    Ok(ast)
}

fn compile(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(String, MutRc<Context>), Error> {
    let ctx = Context::new(args.clone());
    let setup_ctx_res = setup_ctx_with_doxy(ctx);
    if setup_ctx_res.is_err() {
        let mut err = setup_ctx_res.err().unwrap();
        err.try_set_source(ErrorSource {
            file_name: "std.doxy".to_string(),
            source: STD_DOXY.to_string(),
        });
        return Err(err);
    }
    let ctx = setup_ctx_res.unwrap();

    ctx.borrow_mut().std_asm_path = args.std_path.clone();
    ctx.borrow_mut().exec_mode = args.exec_mode;

    // TODO: Hack to extend the lifetimes of the strings,
    //      so that they can be used in the context,
    //      which permanently leaks their memory.
    let current_dir = env::current_dir().unwrap();
    let current_dir =
        current_dir.to_str().unwrap().to_owned();
    let current_dir_leaked_str =
        string_to_static_str(current_dir);
    let current_dir = Path::new(current_dir_leaked_str);

    let file_path_leaked_str =
        string_to_static_str(file_name.clone());
    let file_dir = Path::new(file_path_leaked_str)
        .parent()
        .unwrap_or(current_dir);

    ctx.borrow_mut().set_current_dir_path(file_dir);

    let mut root_node = ExecRootNode {
        statements: generate_ast(
            &ctx.borrow().cli_args,
            input,
            file_name.clone(),
        )?,
    };

    let start = Instant::now();

    root_node.setup(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Setup AST");
    let start = Instant::now();

    root_node.type_check(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Type-checked");
    let start = Instant::now();

    let compile_res = root_node.asm(ctx.clone())?;

    perf!(ctx.borrow().cli_args, start, "Compiled");
    let start = Instant::now();

    let asm = post_process(compile_res, args);

    perf!(ctx.borrow().cli_args, start, "Post Processed");

    Ok((asm, ctx.clone()))
}

fn assemble(
    args: &Args,
    file_name: String,
    asm_out_file: String,
    o_out_file: String,
) -> Result<(), Error> {
    let start = Instant::now();

    let start_pos = Position {
        file: file_name.clone(),
        idx: 0,
        line: 0,
        col: 0,
    };

    let nasm_out = Command::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg(asm_out_file.clone().as_str())
        .arg("-o")
        .arg(o_out_file.clone().as_str())
        .output();
    if nasm_out.is_err() {
        return Err(io_error(format!(
            "Could not assemble ('{asm_out_file}') file: {}",
            nasm_out.err().unwrap()
        )));
    }
    let nasm_out = nasm_out.unwrap();
    if !nasm_out.status.success() {
        return Err(io_error(
            String::from_utf8(nasm_out.stderr).unwrap(),
        )
        .set_pos(start_pos.clone(), Position::unknown()));
    }

    perf!(args, start, "NASM");

    if args.exec_mode == ExecMode::Bin {
        let start = Instant::now();

        let ls_out = Command::new("gcc")
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
                String::from_utf8(ls_out.stderr).unwrap(),
            )
            .set_pos(
                start_pos.clone(),
                Position::unknown(),
            ));
        }

        perf!(args, start, "gcc");
    }

    let start = Instant::now();

    if !args.keep {
        fs::remove_file(asm_out_file)
            .expect("Could not remove assembly file");
        fs::remove_file(o_out_file)
            .expect("Could not remove object file");
    }

    perf!(args, start, "Clean up files");

    Ok(())
}

fn write_asm_to_file(
    args: &Args,
    asm: &[u8],
    asm_out_file: String,
) -> Result<(), Error> {
    let start = Instant::now();

    let file = File::create(asm_out_file.clone());
    if file.is_err() {
        return Err(io_error(format!(
            "Could not create assembly ('{asm_out_file}') file"
        )));
    }
    let file_write_result = file.unwrap().write_all(asm);
    if file_write_result.is_err() {
        return Err(io_error(format!(
            "Could not write to assembly ('{asm_out_file}') file"
        )));
    }

    perf!(args, start, "Write asm to file");

    Ok(())
}

pub fn compile_and_assemble(
    input: String,
    file_name: String,
    args: &Args,
) -> Result<(), Error> {
    let start = Instant::now();

    let compile_res =
        compile(input, file_name.clone(), args)?;
    let asm_out_file = format!("{}.asm", args.out);
    let o_out_file = format!("{}.o", args.out);

    write_asm_to_file(
        args,
        compile_res.0.as_bytes(),
        asm_out_file.clone(),
    )?;

    if !args.stop_after_asm {
        assemble(
            args,
            file_name,
            asm_out_file,
            o_out_file,
        )?;
    }

    perf!(args, start, "Compile and Assemble Total");

    Ok(())
}
