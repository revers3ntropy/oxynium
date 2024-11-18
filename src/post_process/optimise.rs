use crate::Args;
use regex::Regex;

// const REGISTERS: [&str; 16] = [
//     "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
//     "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
// ];

const REGISTERS_NO_STACK: [&str; 14] = [
    "rax", "rbx", "rcx", "rdx", "rsi", "rdi", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15",
];

pub fn o1<T>(name: &str, args: &Args, cb: &impl Fn() -> T) -> Option<T> {
    if args.enable.contains(&name.to_string()) {
        Some(cb())
    } else if args.disable.contains(&name.to_string()) {
        None
    } else if args.optimise >= 1 {
        Some(cb())
    } else {
        None
    }
}

#[allow(dead_code)]
pub fn o2<T>(name: &str, args: &Args, cb: &impl Fn() -> T) -> Option<T> {
    if args.enable.contains(&name.to_string()) {
        Some(cb())
    } else if args.disable.contains(&name.to_string()) {
        None
    } else if args.optimise >= 2 {
        Some(cb())
    } else {
        None
    }
}

fn o1_asm(
    name: &str,
    asm: Vec<String>,
    args: &Args,
    cb: fn(Vec<String>) -> Vec<String>,
) -> Vec<String> {
    o1(name, args, &|| cb(asm.clone())).unwrap_or(asm)
}

pub fn optimise(asm: Vec<String>, args: &Args) -> Vec<String> {
    let mut asm = asm;

    asm = o1_asm("redundant-push", asm, args, redundant_push);
    asm = o1_asm("redundant-mov", asm, args, redundant_mov);
    asm = o1_asm("redundant-jmp", asm, args, redundant_jmp);

    asm
}

fn redundant_push(ast: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();

    #[inline]
    fn valid_push_str(line: &String) -> (bool, String) {
        let (push, register) = line.split_once(" ").unwrap_or(("", ""));
        (
            push == "push"
                && (REGISTERS_NO_STACK.contains(&register)
                    || register.starts_with("qword [")
                    || !register.chars().any(|c| !c.is_digit(10))),
            register.to_string(),
        )
    }

    #[inline]
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

        // remove excess pushes before push-pop pairs start
        while pushes.len() > pops.len() {
            res.push(format!("push {}", pushes[0]));
            pushes.remove(0);
        }
        
        // remove excess pops before push-pop pairs start
        // so we can analyse if there are any issues
        let excess_pop_count = pops.len() - pushes.len();
        let mut excess_pops = pops.split_off(pops.len() - excess_pop_count);

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
            // insert backwards to avoid register clobbering
            res.insert(past_last_push_idx, format!("mov {}, {}", pop_reg.clone(), push_reg.clone()));
            pushes.pop();
            pops.remove(0);
        }

        // deal with excess pops
        while excess_pops.len() > 0 {
            res.push(format!("pop {}", excess_pops[0]));
            excess_pops.remove(0);
        }
    }

    res
}

fn redundant_mov(ast: Vec<String>) -> Vec<String> {
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

fn redundant_jmp(ast: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();

    let mut i: i64 = -1;
    loop {
        i += 1;
        if i >= ast.len() as i64 {
            break;
        }
        let line = ast[i as usize].clone();

        if !line.starts_with("jmp") {
            res.push(line);
            continue;
        }
        if i + 1 >= ast.len() as i64 {
            res.push(line);
            break;
        }
        let next = ast[i as usize + 1].clone();

        if !next.ends_with(":") || !line.ends_with(&next[..next.len() - 1]) {
            res.push(line);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use crate::strings_vec;

    #[test]
    fn redundant_push() {
        assert_eq!(
            super::redundant_push(strings_vec!["push r11", "pop r12",]),
            strings_vec!["mov r12, r11",]
        );
        assert_eq!(
            super::redundant_push(strings_vec!["push qword [rax]", "pop r12",]),
            strings_vec!["mov r12, qword [rax]",]
        );

        assert_eq!(
            super::redundant_push(strings_vec!["push r10", "push r11", "pop r12", "pop r13",]),
            strings_vec!["mov r13, r10", "mov r12, r11",]
        );

        assert_eq!(
            super::redundant_push(strings_vec!["push r10", "push r11", "pop r12",]),
            strings_vec!["push r10", "mov r12, r11",]
        );

        assert_eq!(
            super::redundant_push(strings_vec![
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

        // case from Range.at_raw, which was optimised incorrectly
        // which lead to a seg fault
        assert_eq!(
            super::redundant_push(strings_vec![
                "push qword [rbp + 16]",
                "pop rax",
                "push qword [rax + 16]",
                "push qword [rbp + 24]",
                "pop rax",
                "pop rbx",
            ]),
            strings_vec![
                "mov rax, qword [rbp + 16]",
                "mov rbx, qword [rax + 16]",
                "mov rax, qword [rbp + 24]",
            ]
        );

        assert_eq!(
            super::redundant_push(strings_vec!["push rax", "push rbx", "pop rbx", "pop rax",]),
            strings_vec!["mov rax, rax", "mov rbx, rbx",]
        );

        assert_eq!(
            super::redundant_push(strings_vec!["push rbx", "push rax", "pop rbx", "pop rax",]),
            strings_vec!["push rbx", "mov rbx, rax", "pop rax",]
        );

        assert_eq!(
            super::redundant_push(strings_vec![
                "push r12", "push rbx", "push rax", "pop rbx", "pop rax", "pop r12",
            ]),
            strings_vec!["push r12", "push rbx", "mov rbx, rax", "pop rax", "pop r12",]
        );

        assert_eq!(
            super::redundant_push(strings_vec!["push rbx", "push rax", "pop rbx", "pop rcx",]),
            strings_vec!["push rbx", "mov rbx, rax", "pop rcx",]
        );
    }

    #[test]
    fn redundant_mov() {
        let mut res = super::redundant_mov(strings_vec!["mov r11, r11",]);
        assert_eq!(res, Vec::<String>::new());

        res = super::redundant_mov(strings_vec!["mov r11, r12",]);
        assert_eq!(res, strings_vec!["mov r11, r12",]);

        res = super::redundant_mov(strings_vec!["mov r11, r12", "mov r11, r11",]);
        assert_eq!(res, strings_vec!["mov r11, r12",]);
    }

    #[test]
    fn redundant_jmp() {
        assert_eq!(
            super::redundant_jmp(strings_vec!["jmp L1", "L1:",]),
            strings_vec!["L1:",]
        );

        assert_eq!(
            super::redundant_jmp(strings_vec!["jmp L1",]),
            strings_vec!["jmp L1",]
        );

        assert_eq!(
            super::redundant_jmp(strings_vec!["jmp L1", "jmp L2", "L1:", "L2:",]),
            strings_vec!["jmp L1", "jmp L2", "L1:", "L2:"]
        );
        assert_eq!(
            super::redundant_jmp(strings_vec!["jmp L2", "jmp L1", "L1:", "L2:",]),
            strings_vec!["jmp L2", "L1:", "L2:"]
        );
    }
}
