use crate::Args;
use regex::Regex;

// const REGISTERS: [&str; 16] = [
//     "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
//     "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
// ];

const REGISTERS_NO_STACK: [&str; 14] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];

fn o1(
    name: &str,
    asm: Vec<String>,
    args: &Args,
    cb: fn(Vec<String>) -> Vec<String>,
) -> Vec<String> {
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

    fn valid_push_str(line: &String) -> (bool, String) {
        let (push, register) = line.split_once(" ").unwrap_or(("", ""));
        (
            push == "push"
                && (REGISTERS_NO_STACK.contains(&register) || register.starts_with("qword [")),
            register.to_string(),
        )
    }

    fn valid_pop_str(line: &String) -> (bool, String) {
        let (push, register) = line.split_once(" ").unwrap_or(("", ""));
        (
            push == "pop"
                && (REGISTERS_NO_STACK.contains(&register) || register.starts_with("qword [")),
            register.to_string(),
        )
    }

    let mut i: usize = 0;
    'outer: loop {
        if i >= ast.len() {
            break;
        }
        if i + 1 >= ast.len() {
            res.push(ast[i].clone());
            break;
        }
        if !valid_push_str(&ast[i].clone()).0 {
            res.push(ast[i].clone());
            i += 1;
            continue;
        }

        // get multiple pushes in a row, store register names
        let mut pushes = vec![];
        loop {
            if i >= ast.len() {
                break;
            }
            let line = ast[i].clone();

            let (valid_push_line, push_register) = valid_push_str(&line);
            if valid_push_line {
                pushes.push(push_register);
            } else {
                break;
            }
            i += 1;
        }

        let mut pops = vec![];
        loop {
            if i >= ast.len() {
                break;
            }
            let line = ast[i].clone();

            let (valid_pop_line, pop_register) = valid_pop_str(&line);
            if valid_pop_line {
                pops.push(pop_register);
            } else {
                break;
            }
            i += 1;
        }

        while pushes.len() > pops.len() {
            res.push(format!("push {}", pushes[0]));
            pushes.remove(0);
        }

        let mut used_registers = Vec::new();
        let mut past_last_push_idx = res.len();

        while pushes.len() > 0 {
            let push_reg = pushes[pushes.len() - 1].clone();
            let pop_reg = pops[0].clone();
            if used_registers.contains(&push_reg.clone()) {
                // this is a very annoying case where we are doing a swap,
                // and without extensive analysis of surrounding code cannot
                // find a safe alternative to pushing and popping.
                // If we find this state, then stop trying to optimise and
                // just 'push' 'pop' around whatever has worked so far
                // (as we work outwards)

                // add in remaining pushes
                for push in pushes.clone() {
                    res.insert(past_last_push_idx, format!("push {push}"));
                    past_last_push_idx += 1;
                }
                for pop in pops.clone() {
                    res.push(format!("pop {pop}"));
                }

                continue 'outer;
            }
            used_registers.push(pop_reg.clone());
            if used_registers.contains(&push_reg) {
                let index = used_registers.iter().position(|x| *x == push_reg).unwrap();
                used_registers.remove(index);
            }
            res.push(format!("mov {}, {}", pop_reg.clone(), push_reg.clone()));
            pushes.pop();
            pops.remove(0);
        }

        while pops.len() > 0 {
            res.push(format!("pop {}", pops[0]));
            pops.remove(0);
        }
    }

    res
}

fn mov_same(ast: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();

    let re = Regex::new(r"mov (.+), ?(.+)").unwrap();

    let mut i: i64 = -1;
    loop {
        i += 1;
        if i >= ast.len() as i64 {
            break;
        }
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

#[cfg(test)]
mod tests {
    use crate::strings_vec;

    #[test]
    fn push_pull_duplication() {
        assert_eq!(
            super::push_pull_duplication(strings_vec!["push r11", "pop r12",]),
            strings_vec!["mov r12, r11",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push r10", "push r11", "pop r12", "pop r13",
            ]),
            strings_vec!["mov r12, r11", "mov r13, r10",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec!["push r10", "push r11", "pop r12",]),
            strings_vec!["push r10", "mov r12, r11",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push r10",
                "push r11",
                "push r17",
                "mov r11, rax",
                "inc r15",
                "pop r12",
            ]),
            strings_vec![
                "push r10",
                "push r11",
                "push r17",
                "mov r11, rax",
                "inc r15",
                "pop r12",
            ]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push rax", "push rbx", "pop rbx", "pop rax",
            ]),
            strings_vec!["mov rbx, rbx", "mov rax, rax",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push rbx", "push rax", "pop rbx", "pop rax",
            ]),
            strings_vec!["push rbx", "mov rbx, rax", "pop rax",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push r12", "push rbx", "push rax", "pop rbx", "pop rax", "pop r12",
            ]),
            strings_vec!["push r12", "push rbx", "mov rbx, rax", "pop rax", "pop r12",]
        );

        assert_eq!(
            super::push_pull_duplication(strings_vec![
                "push rbx", "push rax", "pop rbx", "pop rcx",
            ]),
            strings_vec!["push rbx", "mov rbx, rax", "pop rcx",]
        );
    }

    #[test]
    fn mov_same() {
        let mut res = super::mov_same(strings_vec!["mov r11, r11",]);
        assert_eq!(res, Vec::<String>::new());

        res = super::mov_same(strings_vec!["mov r11, r12",]);
        assert_eq!(res, strings_vec!["mov r11, r12",]);

        res = super::mov_same(strings_vec!["mov r11, r12", "mov r11, r11",]);
        assert_eq!(res, strings_vec!["mov r11, r12",]);
    }
}
