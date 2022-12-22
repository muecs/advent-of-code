//! Day 21: Monkey Math

use std::{collections::HashMap, str::FromStr};

/// Result of `root` equation
pub fn a(input: &Vec<&str>) -> String {
    let monkeys = parse_input(input);
    solve("root", &monkeys).to_string()
}

/// Value for `humn` node with equality at `root`
pub fn b(input: &Vec<&str>) -> String {
    let monkeys = parse_input(input);
    let mut variable_path = Vec::new();
    get_variable_path("root", &monkeys, &mut variable_path);
    // println!("variable path: {variable_path:?}");
    let value = inverse_solve("root", &0, variable_path, &monkeys);
    value.to_string()
}

fn parse_input<'a>(input: &'a Vec<&str>) -> Monkeys<'a> {
    input
        .iter()
        .map(|line| {
            (
                &line[0..4],
                if line.len() == 17 {
                    MonkeyJob::Equation {
                        op1: &line[6..10],
                        op2: &line[13..17],
                        operation: line[11..12].parse().unwrap(),
                    }
                } else {
                    MonkeyJob::Literal(line[6..].parse().unwrap())
                },
            )
        })
        .collect()
}

fn solve(monkey: &str, monkeys: &Monkeys) -> isize {
    match &monkeys[monkey] {
        MonkeyJob::Literal(val) => *val,
        MonkeyJob::Equation {
            op1,
            op2,
            operation,
        } => {
            let a = solve(op1, monkeys);
            let b = solve(op2, monkeys);
            match operation {
                Operation::Add => a + b,
                Operation::Subtract => a - b,
                Operation::Multiply => a * b,
                Operation::Divide => a / b,
            }
        }
    }
}

fn inverse_solve(monkey: &str, result: &isize, mut path: Vec<&str>, monkeys: &Monkeys) -> isize {
    if let Some(var) = path.pop() {
        match &monkeys[monkey] {
            MonkeyJob::Literal(_) => unreachable!(),
            MonkeyJob::Equation {
                op1,
                op2,
                operation,
            } => {
                let other = if *op1 == var { *op2 } else { *op1 };
                let sub_result = solve(other, monkeys);
                let var_result = if monkey == "root" {
                    sub_result
                } else if *op1 == var {
                    // result = var_result ? sub_result
                    match operation {
                        Operation::Add => result - sub_result,
                        Operation::Subtract => result + sub_result,
                        Operation::Multiply => result / sub_result,
                        Operation::Divide => result * sub_result,
                    }
                } else if *op2 == var {
                    // result = sub_result ? var_result
                    match operation {
                        Operation::Add => result - sub_result,
                        Operation::Subtract => sub_result - result,
                        Operation::Multiply => result / sub_result,
                        Operation::Divide => sub_result / result,
                    }
                } else {
                    unreachable!("no variable in equation");
                };
                return inverse_solve(var, &var_result, path, monkeys);
            }
        }
    } else if monkey == "humn" {
        return *result;
    }
    unreachable!();
}

/// finds the `humn` variable in the operation tree and builds a path to it
fn get_variable_path<'a>(monkey: &str, monkeys: &Monkeys<'a>, path: &mut Vec<&'a str>) -> bool {
    match &monkeys[monkey] {
        MonkeyJob::Literal(_) => monkey == "humn",
        MonkeyJob::Equation {
            op1,
            op2,
            operation: _,
        } => {
            if get_variable_path(op1, monkeys, path) {
                path.push(op1);
                true
            } else if get_variable_path(op2, monkeys, path) {
                path.push(op2);
                true
            } else {
                false
            }
        }
    }
}

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operation::Add),
            "-" => Ok(Operation::Subtract),
            "*" => Ok(Operation::Multiply),
            "/" => Ok(Operation::Divide),
            _ => Err("invalid operation"),
        }
    }
}

enum MonkeyJob<'a> {
    Literal(isize),
    Equation {
        op1: &'a str,
        op2: &'a str,
        operation: Operation,
    },
}

type Monkeys<'a> = HashMap<&'a str, MonkeyJob<'a>>;

#[test]
pub fn test() {
    let input = vec![
        "root: pppw + sjmn",
        "dbpl: 5",
        "cczh: sllz + lgvd",
        "zczc: 2",
        "ptdq: humn - dvpt",
        "dvpt: 3",
        "lfqf: 4",
        "humn: 5",
        "ljgn: 2",
        "sjmn: drzm * dbpl",
        "sllz: 4",
        "pppw: cczh / lfqf",
        "lgvd: ljgn * ptdq",
        "drzm: hmdt - zczc",
        "hmdt: 32",
    ];

    assert_eq!(a(&input), "152");
    assert_eq!(b(&input), "301");
}
