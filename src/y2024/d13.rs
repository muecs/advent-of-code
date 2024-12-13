//! Day 13: Claw Contraption

/// fewest tokens to win all possible prizes
pub fn a(input: &Vec<&str>) -> String {
    let machines = parse_input(input);
    machines.iter().map(min_tokens).sum::<isize>().to_string()
}

/// shifted price coords
pub fn b(input: &Vec<&str>) -> String {
    const OFFSET: isize = 10000000000000;
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine| {
            min_tokens(&Machine {
                a: machine.a,
                b: machine.b,
                p: (machine.p.0 + OFFSET, machine.p.1 + OFFSET),
            })
        })
        .sum::<isize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Machine> {
    input
        .chunks(4)
        .map(|machine| Machine {
            a: machine[0]
                .split_once(',')
                .map(|(x, y)| (x[12..].parse().unwrap(), y[3..].parse().unwrap()))
                .unwrap(),
            b: machine[1]
                .split_once(',')
                .map(|(x, y)| (x[12..].parse().unwrap(), y[3..].parse().unwrap()))
                .unwrap(),
            p: machine[2]
                .split_once(',')
                .map(|(x, y)| (x[9..].parse().unwrap(), y[3..].parse().unwrap()))
                .unwrap(),
        })
        .collect()
}

/// minimize A (3 tokens), maximize B (1 token)
fn min_tokens(machine: &Machine) -> isize {
    /*
    // How many times does B fit into P?
    let max_b = isize::min(machine.p.0 / machine.b.0, machine.p.1 / machine.b.1);
    for b in (1..=max_b).rev() {
        let rem = (machine.p.0 - b * machine.b.0, machine.p.1 - b * machine.b.1);
        if rem.0 % machine.a.0 == 0 && rem.1 % machine.a.1 == 0 {
            // How many multiples of A fit into remainder?
            let a = (rem.0 / machine.a.0, rem.1 / machine.a.1);
            if a.0 == a.1 {
                return a.0 * 3 + b;
            }
        }
    }
    */

    // solve 2x2 equation system assuming there is only one solution
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
    let a = (machine.p.0 * machine.b.1 - machine.p.1 * machine.b.0) / det;
    let b = (machine.a.0 * machine.p.1 - machine.a.1 * machine.p.0) / det;
    if (machine.a.0 * a + machine.b.0 * b, machine.a.1 * a + machine.b.1 * b) == machine.p {
        return a * 3 + b;
    }

    0 // no solution
}

type Pos = (isize, isize);

#[derive(Clone, Debug)]
struct Machine {
    a: Pos,
    b: Pos,
    p: Pos,
}

#[test]
pub fn test() {
    let input = vec![
        "Button A: X+94, Y+34",
        "Button B: X+22, Y+67",
        "Prize: X=8400, Y=5400",
        "",
        "Button A: X+26, Y+66",
        "Button B: X+67, Y+21",
        "Prize: X=12748, Y=12176",
        "",
        "Button A: X+17, Y+86",
        "Button B: X+84, Y+37",
        "Prize: X=7870, Y=6450",
        "",
        "Button A: X+69, Y+23",
        "Button B: X+27, Y+71",
        "Prize: X=18641, Y=10279",
    ];

    assert_eq!(a(&input), "480");
    // assert_eq!(b(&input), "");
}
