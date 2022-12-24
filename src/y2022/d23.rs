//! Day 23: Unstable Diffusion

use std::collections::{HashMap, HashSet};

/// Number of empty positions in bounding rectangle
pub fn a(input: &Vec<&str>) -> String {
    let mut points = parse_input(input);
    simulate(&mut points, 10);
    let (min, max) = points.iter().fold(
        ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN)),
        |acc, p| {
            (
                (acc.0 .0.min(p.0), acc.0 .1.min(p.1)),
                (acc.1 .0.max(p.0), acc.1 .1.max(p.1)),
            )
        },
    );
    let area = ((max.0 + 1 - min.0) * (max.1 + 1 - min.1)) as usize;
    (area - points.len()).to_string()
}

/// Number of first round with equilibrium state
pub fn b(input: &Vec<&str>) -> String {
    let mut points = parse_input(input);
    let rounds = simulate(&mut points, 1000000);
    rounds.to_string()
}

fn parse_input(input: &Vec<&str>) -> PointSet {
    let mut points = PointSet::new();
    for (y, line) in input.iter().enumerate() {
        for (x, b) in line.bytes().enumerate() {
            if b == b'#' {
                points.insert((x as isize, y as isize));
            }
        }
    }
    points
}

fn simulate(positions: &mut PointSet, max_rounds: usize) -> usize {
    const DIRECTIONS: [Point; 4] = [
        (0, -1), // N
        (0, 1),  // S
        (-1, 0), // W
        (1, 0),  // E
    ];
    let mut start_dir = 0usize;

    for round in 0..max_rounds {
        let mut proposed = PointMap::with_capacity(positions.len());

        for pos in positions.iter() {
            // check adjacent positions
            let mut n = 0;
            for y in pos.1 - 1..=pos.1 + 1 {
                for x in pos.0 - 1..=pos.0 + 1 {
                    let adj = (x, y);
                    if adj != *pos && positions.contains(&adj) {
                        n += 1;
                    }
                }
            }
            if n == 0 {
                // no need to move
                continue;
            }

            // check directions in order, select if all 3 positions empty
            for d in 0..4 {
                let dir = DIRECTIONS[(start_dir + d) % 4];
                let empty = (-1..=1)
                    .map(|a| if dir.0 == 0 { (a, dir.1) } else { (dir.0, a) })
                    .all(|p| !positions.contains(&(pos.0 + p.0, pos.1 + p.1)));
                if empty {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    proposed
                        .entry(new_pos)
                        .and_modify(|e| *e = (isize::MIN, isize::MIN))
                        .or_insert(*pos);
                    break;
                }
            }
        }

        if proposed.is_empty() {
            // reached equilibrium
            return round + 1;
        }

        // move to proposed position if uniquely selected
        for (new_pos, old_pos) in &proposed {
            if positions.remove(old_pos) {
                positions.insert(*new_pos);
            }
        }

        start_dir = (start_dir + 1) % 4;
    }

    usize::MAX // did not terminate
}

type Point = (isize, isize);
type PointSet = HashSet<Point>;
type PointMap = HashMap<Point, Point>;

#[test]
pub fn test() {
    let input = vec![
        "....#..", "..###.#", "#...#.#", ".#...##", "#.###..", "##.#.##", ".#..#..",
    ];

    let test_input1 = vec![".....", "..##.", "..#..", ".....", "..##.", "....."];
    let test_input2 = vec!["..##.", ".....", "..#..", "...#.", "..#..", "....."];
    let test_input3 = vec![".....", "..##.", ".#...", "....#", ".....", "..#.."];

    let mut points = parse_input(&test_input1);
    simulate(&mut points, 1);
    assert_eq!(points, parse_input(&test_input2));

    let mut points = parse_input(&test_input1);
    simulate(&mut points, 2);
    assert_eq!(points, parse_input(&test_input3));

    assert_eq!(a(&test_input1), "25");
    assert_eq!(a(&input), "110");
    assert_eq!(b(&test_input1), "4");
    assert_eq!(b(&input), "20");
}
