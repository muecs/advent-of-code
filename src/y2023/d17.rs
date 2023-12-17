//! Day 17: Clumsy Crucible

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

/// path with min heat loss for crucible
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    find_min_path(&grid, false).to_string()
}

/// path with min heat loss for ultra crucible
pub fn b(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    find_min_path(&grid, true).to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    Grid {
        blocks: input
            .iter()
            .map(|s| s.bytes().map(|b| b - b'0'))
            .flatten()
            .collect(),
        width: input[0].len(),
        height: input.len(),
    }
}

fn find_min_path(grid: &Grid, ultra: bool) -> usize {
    let mut unvisited = BinaryHeap::from([(
        Reverse(0),
        Point {
            idx: 0,
            dir: 0,
            straight: 0,
        },
    )]);
    let mut costs = HashMap::<Point, usize>::new();

    while let Some(candidate) = unvisited.pop() {
        let p = &candidate.1;
        if costs.contains_key(p) {
            continue; // already processed
        }

        let cost = candidate.0 .0;
        costs.insert(*p, cost);

        let x = p.idx % grid.width;
        let y = p.idx / grid.width;
        for (dir, (dx, dy)) in [(1, 0), (0, 1), (-1, 0), (0, -1)].iter().enumerate() {
            // check direction constraints
            let dir = dir as u8;
            if dir == (p.dir + 2) % 4 {
                // can't reverse direction
                continue;
            }
            let straight = if dir == p.dir { p.straight + 1 } else { 1 };
            if ultra {
                if straight > 10 {
                    // can't move more than 10 blocks in the same direction
                    continue;
                }
                if dir != p.dir && p.straight > 0 && p.straight < 4 {
                    // can't have moved less than 4 blocks before a turn
                    continue;
                }
            } else if straight > 3 {
                // can't move more than 3 blocks in the same direction
                continue;
            }

            // check grid boundaries
            if (x == 0 && *dx < 0) || (x + 1 == grid.width && *dx > 0) {
                continue;
            }
            if (y == 0 && *dy < 0) || (y + 1 == grid.height && *dy > 0) {
                continue;
            }

            let idx = (x as isize + dx) as usize + (y as isize + dy) as usize * grid.width;
            let new_cost = cost + grid.blocks[idx] as usize;
            unvisited.push((Reverse(new_cost), Point { idx, dir, straight }));
        }
    }

    *costs
        .iter()
        .filter_map(|(p, cost)| (p.idx + 1 == grid.blocks.len()).then_some(cost))
        .min()
        .unwrap()
}

struct Grid {
    blocks: Vec<u8>,
    width: usize,
    height: usize,
}

#[derive(Clone, Copy, Hash, Eq, Ord, PartialEq, PartialOrd)]
struct Point {
    idx: usize,
    dir: u8,
    straight: u8,
}

#[test]
pub fn test() {
    let input = vec![
        "2413432311323",
        "3215453535623",
        "3255245654254",
        "3446585845452",
        "4546657867536",
        "1438598798454",
        "4457876987766",
        "3637877979653",
        "4654967986887",
        "4564679986453",
        "1224686865563",
        "2546548887735",
        "4322674655533",
    ];

    assert_eq!(a(&input), "102");
    assert_eq!(b(&input), "94");
}
