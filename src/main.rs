use crate::args::{check_args, get_args_cmd, get_cli_args, Args};
use crate::compile::compile_and_assemble;
use crate::error::{io_error, ErrorSource};
use crate::util::read_file;
use std::path::Path;

mod args;
mod ast;
mod backend;
mod compile;
mod context;
mod error;
mod log;
mod oxy_std;
mod parse;
mod position;
mod post_process;
mod symbols;
mod target;
mod types;
mod util;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn cli_exec(args: &Args) {
    let args_ = args.clone();
    let res = compile_and_assemble(args.eval.clone(), "CLI".to_owned(), &args_);
    if let Some(mut err) = res.err() {
        err.try_set_source(ErrorSource {
            file_name: "CLI".to_string(),
            source: args.eval.clone(),
        });
        err.pretty_print_stderr();
    }
}

fn import_exec(args: &Args) {
    if !Path::new(args.input.as_str()).exists() {
        io_error(format!("Path '{}' doesn't exist\n", args.input)).print_stderr();
        return;
    }

    let read_result = read_file(args.input.as_str());
    if read_result.is_err() {
        read_result.err().unwrap().print_stderr();
        return;
    }

    let source_code = read_result.unwrap();
    // replace tabs with 4 spaces so error messages are more accurate
    let source_code = source_code.replace("\t", "    ");
    let res = compile_and_assemble(source_code.clone(), args.input.clone(), &args);
    if let Some(mut err) = res.err() {
        err.try_set_source(ErrorSource {
            file_name: args.input.clone(),
            source: source_code,
        });
        err.pretty_print_stderr();
    }
}

fn print_usage() {
    println!("{}", get_args_cmd().render_usage());
}

fn main() {
    let args = get_cli_args();

    if let Err(e) = check_args(&args) {
        e.print_stderr();
    }

    if args.version {
        println!("Oxynium v{}", VERSION);
        return;
    }

    if !args.eval.is_empty() {
        cli_exec(&args);
        return;
    }

    if !args.input.is_empty() {
        import_exec(&args);
        return;
    }

    print_usage()
}
