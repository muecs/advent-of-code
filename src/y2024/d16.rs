//! Day 16: Reindeer Maze

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

/// score of cheapest path through maze
pub fn a(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    find_path(&map, false).to_string()
}

/// number of tiles along all cheapest paths
pub fn b(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    find_path(&map, true).to_string()
}

fn parse_input(input: &Vec<&str>) -> Map {
    input
        .iter()
        .map(|s| s.bytes().map(|b| b != b'#').collect())
        .collect()
}

/// determines cheapest path
fn find_path(map: &Map, count_tiles: bool) -> usize {
    let start = (1, map.len() - 2); // bottom left
    let end = (map[0].len() - 2, 1); // top right
    let mut unvisited = BinaryHeap::from([(Reverse(0usize), start, 1, vec![start])]);
    let mut scores = HashMap::new();
    let mut min_score = usize::MAX;
    let mut paths = Vec::new();

    while let Some((score, pos, dir, path)) = unvisited.pop() {
        if pos == end {
            if score.0 < min_score {
                min_score = score.0;
                paths = vec![path.to_owned()];
            } else if count_tiles && score.0 == min_score {
                paths.push(path.to_owned());
            } else {
                break;
            }
        }

        scores.insert((pos, dir), score.0);

        // movement options: left, forward, right
        for d in 3..=5 {
            let next_dir = (dir + d) % 4;
            let next_pos = (
                pos.0.checked_add_signed([0, 1, 0, -1][next_dir]).unwrap(),
                pos.1.checked_add_signed([-1, 0, 1, 0][next_dir]).unwrap(),
            );
            let next_score = score.0 + if next_dir == dir { 1 } else { 1001 };

            if map[next_pos.1][next_pos.0]
                && scores
                    .get(&(next_pos, next_dir))
                    .is_none_or(|&prev_score| next_score < prev_score)
            {
                // found a cheaper path to adjacent tile
                unvisited.push((
                    Reverse(next_score),
                    next_pos,
                    next_dir,
                    [&path[..], &[next_pos]].concat(),
                ));
            }
        }
    }

    if count_tiles {
        paths
            .iter()
            .flat_map(|path| path.iter().map(|p| (p.0, p.1)))
            .collect::<HashSet<_>>()
            .len()
    } else {
        min_score
    }
}

type Map = Vec<Vec<bool>>;

#[test]
pub fn test() {
    let input1 = vec![
        "###############",
        "#.......#....E#",
        "#.#.###.#.###.#",
        "#.....#.#...#.#",
        "#.###.#####.#.#",
        "#.#.#.......#.#",
        "#.#.#####.###.#",
        "#...........#.#",
        "###.#.#####.#.#",
        "#...#.....#.#.#",
        "#.#.#.###.#.#.#",
        "#.....#...#.#.#",
        "#.###.#.#.#.#.#",
        "#S..#.....#...#",
        "###############",
    ];

    let input2 = vec![
        "#################",
        "#...#...#...#..E#",
        "#.#.#.#.#.#.#.#.#",
        "#.#.#.#...#...#.#",
        "#.#.#.#.###.#.#.#",
        "#...#.#.#.....#.#",
        "#.#.#.#.#.#####.#",
        "#.#...#.#.#.....#",
        "#.#.#####.#.###.#",
        "#.#.#.......#...#",
        "#.#.###.#####.###",
        "#.#.#...#.....#.#",
        "#.#.#.#####.###.#",
        "#.#.#.........#.#",
        "#.#.#.#########.#",
        "#S#.............#",
        "#################",
    ];

    assert_eq!(a(&input1), "7036");
    assert_eq!(a(&input2), "11048");
    assert_eq!(b(&input1), "45");
    assert_eq!(b(&input2), "64");
}
