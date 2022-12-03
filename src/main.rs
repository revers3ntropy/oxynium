extern crate core;

mod parse;
mod ast;
mod context;
mod error;
mod position;
mod post_process;

use std::{env, fs};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{arg, ArgMatches, Command};
use crate::ast::types::Type;
use crate::parse::lexer::{Lexer};
use crate::parse::parser::Parser;
use crate::context::{Context, SymbolDec, SymbolDef};
use std::process::Command as Exec;
use crate::error::{Error, io_error};
use crate::post_process::format_asm::post_process;

const STD_DOXY: &str = include_str!("../std/std.doxy");


fn setup_ctx_with_doxy(mut ctx: Context) -> Result<Context, Error> {
    // declare the built in types
    ctx.declare(SymbolDec {
        name: "Int".to_string(),
        is_constant: true,
        is_type: true,
        type_: Box::new(Type {
            id: 0,
            name: "Int".to_string(),
            children: vec![]
        })
    })?;
    ctx.declare(SymbolDec {
        name: "Bool".to_string(),
        is_constant: true,
        is_type: true,
        type_: Box::new(Type {
            id: 1,
            name: "Bool".to_string(),
            children: vec![]
        })
    })?;
    ctx.declare(SymbolDec {
        name: "Str".to_string(),
        is_constant: true,
        is_type: true,
        type_: Box::new(Type {
            id: 2,
            name: "Str".to_string(),
            children: vec![]
        })
    })?;
    ctx.declare(SymbolDec {
        name: "Void".to_string(),
        is_constant: true,
        is_type: true,
        type_: Box::new(Type {
            id: 3,
            name: "Void".to_string(),
            children: vec![]
        })
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

    let mut node = ast.node.unwrap();
    let type_check_res = node.type_check(&mut ctx);
    if type_check_res.is_err() {
        return Err(type_check_res.err().unwrap());
    }
    let asm_error = node.asm(&mut ctx);
    if asm_error.is_err() {
        return Err(asm_error.err().unwrap());
    }

    ctx.define(SymbolDef {
        name: "true".to_string(),
        data: Some("dq 1".to_string()),
        text: None,
        is_local: false
    }, false)?;
    ctx.define(SymbolDef {
        name: "false".to_string(),
        data: Some("dq 0".to_string()),
        text: None,
        is_local: false
    }, false)?;

    Ok(ctx)
}

fn compile (input: String, file_name: String, args: &Args) -> Result<(String, Context), Error> {
    let mut ctx = setup_ctx_with_doxy(Context::new(None))?;
    ctx.std_asm_path = args.std_path.clone();
    ctx.exec_mode = args.exec_mode;
    ctx.allow_overrides = args.allow_overrides;


    let mut lexer = Lexer::new(input.clone(), file_name);
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    if ast.error.is_some() {
        return Err(ast.error.unwrap());
    }

    let mut root_node = ast.node.unwrap();
    root_node.type_check(&mut ctx)?;
    let compile_res = root_node.asm(&mut ctx)?;

    let asm = post_process(compile_res);
    Ok((asm, ctx))
}

fn compile_and_assemble(input: String, file_name: String, args: &Args) -> Result<(), Error> {
    let compile_res = compile(input, file_name, args)?;

    let asm_out_file = format!("{}.asm", args.out);
    let o_out_file = format!("{}.o", args.out);

    let file = File::create(asm_out_file.clone());
    if file.is_err() {
        return Err(io_error(format!("Could not create assembly ('{asm_out_file}') file")));
    }
    file.unwrap().write_all(compile_res.0.as_bytes())
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

#[derive(Debug, Clone)]
struct Args {
    input: String,
    out: String,
    eval: String,
    exec_mode: u8,
    std_path: String,
    allow_overrides: bool,
    keep: bool
}

fn get_int_cli_arg (m: &ArgMatches, name: &str, default: u8) -> u8 {
    let res = m.get_one::<String>(name)
        .unwrap_or(&String::from(default.to_string()))
        .to_string()
        .parse::<u8>();

    if res.is_err() {
        let mut e = std::io::stderr();
        let _ = e.write(format!(
            "warning: arg '{name}' must be an integer, using default value {default}"
        ).as_bytes());
    }

    res.unwrap_or(default)
}

fn get_cli_args () -> Args {
    let mut e = std::io::stderr();

    let cmd = Command::new("res")
        .args(&[
            arg!(-o --out       [FILE] "Where to put assembly output"),
            arg!(-e --eval      [EXPR] "Compiles and prints a single expression"),
            arg!(-s --std       [PATH] "Path to STD assembly file"),
            arg!(-k --keep             "Keep output assembly and object files"),
            arg!(   --allow_overrides  "Ignore re-declarations (very unsafe, used to compile std)"),
            arg!(-x --exec_mode [INT]  "Exec mode"),
            arg!(   [input]            "Input code to evaluate"),
        ]);
    let args: Vec<String> = env::args().collect();
    let matches = cmd.try_get_matches_from(args);
    if matches.is_err() {
        let _ = e.write(format!("{}", matches.err().unwrap()).as_bytes());
        std::process::exit(1);
    }
    let m = matches.expect("Failed to parse arguments");

    Args {
        out: m.get_one::<String>("out").unwrap_or(&String::from("oxy-out")).to_string(),
        input: m.get_one::<String>("input").unwrap_or(&String::from("")).to_string(),
        eval: m.get_one::<String>("eval").unwrap_or(&String::from("")).to_string(),
        std_path: m.get_one::<String>("std")
            .unwrap_or(&String::from("/usr/local/bin/oxy-std.asm")).to_string(),
        allow_overrides: m.get_flag("allow_overrides"),
        exec_mode: get_int_cli_arg(&m, "exec_mode", 0),
        keep: m.get_flag("keep")
    }
}

fn print_usage () {
    println!("Usage: res [options] [input]");
    println!("Options:");
    println!("  -o, --out  <FILE>  Where to put assembly output");
    println!("  -e, --eval <EXP>   Compiles and prints a single expression");
    println!("  -s, --std  <PATH>  Path to STD assembly file");
    println!("  -x, --exec_mode <INT>  Exec mode");
}

fn main() -> std::io::Result<()> {
    let mut e = std::io::stderr();

    let args = get_cli_args();

    if !args.input.is_empty() && !args.eval.is_empty() {
        let _ = e.write("Cannot specify both 'input' and 'eval' options\n".as_bytes());
        return Ok(());
    }
    if args.exec_mode != 1 && !Path::new(&args.std_path).exists() {
        let _ = e.write(format!("STD file '{}' does not exist or is not accessible\n", args.std_path).as_bytes());
        return Ok(());
    }

    if !args.eval.is_empty() {
        let args_ = args.clone();
        let res = compile_and_assemble(
            args.eval,
            "CLI".to_owned(),
            &args_
        );
        if res.is_err() {
            let _ = e.write(format!("{}\n", res.err().unwrap().str()).as_bytes());
        }
        return Ok(());
    }

    if !args.input.is_empty() {
        if !Path::new(args.input.as_str()).exists() {
            let _ = e.write(format!("Path '{}' doesn't exist\n", args.input).as_bytes());
            return Ok(());
        }

        let mut input_file = File::open(args.input.clone())?;
        let mut input = String::new();
        input_file.read_to_string(&mut input)?;

        let res = compile_and_assemble(
            input,
            args.input.clone(),
            &args
        );
        if res.is_err() {
            let _ = e.write(format!("{}\n", res.err().unwrap().str()).as_bytes());
        }
        return Ok(());
    }

    print_usage();
    Ok(())
}
