use crate::error::{arg_error, Error};
use crate::target::Target;
use crate::util::string_to_static_str;
use clap::parser::ValuesRef;
use clap::{arg, ArgMatches, Command};
use std::env;
use std::io::Write;

extern crate shellexpand;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum ExecMode {
    Bin = 0,
    Lib = 1,
}

fn exec_mode_from_int(i: u8) -> Option<ExecMode> {
    match i {
        0 => Some(ExecMode::Bin),
        1 => Some(ExecMode::Lib),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Args {
    pub input: String,
    pub out: String,
    pub eval: String,
    pub exec_mode: ExecMode,
    pub std_path: &'static str,
    pub keep: bool,
    pub optimise: u8,
    pub enable: Vec<String>,
    pub disable: Vec<String>,
    pub comp_debug: bool,
    pub allow_overrides: bool,
    pub stop_after_asm: bool,
    pub version: bool,
    pub target: Target,
}

pub fn get_int_cli_arg(m: &ArgMatches, name: &str, default: u8) -> u8 {
    let res = m
        .get_one::<String>(name)
        .unwrap_or(&String::from(default.to_string()))
        .to_string()
        .parse::<u8>();

    if res.is_err() {
        let mut e = std::io::stderr();
        let _ = e.write(
            format!("warning: arg '{name}' must be an integer, using default value {default}")
                .as_bytes(),
        );
    }

    res.unwrap_or(default)
}

pub fn get_args_cmd() -> Command {
    Command::new("oxy").args(&[
        arg!(-o --out             [FILE]   "File name of output"),
        arg!(-t --target          [TARGET] "Specify compilation target"),
        arg!(-e --eval            [EXPR]   "Compiles and prints a single expression"),
        arg!(-s --std             [PATH]   "Path to STD assembly file"),
        arg!(-k --keep                     "Keep output assembly and object files"),
        arg!(-x --exec_mode       [INT]    "Exec mode"),
        arg!(-O --optimise        [INT]    "Optimisation level"),
        arg!(-E --enable          [ID]...  "Enable specific optimisations"),
        arg!(-D --disable         [ID]...  "Disable specific optimisations"),
        arg!(-d --comp_debug               "For debugging the compiler"),
        arg!(   --allow_overrides          "Allows symbols to be redeclared"),
        arg!(   --stop_after_asm           "Stop after emitting assembly"),
        arg!(-v --version                  "Log version"),
        arg!(                     [path]   "Path to input file"),
    ])
}

pub fn get_cli_args() -> Args {
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
        out: m
            .get_one::<String>("out")
            .unwrap_or(&String::from("oxy-out"))
            .to_string(),
        input: m
            .get_one::<String>("path")
            .unwrap_or(&String::from(""))
            .to_string(),
        eval: m
            .get_one::<String>("eval")
            .unwrap_or(&String::from(""))
            .to_string(),
        std_path: unsafe {
            string_to_static_str(
                m.get_one::<String>("std")
                    .unwrap_or(
                        &shellexpand::tilde(&String::from("~/.oxynium/std.macos.asm")).to_string(),
                    )
                    .to_string(),
            )
        },
        exec_mode: exec_mode_from_int(get_int_cli_arg(&m, "exec_mode", 0)).unwrap_or(ExecMode::Bin),
        optimise: get_int_cli_arg(&m, "optimise", 1),
        keep: m.get_flag("keep"),
        enable: m
            .get_many::<String>("enable")
            .unwrap_or(ValuesRef::default())
            .into_iter()
            .map(|a| a.to_string())
            .collect(),
        disable: m
            .get_many::<String>("disable")
            .unwrap_or(ValuesRef::default())
            .into_iter()
            .map(|a| a.to_string())
            .collect(),
        comp_debug: m.get_flag("comp_debug"),
        allow_overrides: m.get_flag("allow_overrides"),
        stop_after_asm: m.get_flag("stop_after_asm"),
        version: m.get_flag("version"),
        target: Target::from_str(
            m.get_one::<String>("target")
                .unwrap_or(&String::from(""))
                .to_string(),
        ),
    }
}

pub fn check_args(args: &Args) -> Result<(), Error> {
    if !args.input.is_empty() && !args.eval.is_empty() {
        return Err(arg_error(
            "Cannot specify both 'input' and 'eval' options\n",
        ));
    }
    Ok(())
}
