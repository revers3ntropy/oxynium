use clap::parser::ValuesRef;
use std::{env};
use std::io::Write;
use clap::{arg, ArgMatches, Command};

#[derive(Debug, Clone)]
pub struct Args {
    pub input: String,
    pub out: String,
    pub eval: String,
    pub exec_mode: u8,
    pub std_path: String,
    pub keep: bool,
    pub optimise: u8,
    pub enable: Vec<String>,
    pub disable: Vec<String>
}

pub fn get_int_cli_arg (m: &ArgMatches, name: &str, default: u8) -> u8 {
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

pub fn get_args_cmd() -> Command {
    Command::new("res")
        .args(&[
            arg!(-o --out       [FILE]  "File name of output"),
            arg!(-e --eval      [EXPR]  "Compiles and prints a single expression"),
            arg!(-s --std       [PATH]  "Path to STD assembly file"),
            arg!(-k --keep              "Keep output assembly and object files"),
            arg!(-x --exec_mode [INT]   "Exec mode"),
            arg!(-O --optimise  [INT]   "Optimisation level"),
            arg!(-E --enable    [ID]... "Enable specific optimisations"),
            arg!(-D --disable   [ID]... "Disable specific optimisations"),
            arg!(               [input] "Input code to evaluate"),
        ])
}

pub fn get_cli_args () -> Args {
    let mut e = std::io::stderr();

    let cmd = get_args_cmd();

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
        exec_mode: get_int_cli_arg(&m, "exec_mode", 0),
        optimise: get_int_cli_arg(&m, "optimise", 1),
        keep: m.get_flag("keep"),
        enable: m.get_many::<String>("enable").unwrap_or(ValuesRef::default())
            .into_iter().map(|a| a.to_string()).collect(),
        disable: m.get_many::<String>("disable").unwrap_or(ValuesRef::default())
            .into_iter().map(|a| a.to_string()).collect()
    }
}