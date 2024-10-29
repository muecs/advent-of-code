//! Day 23: A Long Walk

use std::collections::{HashMap, HashSet, VecDeque};

/// longest path through grid considering slopes
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    find_max_path(&grid, false).to_string()
}

/// longest path through grid ignoring slopes
pub fn b(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    find_max_path(&grid, true).to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|s| {
            s.bytes()
                .map(|b| match b {
                    b'#' => NONE,
                    b'>' => RIGHT,
                    b'v' => DOWN,
                    b'.' => ALL,
                    b'<' | b'^' | _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn find_max_path(grid: &Grid, ignore_slopes: bool) -> usize {
    let dims = (grid[0].len(), grid.len());
    let start: Point = (1, 0);
    let end: Point = (dims.0 - 2, dims.1 - 2);

    // BFS to discover distances between junctions
    // Note: can only go down or right in the directed (sloped) case
    let mut pending = VecDeque::from([(start, (start.0, start.1 + 1))]);
    let mut visited = HashSet::from([start]);
    let mut graph = HashMap::from([
        (start, HashSet::<(Point, usize)>::new()),
        (end, HashSet::<(Point, usize)>::new()),
    ]);
    while let Some((junction, next)) = pending.pop_front() {
        let mut p = next;
        let mut steps = 0;

        loop {
            steps += 1;

            // determine valid neighbours
            let adj = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)]
                .iter()
                .enumerate()
                .filter_map(|(dir, (dx, dy))| {
                    // check grid boundaries
                    if (p.0 == 0 && *dx < 0) || (p.0 + 1 == dims.0 && *dx > 0) {
                        return None;
                    }
                    if (p.1 == 0 && *dy < 0) || (p.1 + 1 == dims.1 && *dy > 0) {
                        return None;
                    }

                    let new_p = (
                        p.0.checked_add_signed(*dx).unwrap(),
                        p.1.checked_add_signed(*dy).unwrap(),
                    );
                    let tile = grid[new_p.1][new_p.0];
                    if tile == NONE {
                        // never accessible
                        return None;
                    }

                    let dir = (1 << dir) as u8;
                    Some((new_p, tile & dir != NONE))
                })
                .collect::<Vec<_>>();

            let mut adj_filtered_it = adj
                .iter()
                .filter_map(|(q, b)| (*b && *q != junction && !visited.contains(q)).then_some(q));
            match adj.len() {
                1 | 2 => {
                    if let Some(q) = adj_filtered_it.next() {
                        visited.insert(p);
                        p = *q;
                        if p == end {
                            // reached destination
                            graph.get_mut(&junction).unwrap().insert((p, steps + 1));
                            break;
                        }
                    } else {
                        break;
                    }
                }
                3 | 4 => {
                    // this tile is a junction, add forward edge from starting junction
                    // println!("junction {p:?} from {junction:?} in {steps} steps");
                    graph.get_mut(&junction).unwrap().insert((p, steps));
                    let new_junction = graph.entry(p).or_default();
                    if ignore_slopes {
                        // when ignoring slopes add backwards edge
                        // TODO: not for 3-way junction along border
                        new_junction.insert((junction, steps));
                    }
                    for q in adj_filtered_it {
                        // enqueue valid paths leading from junction
                        pending.push_back((p, *q));
                    }
                    break;
                }
                _ => unreachable!(),
            };
        }
    }

    // println!("{graph:?}");

    // DFS to find longest path in graph
    let mut stack = vec![(0usize, start, HashSet::from([start]))];
    let mut max_path_length = 0;
    while let Some((path_length, p, visited)) = stack.pop() {
        if p == end {
            // reached destination, update longest path
            max_path_length = max_path_length.max(path_length);
            // println!("{max_path_length} {}", visited.len());
            continue;
        }

        // traverse adjacent nodes
        for (q, steps) in graph.get(&p).unwrap() {
            if visited.contains(q) {
                // already visited
                continue;
            }

            let mut new_visited = visited.to_owned();
            new_visited.insert(*q);
            stack.push((path_length + steps, *q, new_visited));
        }
    }

    max_path_length + 1
}

// permitted directions
const NONE: u8 = 0;
const RIGHT: u8 = 0b0001;
const DOWN: u8 = 0b0010;
// const LEFT: u8 = 0b0100;
// const UP: u8 = 0b1000;
const ALL: u8 = 0b1111;

type Grid = Vec<Vec<u8>>;
type Point = (usize, usize);

#[test]
pub fn test() {
    let input = vec![
        "#.#####################",
        "#.......#########...###",
        "#######.#########.#.###",
        "###.....#.>.>.###.#.###",
        "###v#####.#v#.###.#.###",
        "###.>...#.#.#.....#...#",
        "###v###.#.#.#########.#",
        "###...#.#.#.......#...#",
        "#####.#.#.#######.#.###",
        "#.....#.#.#.......#...#",
        "#.#####.#.#.#########v#",
        "#.#...#...#...###...>.#",
        "#.#.#v#######v###.###v#",
        "#...#.>.#...>.>.#.###.#",
        "#####v#.#.###v#.#.###.#",
        "#.....#...#...#.#.#...#",
        "#.#########.###.#.#.###",
        "#...###...#...#...#.###",
        "###.###.#.###v#####v###",
        "#...#...#.#.>.>.#.>.###",
        "#.###.###.#.###.#.#v###",
        "#.....###...###...#...#",
        "#####################.#",
    ];

    assert_eq!(a(&input), "94");
    assert_eq!(b(&input), "154");
}
