//! Day 18: RAM Run

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[cfg(test)]
const DIM: usize = 7;
#[cfg(not(test))]
const DIM: usize = 71;

#[cfg(test)]
const LIMIT: usize = 12;
#[cfg(not(test))]
const LIMIT: usize = 1024;

/// shortest path through first KB of fallen bytes
pub fn a(input: &Vec<&str>) -> String {
    let bytes = parse_input(input);
    let mut map = vec![vec![true; DIM]; DIM];
    bytes
        .iter()
        .take(LIMIT)
        .for_each(|&(x, y)| map[y][x] = false);
    find_path(&map).len().saturating_sub(1).to_string()
}

/// coords of first byte to block all paths
pub fn b(input: &Vec<&str>) -> String {
    let bytes = parse_input(input);
    let mut map = vec![vec![true; DIM]; DIM];
    let mut path = Vec::new();
    let (_, (x, y)) = bytes
        .iter()
        .enumerate()
        .find(|&(ref i, &p)| {
            map[p.1][p.0] = false;
            if *i < LIMIT || !(path.is_empty() || path.contains(&p)) {
                return false;
            }
            // path blocked, find alternative
            path = find_path(&map);
            path.is_empty()
        })
        .unwrap();
    format!("{x},{y}")
}

fn parse_input(input: &Vec<&str>) -> Vec<Pos> {
    input
        .iter()
        .map(|s| {
            s.split_once(',')
                .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                .unwrap()
        })
        .collect()
}

/// determines shortest path
fn find_path(map: &Map) -> Vec<Pos> {
    let start = (0, 0); // top left
    let end = (DIM - 1, DIM - 1); // bottom right
    let mut unvisited = BinaryHeap::from([(Reverse(0usize), start)]);
    let mut costs = HashMap::from([(start, (0usize, start))]);

    while let Some((_, pos)) = unvisited.pop() {
        let (cost, prev_pos) = *costs.entry(pos).or_insert((usize::MAX, pos));

        if pos == end {
            // reached bottom right; reconstruct path
            let mut path = vec![pos, prev_pos];
            loop {
                let p = costs.get(path.last().unwrap()).unwrap().1;
                path.push(p);
                if p == start {
                    return path;
                }
            }
        }

        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            // check grid boundaries
            if (pos.0 == 0 && dx < 0) || (pos.0 + 1 == DIM && dx > 0) {
                continue;
            }
            if (pos.1 == 0 && dy < 0) || (pos.1 + 1 == DIM && dy > 0) {
                continue;
            }

            let next_pos = (
                pos.0.checked_add_signed(dx).unwrap(),
                pos.1.checked_add_signed(dy).unwrap(),
            );
            let next_cost = cost + 1;

            if !map[next_pos.1][next_pos.0] {
                continue; // blocked
            }

            let (prev_cost, prev_pos) = costs.entry(next_pos).or_insert((usize::MAX, next_pos));
            if next_cost < *prev_cost {
                // found a shorter path to adjacent tile
                *prev_cost = next_cost;
                *prev_pos = pos;
                unvisited.push((Reverse(next_cost), next_pos));
            }
        }
    }

    Vec::new()
}

type Pos = (usize, usize);
type Map = Vec<Vec<bool>>;

#[test]
pub fn test() {
    let input = vec![
        "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1", "1,2",
        "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6", "2,0",
    ];

    assert_eq!(a(&input), "22");
    assert_eq!(b(&input), "6,1");
}
