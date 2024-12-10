//! Day 10: Hoof It

use std::collections::{HashSet, VecDeque};

/// sum of number of peaks reachable from each trailhead
pub fn a(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    trailheads(&map)
        .iter()
        .map(|start| rate_trailhead(&map, &start, false))
        .sum::<usize>()
        .to_string()
}

/// part b
pub fn b(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    trailheads(&map)
        .iter()
        .map(|start| rate_trailhead(&map, &start, true))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Map {
    input
        .iter()
        .map(|s| {
            s.bytes()
                .map(|b| match b {
                    b'0'..=b'9' => b - b'0',
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn trailheads(map: &Map) -> Vec<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, b)| (*b == 0).then_some((x, y)))
        })
        .collect()
}

fn rate_trailhead(map: &Map, start: &(usize, usize), distinct: bool) -> usize {
    // BFS to find paths from given 0 to all reachable 9s
    let mut pending = VecDeque::from([*start]);
    let mut peaks = HashSet::new();
    let mut score = 0;
    while let Some(pos) = pending.pop_front() {
        for (dx, dy) in [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)] {
            // check grid boundaries
            if (pos.0 == 0 && dx < 0) || (pos.0 + 1 == map[0].len() && dx > 0) {
                continue;
            }
            if (pos.1 == 0 && dy < 0) || (pos.1 + 1 == map.len() && dy > 0) {
                continue;
            }

            let next = (
                pos.0.checked_add_signed(dx).unwrap(),
                pos.1.checked_add_signed(dy).unwrap(),
            );
            if map[next.1][next.0] == map[pos.1][pos.0] + 1 {
                if map[next.1][next.0] == 9 {
                    // reached goal
                    if distinct {
                        score += 1
                    } else {
                        peaks.insert(next);
                    }
                } else {
                    pending.push_back(next);
                }
            }
        }
    }
    if distinct {
        score
    } else {
        peaks.len()
    }
}

type Map = Vec<Vec<u8>>;

#[test]
pub fn test() {
    let input = vec![
        "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
        "10456732",
    ];

    assert_eq!(a(&input), "36");
    assert_eq!(b(&input), "81");
}
