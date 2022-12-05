//! Day 5: Supply Stacks

use std::collections::VecDeque;

/// top of each stack after rearrangement, crate by crate
pub fn a(input: &Vec<&str>) -> String {
    let (mut stacks, steps) = parse_input(input);
    rearrange(&mut stacks, &steps);
    stacks
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect::<String>()
}

/// top of each stack after rearrangement, crates in batches
pub fn b(input: &Vec<&str>) -> String {
    let (mut stacks, steps) = parse_input(input);
    rearrange_batched(&mut stacks, &steps);
    stacks
        .iter()
        .map(|stack| *stack.back().unwrap())
        .collect::<String>()
}

fn parse_input(input: &Vec<&str>) -> (Vec<VecDeque<char>>, Vec<Step>) {
    let stack_count = (input[0].len() + 1) / 4;
    let mut stacks = vec![VecDeque::new(); stack_count];
    let mut it = input.iter();
    loop {
        let line = it.next().unwrap().as_bytes();
        if line[1] == b'1' {
            break;
        }
        for i in 0..stack_count {
            let c = line[1 + i * 4] as char;
            if c != ' ' {
                stacks[i].push_front(c);
            }
        }
    }

    assert_eq!(it.next(), Some(&""));

    let steps = it
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Step>()
        })
        .collect::<Vec<_>>();

    (stacks, steps)
}

fn rearrange(stacks: &mut Vec<VecDeque<char>>, steps: &Vec<Step>) {
    for step in steps {
        for _ in 0..step.count {
            let c = stacks[step.from].pop_back().unwrap();
            stacks[step.to].push_back(c);
        }
    }
}

fn rearrange_batched(stacks: &mut Vec<VecDeque<char>>, steps: &Vec<Step>) {
    for step in steps {
        let new_len = stacks[step.from].len() - step.count;
        let batch = stacks[step.from]
            .iter()
            .skip(new_len)
            .copied()
            .collect::<Vec<_>>();
        stacks[step.from].resize(new_len, ' ');
        stacks[step.to].extend(batch.iter());
    }
}

struct Step {
    count: usize,
    from: usize,
    to: usize,
}

impl FromIterator<usize> for Step {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        let mut it = iter.into_iter();
        Self {
            count: it.next().unwrap(),
            from: it.next().unwrap() - 1,
            to: it.next().unwrap() - 1,
        }
    }
}

#[test]
pub fn test() {
    let input = vec![
        "    [D]    ",
        "[N] [C]    ",
        "[Z] [M] [P]",
        " 1   2   3 ",
        "",
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];

    assert_eq!(a(&input), "CMZ");
    assert_eq!(b(&input), "MCD");
}
