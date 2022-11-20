extern crate core;

use std::env;
use std::fs::File;
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

fn execute (input: String) -> String {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex();

    let mut parser = Parser::new(tokens);
    let mut ast = parser.parse();

    if ast.error.is_some() {
        return ast.error.unwrap().str();
    }

    let mut ctx = Context::new();

    post_process(ast.node.unwrap().asm(&mut ctx))
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

    let assembly = execute(input.to_owned());

    let mut file = File::create(output_file)?;
    file.write_all(assembly.as_bytes())?;

    Ok(())
}
