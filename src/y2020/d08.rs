//! Day 8: Handheld Halting

/// accumulator value before any instruction is executed a second time
pub fn a(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    execute(&program).unwrap_err().to_string()
}

/// part b
pub fn b(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    for i in 0..program.len() {
        let mut modified_program = program.clone();
        match program[i].operation {
            Operation::Acc => continue,
            Operation::Jmp => modified_program[i].operation = Operation::Nop,
            Operation::Nop => modified_program[i].operation = Operation::Jmp,
        }
        if let Ok(acc) = execute(&modified_program) {
            return acc.to_string();
        }

    }
    unreachable!("solution not found")
}

fn parse_input(input: &Vec<&str>) -> Vec<Instruction> {
    input
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(op, arg)| Instruction {
            operation: match op {
                "acc" => Operation::Acc,
                "jmp" => Operation::Jmp,
                "nop" => Operation::Nop,
                _ => unreachable!("invalid instruction"),
            },
            argument: arg.parse().unwrap(),
        })
        .collect::<Vec<_>>()
}

fn execute(program: &Vec<Instruction>) -> Result<isize, isize> {
    let mut acc = 0isize;
    let mut ip = 0usize;
    let mut covered = vec![false; program.len()];
    while ip < program.len() {
        if covered[ip] {
            return Err(acc);
        }
        covered[ip] = true;
        let inst = &program[ip];
        match inst.operation {
            Operation::Acc => {
                acc += inst.argument;
                ip += 1;
            }
            Operation::Jmp => ip = ((ip as isize) + inst.argument) as usize,
            Operation::Nop => ip += 1,
        }
    }
    Ok(acc)
}

#[derive(Clone, Copy)]
enum Operation {
    Acc,
    Jmp,
    Nop,
}

#[derive(Clone, Copy)]
struct Instruction {
    operation: Operation,
    argument: isize,
}

#[test]
pub fn test() {
    let input = vec![
        "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4", "acc +6",
    ];

    assert_eq!(a(&input), "5");
    assert_eq!(b(&input), "8");
}
