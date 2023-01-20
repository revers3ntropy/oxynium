use crate::args::{Args, ExecMode};
use crate::context::Context;
use crate::error::{io_error, Error};
use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::perf;
use crate::position::Position;
use crate::post_process::format_asm::post_process;
use crate::util::MutRc;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use std::time::Instant;

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
