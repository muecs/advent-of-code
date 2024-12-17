//! Day 17: Chronospatial Computer

/// determine program output
pub fn a(input: &Vec<&str>) -> String {
    let mut machine = parse_input(input);
    machine.run();
    machine.format_output()
}

/// lowest possible A so that output equals program
pub fn b(input: &Vec<&str>) -> String {
    /* Observations:
     *   - program loops while A != 0
     *   - loop state is only kept in A
     *   - each iteration consumes last 3 bits of A
     *   - each iteration prints B mod 8, with B calculated from A
     */

    let mut machine = parse_input(input);
    let mut a = 0;
    for tail in (0..machine.prog.len()).rev() {
        // build A iteratively for groups of 3 bits
        for i in 0..8 {
            // find the 3 bit number that matches the end of the program
            let candidate = a << 3 | i;
            machine.a = candidate;
            machine.ip = 0;
            machine.out.clear();
            machine.run();
            if machine.out[..] == machine.prog[tail..] {
                // println!("{candidate} {}", machine.format_output());
                a = candidate;
                break;
            }
        }
    }

    a.to_string()
}

fn parse_input(input: &Vec<&str>) -> Machine {
    Machine {
        a: input[0][12..].parse().unwrap(),
        b: input[1][12..].parse().unwrap(),
        c: input[2][12..].parse().unwrap(),
        ip: 0,
        prog: input[4][9..]
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect(),
        out: Vec::new(),
    }
}

impl Machine {
    fn run(&mut self) {
        while self.ip + 1 < self.prog.len() {
            self.step()
        }
    }

    fn step(&mut self) {
        let op = self.op();
        let val = self.prog[self.ip + 1];
        // println!(
        //     "{:x}: {op:?} {val} (A={}, B={}, C={})",
        //     self.ip, self.a, self.b, self.c
        // );
        match op {
            Op::Adv => self.a /= 1 << self.combo(val),
            Op::Bxl => self.b ^= val as usize,
            Op::Bst => self.b = self.combo(val) % 8,
            Op::Jnz => {
                if self.a != 0 {
                    self.ip = val as usize;
                    return;
                }
            }
            Op::Bxc => self.b ^= self.c,
            Op::Out => self.out.push((self.combo(val) % 8) as u8),
            Op::Bdv => self.b = self.a / (1 << self.combo(val)),
            Op::Cdv => self.c = self.a / (1 << self.combo(val)),
        }
        self.ip += 2;
    }

    fn op(&self) -> Op {
        self.prog[self.ip].into()
    }

    fn combo(&self, val: u8) -> usize {
        match val {
            0..=3 => val as usize,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!(),
        }
    }

    fn format_output(&self) -> String {
        self.out
            .iter()
            .map(|&b| String::from((b'0' + b) as char))
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    /// division of A by 2^combo to A
    Adv,
    /// bitwise XOR of B and lit to B
    Bxl,
    /// combo mod 8 to B
    Bst,
    /// jump to A if not zero
    Jnz,
    /// bitwise XOR of B and C to B, ignore lit
    Bxc,
    /// print combo mod 8
    Out,
    /// division of A by 2^combo to B
    Bdv,
    /// division of A by 2^combo to C
    Cdv,
}

#[derive(Clone, Debug)]
struct Machine {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    prog: Vec<u8>,
    out: Vec<u8>,
}

#[test]
pub fn test() {
    let input1 = vec![
        "Register A: 729",
        "Register B: 0",
        "Register C: 0",
        "",
        "Program: 0,1,5,4,3,0",
    ];

    let _input2 = vec![
        "Register A: 2024",
        "Register B: 0",
        "Register C: 0",
        "",
        "Program: 0,3,5,4,3,0",
    ];

    assert_eq!(a(&input1), "4,6,3,5,6,3,5,2,1,0");
    // assert_eq!(b(&input2), "117440");
}
