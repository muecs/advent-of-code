//! Day 24: Arithmetic Logic Unit
//!
//! Observations:
//! * The MONAD program is grouped into 14 blocks of 18 instructions
//! * Each block reads input into `w`; clears `x` and `y`; only `z` has result
//! * It is not possible to keep `z` at zero per block, so they need balancing
//! * Operations are identical per block, just in 3 lines the operands differ
//! * Considering the 3 variables `a`/`b`/`c`, blocks are equivalent to:
//!   ```
//!   w = digit
//!   x = z % 26
//!   z /= a  // a is either 1 or 26
//!   if (x + b) != w {
//!     z = z * 26 + w + c
//!   }
//!   ```
//! * only blocks with `a == 26` can decrease `z` to 0, `z` has to be below 26
//! * there is an equal number of both types of blocks, they have to be paired

const DIGIT_COUNT: usize = 14;

/// largest 14 digit number accepted by given program
pub fn a(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    let number = find_accepted_number(&program, true);
    number.iter().map(|n| n.to_string()).collect::<String>()
}

/// smallest 14 digit number accepted by given program
pub fn b(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    let number = find_accepted_number(&program, false);
    number.iter().map(|n| n.to_string()).collect::<String>()
}

fn parse_input(input: &Vec<&str>) -> Program {
    let mut program = Program::with_capacity(input.len());
    for &line in input {
        let operation = match &line[0..3] {
            "inp" => Operation::Input,
            "add" => Operation::Add,
            "mul" => Operation::Multiply,
            "div" => Operation::Divide,
            "mod" => Operation::Modulo,
            "eql" => Operation::Equals,
            _ => unreachable!("invalid operation"),
        };
        let register = (line[4..5].as_bytes()[0] - b'w') as usize;
        let operand = if line.len() > 6 {
            let operand_str = &line[6..];
            if ["w", "x", "y", "z"].contains(&operand_str) {
                Operand::Register((operand_str.as_bytes()[0] - b'w') as usize)
            } else {
                Operand::Literal(operand_str.parse().unwrap())
            }
        } else {
            Operand::None
        };
        program.push(Instruction {
            operation,
            register,
            operand,
        });
    }
    program
}

fn find_accepted_number(program: &[Instruction], maximize: bool) -> [isize; DIGIT_COUNT] {
    let block_size = program.len() / DIGIT_COUNT;
    let digits = if maximize {
        (1..=9).rev().collect::<Vec<isize>>()
    } else {
        (1..=9).collect::<Vec<isize>>()
    };

    let mut stack = Vec::new();
    let mut number = [0isize; DIGIT_COUNT];

    let mut i = 0;
    while i < program.len() {
        let type_instruction = &program[i + 4];
        assert_eq!(type_instruction.operation, Operation::Divide);
        assert_eq!(type_instruction.register, 3);
        match type_instruction.operand {
            Operand::Literal(1) => stack.push(i),
            Operand::Literal(26) => {
                let j = stack.pop().unwrap();
                let block_pair =
                    [&program[j..j + block_size], &program[i..i + block_size]].concat();
                'outer: for a in &digits {
                    for b in &digits {
                        let z = evaluate(&block_pair, &[*a, *b])[3];
                        if z == 0 {
                            number[j / block_size] = *a;
                            number[i / block_size] = *b;
                            break 'outer;
                        }
                    }
                }
            }
            _ => unreachable!("unexpected instruction"),
        }
        i += block_size;
    }

    number
}

fn evaluate(program: &[Instruction], inputs: &[isize]) -> Registers {
    let mut registers = Registers::default();
    let mut i = 0;
    #[cfg(test)]
    println!("inputs: {:?}", inputs);
    for instruction in program {
        let registers_copy = registers.to_owned();
        let reg = &mut registers[instruction.register];
        #[cfg(test)]
        let reg_char = (b'w' + instruction.register as u8) as char;
        if instruction.operation == Operation::Input {
            if i >= inputs.len() {
                #[cfg(test)]
                println!("Warning: program expected more than {i} inputs");
                return registers; // ran out of inputs
            }
            #[cfg(test)]
            println!("{} = {}", reg_char, inputs[i]);
            *reg = inputs[i];
            i += 1;
        } else {
            #[allow(unused_variables)]
            let (operand, operand_str) = match instruction.operand {
                Operand::Register(r) => (
                    registers_copy[r],
                    String::from_utf8(vec![b'w' + r as u8]).unwrap(),
                ),
                Operand::Literal(l) => (l, format!("{}", l)),
                Operand::None => unreachable!(),
            };
            #[allow(unused_variables)]
            let operation_char = match instruction.operation {
                Operation::Input => unreachable!(),
                Operation::Add => {
                    *reg += operand;
                    '+'
                }
                Operation::Multiply => {
                    *reg *= operand;
                    '*'
                }
                Operation::Divide => {
                    *reg /= operand;
                    '/'
                }
                Operation::Modulo => {
                    *reg %= operand;
                    '%'
                }
                Operation::Equals => {
                    *reg = (registers_copy[instruction.register] == operand) as isize;
                    '='
                }
            };
            #[cfg(test)]
            println!(
                "{} {}= {} ({})",
                reg_char, operation_char, operand_str, *reg
            );
        }
    }
    registers
}

type Registers = [isize; 4]; // w|x|y|z

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equals,
}

#[derive(Clone, Debug)]
enum Operand {
    Register(usize),
    Literal(isize),
    None,
}

#[derive(Clone, Debug)]
struct Instruction {
    operation: Operation,
    register: usize,
    operand: Operand,
}
type Program = Vec<Instruction>;

#[test]
pub fn test() {
    let example1 = vec!["inp x", "mul x -1"];
    let program1 = parse_input(&example1);
    assert_eq!(program1.len(), 2);
    assert_eq!(evaluate(&program1, &[2]), [0, -2, 0, 0]);

    let example2 = vec!["inp z", "inp x", "mul z 3", "eql z x"];
    let program2 = parse_input(&example2);
    assert_eq!(program2.len(), 4);
    assert_eq!(evaluate(&program2, &[1, 3]), [0, 3, 0, 1]);
    assert_eq!(evaluate(&program2, &[2, 5]), [0, 5, 0, 0]);

    let example3 = vec![
        "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
        "mod x 2", "div w 2", "mod w 2",
    ];
    let program3 = parse_input(&example3);
    assert_eq!(program3.len(), 11);
    assert_eq!(evaluate(&program3, &[0]), [0, 0, 0, 0]);
    assert_eq!(evaluate(&program3, &[1]), [0, 0, 0, 1]);
    assert_eq!(evaluate(&program3, &[2]), [0, 0, 1, 0]);
    assert_eq!(evaluate(&program3, &[4]), [0, 1, 0, 0]);
    assert_eq!(evaluate(&program3, &[8]), [1, 0, 0, 0]);
    assert_eq!(evaluate(&program3, &[15]), [1, 1, 1, 1]);
    assert_eq!(evaluate(&program3, &[16]), [0, 0, 0, 0]);

    // assert_eq!(a(&input), "");
    // assert_eq!(b(&input), "");
}
