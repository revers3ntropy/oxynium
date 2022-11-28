extern crate core;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use clap::{arg, ArgMatches, Command};
use crate::ast::types::Type;

mod parse;
mod ast;
mod context;
mod error;
mod position;
mod post_process;

use crate::parse::lexer::{Lexer, token_type_str};
use crate::parse::parser::Parser;
use crate::context::{Context, Symbol};
use std::process::Command as Exec;
use crate::post_process::format_asm::post_process;

const STD_DOXY: &str = include_str!("../std/std.doxy");

#[derive(Debug)]
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
        type_: Box::new(Type {
            id: 0,
            name: "Int".to_string(),
            children: vec![]
        })
    });
    ctx.declare(Symbol {
        name: "Bool".to_string(),
        data: None,
        constant: true,
        type_: Box::new(Type {
            id: 1,
            name: "Bool".to_string(),
            children: vec![]
        })
    });
    ctx.declare(Symbol {
        name: "Str".to_string(),
        data: None,
        constant: true,
        type_: Box::new(Type {
            id: 2,
            name: "Str".to_string(),
            children: vec![]
        })
    });
    ctx.declare(Symbol {
        name: "Void".to_string(),
        data: None,
        constant: true,
        type_: Box::new(Type {
            id: 3,
            name: "Void".to_string(),
            children: vec![]
        })
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

    CompileResults {
        error: None,
        asm: None,
        ctx
    }
}

fn compile (input: String, file_name: String, exec_mode: u8, std_path: String) -> CompileResults {
    let ctx_res = setup_ctx_with_doxy(Context::new());
    if ctx_res.error.is_some() { return ctx_res; }
    let mut ctx = ctx_res.ctx;
    ctx.std_asm_path = std_path;

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

fn compile_and_assemble(input: String, file_name: String, exec_mode: u8, std_path: String) -> Result<(), String> {
    let compile_res = compile(input, file_name, exec_mode, std_path);

    if let Some(e) = compile_res.error {
        return Err(e);
    }

    let file = File::create("oxy-out.asm");
    if file.is_err() { return Err("Could not create temporary assembly file".to_string()); }
    file.unwrap().write_all(compile_res.asm.unwrap().as_bytes())
        .expect("Could not write assembly output");

    let nasm_out =Exec::new("nasm")
        .arg("-f")
        .arg("elf64")
        .arg("oxy-out.asm")
        .output()
        .expect("Could not assemble");
    if !nasm_out.status.success() {
        return Err(String::from_utf8(nasm_out.stderr).unwrap());
    }

    let ls_out = Exec::new("ld")
        .arg("-s")
        .arg("-o")
        .arg("out")
        .arg("oxy-out.o")
        .output()
        .expect("Could not assemble");
    if !ls_out.status.success() {
        return Err(String::from_utf8(ls_out.stderr).unwrap());
    }

    Ok(())
}

#[derive(Debug)]
struct Args {
    input: String,
    out: String,
    eval: String,
    exec_mode: u8,
    std_path: String
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
            arg!(-s --std [PATH] "Path to STD assembly file"),
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
        std_path: m.get_one::<String>("std")
            .unwrap_or(&String::from("/usr/local/bin/oxy-std.asm")).to_string(),
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

    if args.exec_mode != 1 && !Path::new(&args.std_path).exists() {
        let _ = e.write(format!("STD file '{}' does not exist or is not accessible\n", args.std_path).as_bytes());
        return Ok(());
    }

    if !args.eval.is_empty() {
        let res = compile_and_assemble(
            args.eval,
            "CLI".to_owned(),
            args.exec_mode,
            args.std_path
        );
        if res.is_err() {
            let _ = e.write(format!("{}\n", res.err().unwrap()).as_bytes());
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
            "CLI".to_owned(),
            args.exec_mode,
            args.std_path
        );
        if res.is_err() {
            let _ = e.write(format!("{}\n", res.err().unwrap()).as_bytes());
        }
        return Ok(());
    }

    print_usage();
    Ok(())
}
