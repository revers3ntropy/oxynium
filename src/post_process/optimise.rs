use regex::Regex;
use crate::Args;

// const REGISTERS: [&str; 16] = [
//     "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
//     "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
// ];

const REGISTERS_NO_STACK: [&str; 14] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi",
    "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
];

fn o1(name: &str, asm: Vec<String>, args: &Args, cb: fn(Vec<String>) -> Vec<String>) -> Vec<String> {
    if args.enable.contains(&name.to_string()) {
        cb(asm)
    } else if args.disable.contains(&name.to_string()) {
        asm
    } else if args.optimise >= 1 {
        cb(asm)
    } else {
        asm
    }
}

pub fn optimise(asm: Vec<String>, args: &Args) -> Vec<String> {
    let mut asm = asm;

    asm = o1("asm-redundant-push", asm, args, push_pull_duplication);
    asm = o1("asm-redundant-mov", asm, args, mov_same);

    asm
}

fn push_pull_duplication(ast: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();

    let mut i: i64 = -1;
    loop {
        i += 1;
        if i >= ast.len() as i64 { break }

        let line = ast[i as usize].clone();

        if !line.starts_with("push") {
            res.push(line);
            continue;
        }

        let (_, register) = line.split_once(" ").unwrap();
        if !REGISTERS_NO_STACK.contains(&register) && !register.starts_with("qword [") {
            res.push(line);
            continue;
        }

        if i + 1 >= ast.len() as i64 {
            res.push(line);
            continue;
        }

        let pop_line = ast[i as usize + 1].clone();
        if !pop_line.starts_with("pop") {
            res.push(line);
            continue;
        }

        let (_, pop_register) = pop_line.split_once(" ").unwrap();
        if !REGISTERS_NO_STACK.contains(&pop_register) && !register.starts_with("qword [") {
            res.push(line);
            continue;
        }

        // replace push/pop with mov
        res.push(format!("mov {}, {}", pop_register, register));
        i += 1;
    }

    res
}

fn mov_same(ast: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();

    let re = Regex::new(r"mov (.+), ?(.+)").unwrap();

    let mut i: i64 = -1;
    loop {
        i += 1;
        if i >= ast.len() as i64 { break }
        let line = ast[i as usize].clone();

        if !line.starts_with("mov") {
            res.push(line);
            continue;
        }

        re.captures(&line).map(|captures| {
            let first = captures.get(1).unwrap().as_str();
            let second = captures.get(2).unwrap().as_str();

            if first == second {
                // remove mov
                return;
            }

            res.push(line.clone());
        });
    }

    res
}