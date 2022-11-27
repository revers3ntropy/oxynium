extern crate core;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{arg, ArgMatches, Command};
use crate::ast::types::built_in::{BOOL, INT};

mod parse;
mod ast;
mod context;
mod error;
mod position;
mod post_process;

use crate::parse::lexer::Lexer;
use crate::parse::parser::Parser;
use crate::context::{Context, Symbol};
use crate::post_process::format_asm::post_process;

const STD_DOXY: &str = include_str!("../std/std.doxy");

struct CompileResults {
    error: Option<String>,
    asm: Option<String>,
    ctx: Context
}

fn setup_ctx_with_doxy(mut ctx: Context) -> CompileResults {

    // declare the built in types
    ctx.declare(Symbol {
        name: "Int".to_string(),
        data: None,
        constant: true,
        type_: Box::new(INT)
    });
    ctx.declare(Symbol {
        name: "Bool".to_string(),
        data: None,
        constant: true,
        type_: Box::new(BOOL)
    });

    let mut lexer = Lexer::new(STD_DOXY.to_owned(), "std.doxy".to_owned());
    let tokens = lexer.lex();
    if tokens.is_err() {
        return CompileResults {
            error: Some(tokens.err().unwrap().str()),
            asm: None,
            ctx
        };
    }

    let mut parser = Parser::new(tokens.unwrap());
    let ast = parser.parse();

    if ast.error.is_some() {
        return CompileResults {
            error: Some(ast.error.unwrap().str()),
            asm: None,
            ctx
        };
    }

    let mut node = ast.node.unwrap();
    let type_check_res = node.type_check(&mut ctx);
    if type_check_res.is_err() {
        return CompileResults {
            error: Some(type_check_res.err().unwrap().str()),
            asm: None,
            ctx
        };
    }
    let asm_error = node.asm(&mut ctx);
    if asm_error.is_err() {
        return CompileResults {
            error: Some(asm_error.err().unwrap().str()),
            asm: None,
            ctx
        };
    }

    println!("{:?}", ctx.get_all_ids());

    CompileResults {
        error: None,
        asm: None,
        ctx
    }
}

fn execute (input: String, file_name: String, exec_mode: u8) -> CompileResults {
    let ctx_res = setup_ctx_with_doxy(Context::new());
    if ctx_res.error.is_some() { return ctx_res; }
    let mut ctx = ctx_res.ctx;

    let mut lexer = Lexer::new(input.clone(), file_name);
    let tokens = lexer.lex();
    if tokens.is_err() {
        return CompileResults {
            error: Some(tokens.err().unwrap().str()),
            asm: None,
            ctx
        };
    }

    let mut parser = Parser::new(tokens.unwrap());
    let ast = parser.parse();

    if ast.error.is_some() {
        return CompileResults {
            error: Some(ast.error.unwrap().str()),
            asm: None,
            ctx
        };
    }

    ctx.exec_mode = exec_mode;

    let mut root_node = ast.node.unwrap();
    let type_check_res = root_node.type_check(&mut ctx);
    if type_check_res.is_err() {
        return CompileResults {
            error: Some(type_check_res.err().unwrap().str()),
            asm: None,
            ctx
        };
    }
    let compile_res = root_node.asm(&mut ctx);
    if compile_res.is_err() {
        return CompileResults {
            error: Some(compile_res.err().unwrap().str()),
            asm: None,
            ctx
        };
    }

    let asm = post_process(compile_res.unwrap());
    CompileResults {
        error: None,
        asm: Some(asm),
        ctx
    }
}

#[derive(Debug)]
struct Args {
    input: String,
    out: String,
    eval: String,
    exec_mode: u8
}

fn get_int_cli_arg (m: ArgMatches, name: &str, default: u8) -> u8 {
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
            arg!(-o --out [FILE] "Where to put assembly output"),
            arg!(-e --eval [EXPR] "Compiles and prints a single expression"),
            arg!(-x --exec_mode [INT] "Exec mode"),
            arg!([input] "Input code to evaluate"),
        ]);
    let args: Vec<String> = env::args().collect();
    let matches = cmd.try_get_matches_from(args);
    if matches.is_err() {
        let _ = e.write(format!("{}", matches.err().unwrap()).as_bytes());
        std::process::exit(1);
    }
    let m = matches.expect("Failed to parse arguments");

    Args {
        out: m.get_one::<String>("out").unwrap_or(&String::from("out.asm")).to_string(),
        input: m.get_one::<String>("input").unwrap_or(&String::from("")).to_string(),
        eval: m.get_one::<String>("eval").unwrap_or(&String::from("")).to_string(),
        exec_mode: get_int_cli_arg(m, "exec_mode", 0)
    }
}

fn print_usage () {
    println!("Usage: res [options] [input]");
    println!("Options:");
    println!("  -o, --out  <FILE>  Where to put assembly output");
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
        let res = execute(args.eval, "CLI".to_owned(), 1);

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
