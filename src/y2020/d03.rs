//! Day 3: Toboggan Trajectory

/// number of trees in given trajectory
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    count_trees(&grid, &(3, 1)).to_string()
}

/// product of trees encountered on given trajectories
pub fn b(input: &Vec<&str>) -> String {
    const TRAJECTORIES: [Point; 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let grid = parse_input(input);
    TRAJECTORIES
        .iter()
        .map(|t| count_trees(&grid, t))
        .product::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn count_trees(grid: &Grid, trajectory: &Point) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut tree_count = 0;
    let mut pos = (0, 0);

    while pos.1 < height {
        if grid[pos.1][pos.0] {
            tree_count += 1;
        }
        pos.0 = (pos.0 + trajectory.0) % width;
        pos.1 += trajectory.1;
    }

    tree_count
}

type Grid = Vec<Vec<bool>>;
type Point = (usize, usize);

#[test]
pub fn test() {
    let input = vec![
        "..##.......",
        "#...#...#..",
        ".#....#..#.",
        "..#.#...#.#",
        ".#...##..#.",
        "..#.##.....",
        ".#.#.#....#",
        ".#........#",
        "#.##...#...",
        "#...##....#",
        ".#..#...#.#",
    ];

    assert_eq!(a(&input), "7");
    assert_eq!(b(&input), "336");
}
