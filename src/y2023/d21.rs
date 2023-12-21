//! Day 21: Step Counter

use std::collections::HashSet;

/// number of reachable plots in 64 steps
pub fn a(input: &Vec<&str>) -> String {
    let (grid, start) = parse_input(input);
    count_path_options(&grid, &start, 64).to_string()
}

/// number of reachable plots in 26501365 steps on infinite map
pub fn b(input: &Vec<&str>) -> String {
    let (grid, start) = parse_input(input);
    count_path_options(&grid, &start, 26501365).to_string()
}

fn parse_input(input: &Vec<&str>) -> (Grid, Point) {
    let mut start: Point = (0, 0);
    (
        input
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'.' => false,
                        b'#' => true,
                        b'S' => {
                            start = (x.try_into().unwrap(), y.try_into().unwrap());
                            false
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start,
    )
}

fn count_path_options(grid: &Grid, start: &Point, length: usize) -> usize {
    let dim = grid.len() as i16;
    let mut pending = HashSet::<Point>::from([*start]);
    let mut plots = Vec::<i64>::new();

    for step in 0..length {
        if step % grid.len() == grid.len() / 2 {
            // we're on the edge between two repeating grids
            // take a snapshot of plots visited
            plots.push(pending.len() as i64);
            // println!("step: {step}, plots: {}", pending.len());
            if plots.len() == 3 {
                // can extrapolate based on 3 measurements
                let x = length as i64 / grid.len() as i64;
                let a = plots[0];
                let b = plots[1] - plots[0];
                let c = plots[2] - plots[1];
                return (x * (x - 1) / 2 * (c - b) + x * b + a) as usize;
            }
        }

        pending = pending
            .iter()
            .map(|p| [(1, 0), (0, 1), (-1, 0), (0, -1)].map(|(dx, dy)| (p.0 + dx, p.1 + dy)))
            .flatten()
            .filter(|p| !grid[p.1.rem_euclid(dim) as usize][p.0.rem_euclid(dim) as usize])
            .collect();
    }

    pending.len()
}

type Grid = Vec<Vec<bool>>;
type Point = (i16, i16);

#[test]
pub fn test() {
    let input = vec![
        "...........",
        ".....###.#.",
        ".###.##..#.",
        "..#.#...#..",
        "....#.#....",
        ".##..S####.",
        ".##..#...#.",
        ".......##..",
        ".##.#.####.",
        ".##..##.##.",
        "...........",
    ];

    let (grid, start) = parse_input(&input);
    assert_eq!(count_path_options(&grid, &start, 1), 2);
    assert_eq!(count_path_options(&grid, &start, 2), 4);
    assert_eq!(count_path_options(&grid, &start, 3), 6);
    assert_eq!(count_path_options(&grid, &start, 6), 16);
    assert_eq!(count_path_options(&grid, &start, 10), 50);
    // assert_eq!(count_path_options(&grid, &start, 50), 1594);
    // assert_eq!(count_path_options(&grid, &start, 100), 6536);
    // assert_eq!(count_path_options(&grid, &start, 500), 167004);
    // assert_eq!(count_path_options(&grid, &start, 1000), 668697);
    // assert_eq!(count_path_options(&grid, &start, 5000), 16733044);

    // assert_eq!(a(&input), "");
    // assert_eq!(b(&input), "");
}
