extern crate core;

use std::env;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::prelude::*;
use clap::{ arg, Command };

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

fn execute (input: String, file_name: String) -> CompileResults {
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

    let asm = post_process(ast.node.unwrap().asm(&mut ctx));
    CompileResults {
        error: None,
        asm: Some(asm)
    }
}

fn main() -> std::io::Result<()> {

    let cmd = Command::new("eval")
        .args(&[
            arg!(-o --out <FILE> "Where to put assembly output"),
            arg!([input] "Input code to evaluate"),
        ]);
    let args: Vec<String> = env::args().collect();
    let matches = cmd.try_get_matches_from(args);
    let m = matches.expect("Failed to parse arguments");

    let default_output_file = &"out.asm".to_owned();
    let output_file = m.get_one::<String>("out").unwrap_or(default_output_file);

    let default_input = &"1".to_owned();
    let input = m.get_one::<String>("input").unwrap_or(default_input);

    let CompileResults { error, asm } = execute(input.to_owned(), "CLI".to_owned());

    if error.is_some() {
        let mut e = std::io::stderr();
        let _ = e.write(format!("{}\n", error.unwrap()).as_bytes());
        return Ok(());
    }

    let mut file = File::create(output_file)?;
    file.write_all(asm.unwrap().as_bytes())?;

    Ok(())
}
