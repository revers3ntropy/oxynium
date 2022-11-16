use std::env;
use std::fs::File;
use std::io::prelude::*;
use clap::{ arg, Command };
use crate::ast::Node;
use crate::context::Context;

mod lexer;
mod parser;
mod token;
mod ast;
mod context;

use crate::lexer::Lexer;
use crate::parser::Parser;

fn execute (input: &String) -> String {
    let lexer = Lexer::new(input);
    let tokens = lexer.lex();

    let parser = Parser::new(tokens);
    let ast = parser.parse();

    let mut ctx = Context::new();

    ast.asm(&mut ctx)
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

    let assembly = execute(input);

    let mut file = File::create(output_file)?;
    file.write_all(assembly.as_bytes())?;

    Ok(())
}
