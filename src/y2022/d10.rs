//! Day 10: Cathode-Ray Tube

/// sum of products of register value and cycle number during certain cycles
pub fn a(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    let mut vm = VirtualMachine::new();
    let mut total = 0;
    for cycle in 1..=220 {
        let result = vm.execute_cycle(&program).unwrap();
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            total += cycle * result;
        }
    }
    total.to_string()
}

/// rendered image
pub fn b(input: &Vec<&str>) -> String {
    let program = parse_input(input);
    let mut vm = VirtualMachine::new();
    let mut image = String::new();

    for cycle in 1..=240 {
        let pos = vm.execute_cycle(&program).unwrap();
        let x = (cycle - 1) % 40;
        if x == 0 {
            image.push('\n');
        }
        image.push(if x >= pos - 1 && x <= pos + 1 {
            '#'
        } else {
            '.'
        });
    }
    image
}

fn parse_input(input: &Vec<&str>) -> Program {
    input
        .iter()
        .map(|&s| match &s[0..4] {
            "addx" => Instruction {
                operation: Operation::Addx,
                value: s[5..].parse().unwrap(),
            },
            "noop" => Instruction {
                operation: Operation::Noop,
                value: 0,
            },
            _ => unreachable!("invalid instruction"),
        })
        .collect()
}

enum Operation {
    Addx,
    Noop,
}

struct Instruction {
    operation: Operation,
    value: isize,
}

type Program = Vec<Instruction>;

struct VirtualMachine {
    reg_x: isize,
    pending: bool,
    ip: usize,
}

impl VirtualMachine {
    fn new() -> Self {
        Self {
            reg_x: 1,
            pending: false,
            ip: 0,
        }
    }

    fn execute_cycle(&mut self, program: &Program) -> Result<isize, isize> {
        if let Some(instruction) = program.get(self.ip) {
            let result = self.reg_x;
            match instruction.operation {
                Operation::Addx => {
                    if self.pending {
                        self.reg_x += instruction.value;
                        self.pending = false;
                        self.ip += 1;
                    } else {
                        self.pending = true;
                    }
                }
                Operation::Noop => self.ip += 1,
            }
            Ok(result)
        } else {
            Err(self.reg_x)
        }
    }
}

#[test]
pub fn test() {
    let input = vec![
        "addx 15", "addx -11", "addx 6", "addx -3", "addx 5", "addx -1", "addx -8", "addx 13",
        "addx 4", "noop", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1", "addx 5", "addx -1",
        "addx 5", "addx -1", "addx -35", "addx 1", "addx 24", "addx -19", "addx 1", "addx 16",
        "addx -11", "noop", "noop", "addx 21", "addx -15", "noop", "noop", "addx -3", "addx 9",
        "addx 1", "addx -3", "addx 8", "addx 1", "addx 5", "noop", "noop", "noop", "noop", "noop",
        "addx -36", "noop", "addx 1", "addx 7", "noop", "noop", "noop", "addx 2", "addx 6", "noop",
        "noop", "noop", "noop", "noop", "addx 1", "noop", "noop", "addx 7", "addx 1", "noop",
        "addx -13", "addx 13", "addx 7", "noop", "addx 1", "addx -33", "noop", "noop", "noop",
        "addx 2", "noop", "noop", "noop", "addx 8", "noop", "addx -1", "addx 2", "addx 1", "noop",
        "addx 17", "addx -9", "addx 1", "addx 1", "addx -3", "addx 11", "noop", "noop", "addx 1",
        "noop", "addx 1", "noop", "noop", "addx -13", "addx -19", "addx 1", "addx 3", "addx 26",
        "addx -30", "addx 12", "addx -1", "addx 3", "addx 1", "noop", "noop", "noop", "addx -9",
        "addx 18", "addx 1", "addx 2", "noop", "noop", "addx 9", "noop", "noop", "noop", "addx -1",
        "addx 2", "addx -37", "addx 1", "addx 3", "noop", "addx 15", "addx -21", "addx 22",
        "addx -6", "addx 1", "noop", "addx 2", "addx 1", "noop", "addx -10", "noop", "noop",
        "addx 20", "addx 1", "addx 2", "addx 2", "addx -6", "addx -11", "noop", "noop", "noop",
    ];

    let program = parse_input(&vec!["noop", "addx 3", "addx -5"]);
    let mut vm = VirtualMachine::new();
    assert_eq!(vm.execute_cycle(&program), Ok(1));
    assert_eq!(vm.execute_cycle(&program), Ok(1));
    assert_eq!(vm.execute_cycle(&program), Ok(1));
    assert_eq!(vm.execute_cycle(&program), Ok(4));
    assert_eq!(vm.execute_cycle(&program), Ok(4));
    assert_eq!(vm.execute_cycle(&program), Err(-1));

    assert_eq!(a(&input), "13140");
    assert_eq!(
        b(&input),
        "\n\
        ##..##..##..##..##..##..##..##..##..##..\n\
        ###...###...###...###...###...###...###.\n\
        ####....####....####....####....####....\n\
        #####.....#####.....#####.....#####.....\n\
        ######......######......######......####\n\
        #######.......#######.......#######....."
    );
}
