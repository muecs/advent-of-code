//! Day 24: Blizzard Basin
//!
//! Assumptions: Map is rectangular, entrance at top left, exit at bottom right

use std::{collections::{HashSet, BinaryHeap}, cmp::Reverse};

/// Fewest number of steps required to reach goal
pub fn a(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    let start: Point = (0, 1);
    let end: Point = (map.width - 2, map.height - 1);
    let steps = find_path(&map, &start, &end, 0);
    steps.to_string()
}

/// Fewest number of steps to goal, back, and there again
pub fn b(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    let start: Point = (0, 1);
    let end: Point = (map.width - 2, map.height - 1);
    let mut steps = find_path(&map, &start, &end, 0);
    steps = find_path(&map, &end, &start, steps);
    steps = find_path(&map, &start, &end, steps);
    steps.to_string()
}

fn parse_input(input: &Vec<&str>) -> Map {
    Map::new(
        input[0].len(),
        input.len(),
        input
            .iter()
            .enumerate() // include walls
            .skip(1)
            .take(input.len() - 2)
            .map(move |(y, line)| {
                line.bytes().enumerate().filter_map(move |(x, b)| {
                    let pos = (x as i8, y as i8);
                    match b {
                        b'>' => Some(Blizzard {
                            pos,
                            dir: Direction::East,
                        }),
                        b'v' => Some(Blizzard {
                            pos,
                            dir: Direction::South,
                        }),
                        b'<' => Some(Blizzard {
                            pos,
                            dir: Direction::West,
                        }),
                        b'^' => Some(Blizzard {
                            pos,
                            dir: Direction::North,
                        }),
                        _ => None,
                    }
                })
            })
            .flatten()
            .collect::<Blizzards>(),
    )
}

fn find_path(map: &Map, start: &Point, end: &Point, step: usize) -> usize {
    let mut visited = HashSet::from([(*start, step)]);
    let mut queue = BinaryHeap::from([(Reverse(0), *start, step)]);

    while let Some((_, pos, step)) = queue.pop() {
        for v in [(1, 0), (0, 1), (-1, 0), (0, -1), (0, 0)] {
            let new_pos = (pos.0 + v.0, pos.1 + v.1);

            // check target (on boundary)
            if new_pos == *end {
                return step + 1;
            }

            if new_pos != *start {
                // check W/E boundaries
                if new_pos.0 < 1 || new_pos.0 > map.width - 2 {
                    continue;
                }

                // check N/S boundaries
                if new_pos.1 < 1 || new_pos.1 > map.height - 2 {
                    continue;
                }

                // check blizzards
                if map.has_blizzard(&(step + 1), &new_pos) {
                    continue;
                }
            }

            if visited.insert((new_pos, step + 1)) {
                queue.push((Reverse(step + distance(&new_pos, end)), new_pos, step + 1));
            }
        }
    }

    unreachable!("failed to find solution");
}

/// calculates Manhattan Distance between two points
fn distance(p1: &Point, p2: &Point) -> usize {
    (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)) as usize
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

type Point = (i8, i8);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Blizzard {
    pos: Point,
    dir: Direction,
}

type Blizzards = Vec<Blizzard>;

struct Map {
    width: i8,
    height: i8,
    periodicity: usize,
    blizzards_per_step: Vec<Blizzards>,
}

impl Map {
    fn new(width: usize, height: usize, mut blizzards: Blizzards) -> Self {
        let periodicity = (width - 2) * (height - 2);
        let mut map = Self {
            width: width as i8,
            height: height as i8,
            periodicity,
            blizzards_per_step: Vec::with_capacity(periodicity),
        };

        // build blizzard cache
        map.blizzards_per_step.push(blizzards.to_owned());
        for _ in 1..periodicity {
            for b in blizzards.iter_mut() {
                match b.dir {
                    Direction::East => {
                        b.pos.0 += 1;
                        if b.pos.0 + 1 == map.width {
                            b.pos.0 = 1
                        }
                    }
                    Direction::South => {
                        b.pos.1 += 1;
                        if b.pos.1 + 1 == map.height {
                            b.pos.1 = 1
                        }
                    }
                    Direction::West => {
                        b.pos.0 -= 1;
                        if b.pos.0 == 0 {
                            b.pos.0 = map.width - 2
                        }
                    }
                    Direction::North => {
                        b.pos.1 -= 1;
                        if b.pos.1 == 0 {
                            b.pos.1 = map.height - 2
                        }
                    }
                }
            }
            blizzards.sort_unstable_by_key(|b| b.pos);
            map.blizzards_per_step.push(blizzards.to_owned());
        }

        map
    }

    fn has_blizzard(&self, step: &usize, pos: &Point) -> bool {
        self.blizzards_per_step[*step % self.periodicity]
            .binary_search_by_key(&pos, |b| &b.pos)
            .is_ok()
    }
}

#[test]
pub fn test() {
    let input = vec![
        "#.######", "#>>.<^<#", "#.<..<<#", "#>v.><>#", "#<^v^^>#", "######.#",
    ];

    let map = parse_input(&input);
    assert_eq!(map.width, 8);
    assert_eq!(map.height, 6);
    assert_eq!(map.blizzards_per_step[0].len(), 19);

    assert_eq!(a(&input), "18");
    assert_eq!(b(&input), "54");
}
