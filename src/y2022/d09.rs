//! Day 9: Rope Bridge

use std::collections::BTreeSet;

/// number of positions visited by 2-node rope tail at least once
pub fn a(input: &Vec<&str>) -> String {
    let motions = parse_input(input);
    simulate(&motions, 2).to_string()
}

/// number of positions visited by 10-node rope tail at least once
pub fn b(input: &Vec<&str>) -> String {
    let motions = parse_input(input);
    simulate(&motions, 10).to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Motion> {
    input
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(dir, steps)| {
            let n: isize = steps.parse().unwrap();
            match dir {
                "R" => Motion::new(true, n),
                "U" => Motion::new(false, n),
                "L" => Motion::new(true, -n),
                "D" => Motion::new(false, -n),
                _ => unreachable!("invalid direction symbol"),
            }
        })
        .collect::<Vec<_>>()
}

fn simulate(motions: &Vec<Motion>, len: usize) -> usize {
    let mut rope = vec![(0, 0); len];
    let mut tail_positions = BTreeSet::new();

    for motion in motions {
        let steps = motion.distance.abs();
        let delta = motion.distance / steps;
        for _ in 0..steps {
            if motion.horizontal {
                rope[0].0 += delta;
            } else {
                rope[0].1 += delta;
            }
            for i in 1..len {
                let diff = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                // if diff.0.abs() >= 2 {
                //     rope[i].0 = (rope[i - 1].0 + rope[i].0) / 2;
                //     rope[i].1 = rope[i - 1].1;
                // } else if diff.1.abs() >= 2 {
                //     rope[i].1 = (rope[i - 1].1 + rope[i].1) / 2;
                //     rope[i].0 = rope[i - 1].0;
                // }
                if diff.0.abs() >= 2 || diff.1.abs() >= 2 {
                    rope[i].0 += diff.0.signum();
                    rope[i].1 += diff.1.signum();
                }
            }
            tail_positions.insert(rope[len - 1]);
        }
    }

    // println!("tail positions: {}", tail_positions.len());
    // tail_positions.iter().for_each(|p| println!("{}, {}", p.0, p.1));

    tail_positions.len()
}

struct Motion {
    horizontal: bool,
    distance: isize,
}

impl Motion {
    fn new(horizontal: bool, distance: isize) -> Self {
        Self {
            horizontal,
            distance,
        }
    }
}

// type Point = (isize, isize);

#[test]
pub fn test() {
    let input = vec!["R 4", "U 4", "L 3", "D 1", "R 4", "D 1", "L 5", "R 2"];
    let input2 = vec!["R 5", "U 8", "L 8", "D 3", "R 17", "D 10", "L 25", "U 20"];

    assert_eq!(a(&input), "13");
    assert_eq!(b(&input), "1");
    assert_eq!(b(&input2), "36");
}
