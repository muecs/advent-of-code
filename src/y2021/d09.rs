//! Day 9: Smoke Basin

use std::collections::BTreeSet;

type Heightmap = Vec<Vec<i8>>;

/// sum of low points + 1
pub fn a(input: &Vec<&str>) -> String {
    let heightmap = parse_input(input);
    get_low_points(&heightmap)
        .iter()
        .fold(0i32, |acc, p| i32::from(heightmap[p.1][p.0]) + 1 + acc)
        .to_string()
}

/// product of sizes of 3 largest basins
pub fn b(input: &Vec<&str>) -> String {
    let heightmap = parse_input(input);
    let mut basin_sizes = get_low_points(&heightmap)
        .iter()
        .map(|p| get_basin_size(&heightmap, p))
        .collect::<Vec<_>>();
    basin_sizes.sort_unstable();
    basin_sizes.reverse();
    basin_sizes
        .iter()
        .take(3)
        .product::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Heightmap {
    input
        .iter()
        .map(|&line| line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
        )
        .collect()
}

/// determines coordinates of local minima
fn get_low_points(heightmap: &Heightmap) -> Vec<(usize, usize)> {
    let height = heightmap.len();
    let width = heightmap[0].len();
    let mut low_points = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let curr = heightmap[y][x];
            if (
                x > 0 && heightmap[y][x - 1] <= curr
            ) || (
                x + 1 < width && heightmap[y][x + 1] <= curr
            ) || (
                y > 0 && heightmap[y - 1][x] <= curr
            ) || (
                y + 1 < height && heightmap[y + 1][x] <= curr
            ) {
                continue
            }
            low_points.push((x, y))
        }
    }
    low_points
}

/// grows point into basin and returns size
fn get_basin_size(heightmap: &Heightmap, low_point: &(usize, usize)) -> usize {
    let height = heightmap.len();
    let width = heightmap[0].len();
    let mut discovered_points = vec![low_point.to_owned()];
    let mut processed_points = BTreeSet::new();
    
    while let Some(point) = discovered_points.pop() {
        if processed_points.contains(&point) {
            continue;
        }

        processed_points.insert(point);

        if point.0 > 0 && heightmap[point.1][point.0 - 1] < 9 {
            discovered_points.push((point.0 - 1, point.1));
        }
        if point.0 + 1 < width && heightmap[point.1][point.0 + 1] < 9 {
            discovered_points.push((point.0 + 1, point.1));
        }
        if point.1 > 0 && heightmap[point.1 - 1][point.0] < 9 {
            discovered_points.push((point.0, point.1 - 1));
        }
        if point.1 + 1 < height && heightmap[point.1 + 1][point.0] < 9 {
            discovered_points.push((point.0, point.1 + 1));
        }
    }

    processed_points.len()
}

#[test]
pub fn test() {
    let input = vec![
        "2199943210",
        "3987894921",
        "9856789892",
        "8767896789",
        "9899965678",
    ];

    assert_eq!(a(&input), "15");
    assert_eq!(b(&input), "1134");
}
