//! Day 12: Hill Climbing Algorithm

use std::collections::{BinaryHeap, HashMap};

/// length of shortest path with max ascent of 1
pub fn a(input: &Vec<&str>) -> String {
    let (heightmap, start, end) = parse_input(input);
    find_path(&heightmap, &start, &end).to_string()
}

/// length of shortest path with free choice of starting position
pub fn b(input: &Vec<&str>) -> String {
    let (heightmap, _, end) = parse_input(input);
    let mut start_options = Vec::new();
    for y in 0..heightmap.len() {
        for x in 0..heightmap[0].len() {
            if heightmap[y][x] == 0 {
                start_options.push((x, y));
            }
        }
    }
    start_options
        .iter()
        .map(|p| find_path(&heightmap, p, &end))
        .min()
        .unwrap()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> (Heightmap, Point, Point) {
    let mut start = Point::default();
    let mut end = Point::default();

    let heightmap = input
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.bytes()
                .enumerate()
                .map(|(x, b)| match b {
                    b'S' => {
                        start = (x, y);
                        0
                    }
                    b'E' => {
                        end = (x, y);
                        25
                    }
                    _ => b - b'a',
                })
                .collect()
        })
        .collect();

    (heightmap, start, end)
}

fn find_path(heightmap: &Heightmap, start: &Point, end: &Point) -> usize {
    let height = heightmap.len();
    let width = heightmap[0].len();
    let mut unvisited = BinaryHeap::from([PathPoint {
        point: start.to_owned(),
        cost: 0,
    }]);
    let mut costs = HashMap::from([(start.to_owned(), 0usize)]);

    while let Some(candidate) = unvisited.pop() {
        let p = &candidate.point;
        let cost = *costs.entry(p.to_owned()).or_insert(usize::MAX);

        if *p == *end {
            return cost;
        }

        let mut adjacent = Vec::new();
        if p.0 > 0 {
            adjacent.push((p.0 - 1, p.1));
        }
        if p.0 + 1 < width {
            adjacent.push((p.0 + 1, p.1));
        }
        if p.1 > 0 {
            adjacent.push((p.0, p.1 - 1));
        }
        if p.1 + 1 < height {
            adjacent.push((p.0, p.1 + 1));
        }
        for adj in &adjacent {
            if heightmap[p.1][p.0] + 1 < heightmap[adj.1][adj.0] {
                continue; // can't climb
            }
            let new_cost = cost + 1;
            let prev_cost = costs.entry(adj.to_owned()).or_insert(usize::MAX);
            if new_cost < *prev_cost {
                // found a shorter path to adjacent point
                *prev_cost = new_cost;
                unvisited.push(PathPoint {
                    point: adj.to_owned(),
                    cost: new_cost,
                });
            }
        }
    }

    usize::MAX
}

type Heightmap = Vec<Vec<u8>>;
type Point = (usize, usize);

#[derive(Eq, PartialEq)]
struct PathPoint {
    point: Point,
    cost: usize,
}

impl Ord for PathPoint {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for PathPoint {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
pub fn test() {
    let input = vec!["Sabqponm", "abcryxxl", "accszExk", "acctuvwj", "abdefghi"];

    assert_eq!(a(&input), "31");
    assert_eq!(b(&input), "29");
}
