//! Day 11: Cosmic Expansion

use std::collections::HashMap;

/// sum of distances with 2x expansion
pub fn a(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    let distances = calc_distances(&map, 2);
    distances.values().sum::<usize>().to_string()
}

/// sum of distances with 1000000x expansion
pub fn b(input: &Vec<&str>) -> String {
    let map = parse_input(input);
    let distances = calc_distances(&map, 1000000);
    distances.values().sum::<usize>().to_string()
}

fn parse_input(input: &Vec<&str>) -> Map {
    let mut row_empty = vec![true; input.len()];
    let mut col_empty = vec![true; input[0].len()];
    let mut stars = Vec::<(usize, usize)>::new();

    for y in 0..input.len() {
        for (x, b) in input[y].bytes().enumerate() {
            if b == b'#' {
                stars.push((x, y));
                col_empty[x] = false;
                row_empty[y] = false;
            }
        }
    }

    Map {
        stars,
        col_empty,
        row_empty,
    }
}

fn calc_distances(map: &Map, expansion_factor: usize) -> Distances {
    assert!(expansion_factor > 0);
    let mut distances = Distances::new();
    for i in 0..map.stars.len() - 1 {
        for j in i + 1..map.stars.len() {
            let x0 = map.stars[i].0.min(map.stars[j].0);
            let x1 = map.stars[i].0.max(map.stars[j].0);
            let y0 = map.stars[i].1.min(map.stars[j].1);
            let y1 = map.stars[i].1.max(map.stars[j].1);
            let x_dist = x1 - x0
                + (x0 + 1..x1).filter(|&x| map.col_empty[x]).count() * (expansion_factor - 1);
            let y_dist = y1 - y0
                + (y0 + 1..y1).filter(|&y| map.row_empty[y]).count() * (expansion_factor - 1);
            distances.insert((i, j), x_dist + y_dist); // Manhattan
        }
    }
    distances
}

struct Map {
    stars: Vec<(usize, usize)>,
    col_empty: Vec<bool>,
    row_empty: Vec<bool>,
}

type Distances = HashMap<(usize, usize), usize>;

#[test]
pub fn test() {
    let input = vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    let map = parse_input(&input);
    assert_eq!(map.stars.len(), 9);
    assert_eq!(map.stars[0], (3, 0));
    assert_eq!(
        map.col_empty,
        [false, false, true, false, false, true, false, false, true, false]
    );
    assert_eq!(
        map.row_empty,
        [false, false, false, true, false, false, false, true, false, false]
    );

    let distances = calc_distances(&map, 2);
    assert_eq!(distances.len(), 36);
    assert_eq!(distances[&(4, 8)], 9);
    assert_eq!(distances[&(0, 6)], 15);
    assert_eq!(distances[&(2, 5)], 17);
    assert_eq!(distances[&(7, 8)], 5);

    assert_eq!(a(&input), "374");

    let distances10 = calc_distances(&map, 10);
    assert_eq!(distances10[&(7, 8)], 4 + 10 - 1);
    assert_eq!(distances10.values().sum::<usize>(), 1030);

    let distances100 = calc_distances(&map, 100);
    assert_eq!(distances100[&(7, 8)], 4 + 100 - 1);
    assert_eq!(distances100.values().sum::<usize>(), 8410);

    // assert_eq!(b(&input), "");
}
