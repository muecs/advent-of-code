//! Day 15: Chiton
 
type Grid = Vec<Vec<usize>>;
type Point = (usize, usize);
// type Points = Vec<Point>;

/// lowest total risk of any path from the top left to the bottom right
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    let score = find_optimal_path_score(&grid, 1);
    score.to_string()
}

/// lowest risk path through 5x5 repeated map
pub fn b(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    let score = find_optimal_path_score(&grid, 5);
    score.to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|&line| line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect()
        )
        .collect()
}

fn find_optimal_path_score(grid: &Grid, repeats: usize) -> usize {
    assert!(repeats > 0);
    let height = grid.len();
    let width = grid[0].len();

    let mut unvisited = vec![(0usize, 0usize)];
    let mut scores = vec![vec![usize::MAX; width * repeats]; height * repeats];
    scores[0][0] = 0;

    while !unvisited.is_empty() {
        // poor man's priority queue
        unvisited.sort_unstable_by(
            |a, b| scores[b.1][b.0].cmp(&scores[a.1][a.0])
        );
        let point = unvisited.pop().unwrap();
        let curr_score = scores[point.1][point.0];

        if point.0 + 1 == width * repeats && point.1 + 1 == height * repeats {
            // arrived at destination
            return curr_score;
        }

        // list valid adjacent points
        let mut adj_points = Vec::new();
        if point.0 > 0 {
            adj_points.push((point.0 - 1, point.1));
        }
        if point.0 + 1 < width * repeats {
            adj_points.push((point.0 + 1, point.1));
        }
        if point.1 > 0 {
            adj_points.push((point.0, point.1 - 1));
        }
        if point.1 + 1 < height * repeats {
            adj_points.push((point.0, point.1 + 1));
        }

        for adj_point in &adj_points {
            let new_score = curr_score + get_risk(grid, &adj_point);
            if new_score < scores[adj_point.1][adj_point.0] {
                // found a better path to adjacent point
                scores[adj_point.1][adj_point.0] = new_score;
                if !unvisited.contains(adj_point) {
                    unvisited.push(*adj_point);
                }
            }
        }
    }

    usize::MAX
}

fn get_risk(grid: &Grid, point: &Point) -> usize {
    let height = grid.len();
    let width = grid[0].len();

    let x = point.0 % width;
    let y = point.1 % height;
    let tile = point.0 / width + point.1 / height;

    (grid[y][x] + tile - 1) % 9 + 1
}

#[test]
pub fn test() {
    let input = vec![
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    let grid = parse_input(&input);
    assert_eq!(get_risk(&grid, &( 0,  0)), 1);
    assert_eq!(get_risk(&grid, &(10,  0)), 2);
    assert_eq!(get_risk(&grid, &( 0, 10)), 2);
    assert_eq!(get_risk(&grid, &(10, 10)), 3);
    assert_eq!(get_risk(&grid, &(20,  0)), 3);
    assert_eq!(get_risk(&grid, &(20, 10)), 4);
    assert_eq!(get_risk(&grid, &( 0, 20)), 3);
    assert_eq!(get_risk(&grid, &(10, 20)), 4);
    assert_eq!(get_risk(&grid, &(20, 20)), 5);
    assert_eq!(get_risk(&grid, &( 2,  1)), 8);
    assert_eq!(get_risk(&grid, &(12,  1)), 9);
    assert_eq!(get_risk(&grid, &(22,  1)), 1);
    assert_eq!(get_risk(&grid, &(32,  1)), 2);

    assert_eq!(a(&input), "40");
    assert_eq!(b(&input), "315");
}
