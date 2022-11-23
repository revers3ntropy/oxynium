extern crate core;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{arg, Command};

mod parse;
mod ast;
mod context;
mod post_process;
mod error;
mod position;

use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::post_process::post_process;
use crate::context::Context;

struct CompileResults {
    error: Option<String>,
    asm: Option<String>
}

fn execute (input: String, file_name: String, exec_mode: bool) -> CompileResults {
    let mut lexer = Lexer::new(input, file_name);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    if ast.error.is_some() {
        return CompileResults {
            error: Some(ast.error.unwrap().str()),
            asm: None
        };
    }

    let mut ctx = Context::new();
    ctx.exec_mode = exec_mode;

    let compile_res = ast.node.unwrap().asm(&mut ctx);
    if compile_res.is_err() {
        return CompileResults {
            error: Some(compile_res.err().unwrap().str()),
            asm: None
        };
    }

    let asm = post_process(compile_res.unwrap());
    CompileResults {
        error: None,
        asm: Some(asm)
    }
}

#[derive(Debug)]
struct Args {
    input: String,
    out: String,
    eval: String,
    exec_mode: bool
}

fn get_cli_args () -> Args {
    let cmd = Command::new("res")
        .args(&[
            arg!(-o --out <FILE> "Where to put assembly output"),
            arg!(-e --eval <EXPR> "Compiles and prints a single expression"),
            arg!(-x --exec "Should print final expression"),
            arg!([input] "Input code to evaluate"),
        ]);
    let args: Vec<String> = env::args().collect();
    let matches = cmd.try_get_matches_from(args);
    let m = matches.expect("Failed to parse arguments");

    Args {
        out: m.get_one::<String>("out").unwrap_or(&String::from("out.asm")).to_string(),
        input: m.get_one::<String>("input").unwrap_or(&String::from("")).to_string(),
        eval: m.get_one::<String>("eval").unwrap_or(&String::from("")).to_string(),
        exec_mode: m.get_one::<bool>("exec").unwrap_or(&false).to_owned()
    }
}

fn print_usage () {
    println!("Usage: res [options] [input]");
    println!("Options:");
    println!("  -o, --out  <FILE>  Where to put NASM output");
    println!("  -e, --eval <EXP>   Compiles and prints a single expression");
}

fn main() -> std::io::Result<()> {

    let mut e = std::io::stderr();

    let args = get_cli_args();


    if !args.input.is_empty() && !args.eval.is_empty() {
        let _ = e.write("Cannot specify both 'input' and 'eval' options\n".as_bytes());
        return Ok(());
    }

    if !args.eval.is_empty() {
        let res = execute(args.eval, "CLI".to_owned(), true);

        if res.error.is_some() {
            let _ = e.write(format!("{}\n", res.error.unwrap()).as_bytes());
            return Ok(());
        }

        let mut file = File::create(args.out)?;
        file.write_all(res.asm.unwrap().as_bytes())?;

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

        let res = execute(input, args.input.clone(), args.exec_mode);

        if res.error.is_some() {
            let _ = e.write(format!("{}\n", res.error.unwrap()).as_bytes());
            return Ok(());
        }

        let mut file = File::create(args.out)?;
        file.write_all(res.asm.unwrap().as_bytes())?;

        return Ok(());
    }

    print_usage();
    Ok(())
}
