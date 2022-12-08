//! Day 8: Treetop Tree House

use std::collections::BTreeSet;

/// grid points not shadowed by higher numbers as seen from outside
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();

    let mut visible = BTreeSet::new();
    for y in 0..height {
        let mut max = (-1, -1);
        for x in 0..width {
            if grid[y][x] > max.0 {
                // visible from west
                max.0 = grid[y][x];
                visible.insert((x, y));
            }
            let x2 = width - 1 - x;
            if grid[y][x2] > max.1 {
                // visible from east
                max.1 = grid[y][x2];
                visible.insert((x2, y));
            }
        }
    }

    for x in 0..width {
        let mut max = (-1, -1);
        for y in 0..height {
            if grid[y][x] > max.0 {
                // visible from north
                max.0 = grid[y][x];
                visible.insert((x, y));
            }
            let y2 = height - 1 - y;
            if grid[y2][x] > max.1 {
                // visible from south
                max.1 = grid[y2][x];
                visible.insert((x, y2));
            }
        }
    }

    visible.len().to_string()
}

/// max product of distances until higher value per grid point
pub fn b(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    let height = grid.len();
    let width = grid[0].len();
    let mut max_score = 0;

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let curr = grid[y][x];
            let mut score = 1;

            let mut dist = 1;
            while x + dist + 1 < width && grid[y][x + dist] < curr {
                dist += 1
            }
            score *= dist;

            dist = 1;
            while x > dist && grid[y][x - dist] < curr {
                dist += 1
            }
            score *= dist;

            dist = 1;
            while y + dist + 1 < height && grid[y + dist][x] < curr {
                dist += 1
            }
            score *= dist;

            dist = 1;
            while y > dist && grid[y - dist][x] < curr {
                dist += 1
            }
            score *= dist;

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score.to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|s| s.bytes().map(|b| (b - b'0') as i8).collect())
        .collect()
}

type Grid = Vec<Vec<i8>>;

#[test]
pub fn test() {
    let input = vec!["30373", "25512", "65332", "33549", "35390"];

    assert_eq!(a(&input), "21");
    assert_eq!(b(&input), "8");
}
