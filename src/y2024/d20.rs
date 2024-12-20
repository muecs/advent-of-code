//! Day 20: Race Condition

use std::collections::HashMap;

#[cfg(test)]
const LIMIT: usize = 50;
#[cfg(not(test))]
const LIMIT: usize = 100;

/// cheats of length 2 saving at least 100 steps
pub fn a(input: &Vec<&str>) -> String {
    let (map, start) = parse_input(input);
    find_cheats(&map, &start, 2).to_string()
}

/// cheats of length 20 saving at least 100 steps
pub fn b(input: &Vec<&str>) -> String {
    let (map, start) = parse_input(input);
    find_cheats(&map, &start, 20).to_string()
}

fn parse_input(input: &Vec<&str>) -> (Map, Pos) {
    let mut start = Pos::default();
    let map = input
        .iter()
        .enumerate()
        .map(|(y, s)| {
            s.char_indices()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' | 'E' => true,
                    'S' => {
                        start = (x, y);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn find_cheats(map: &Map, start: &Pos, max_dist: usize) -> usize {
    let mut course = HashMap::new();
    let mut pending = vec![*start];
    let mut steps = 0usize;
    let mut cheats = 0usize;

    while let Some(pos) = pending.pop() {
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next_pos = (
                pos.0.checked_add_signed(dx).unwrap(),
                pos.1.checked_add_signed(dy).unwrap(),
            );
            if map[next_pos.1][next_pos.0] && !course.contains_key(&next_pos) {
                pending.push(next_pos);
                break; // only one way
            }
        }

        // count allowable cheats from this tile to previous ones
        cheats += course
            .iter()
            .filter(|(prev_pos, prev_steps)| {
                let dist = distance(&pos, prev_pos);
                dist <= max_dist && steps - *prev_steps >= dist + LIMIT
            })
            .count();

        course.insert(pos, steps);
        steps += 1;
    }

    cheats
}

/// calculates Manhattan Distance between two points
fn distance(p1: &Pos, p2: &Pos) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

type Pos = (usize, usize);
type Map = Vec<Vec<bool>>;

#[test]
pub fn test() {
    let input = vec![
        "###############",
        "#...#...#.....#",
        "#.#.#.#.#.###.#",
        "#S#...#.#.#...#",
        "#######.#.#.###",
        "#######.#.#...#",
        "#######.#.###.#",
        "###..E#...#...#",
        "###.#######.###",
        "#...###...#...#",
        "#.#####.#.###.#",
        "#.#...#.#.#...#",
        "#.#.#.#.#.#.###",
        "#...#...#...###",
        "###############",
    ];

    assert_eq!(a(&input), "1");
    assert_eq!(b(&input), "285");
}
