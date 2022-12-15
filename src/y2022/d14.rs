//! Day 14: Regolith Reservoir

/// number of simulation steps until overflow
pub fn a(input: &Vec<&str>) -> String {
    let (mut grid, emitter) = parse_input(input, false);
    let mut steps = 0usize;
    while simulate(emitter, &mut grid) { steps += 1; }
    steps.to_string()
}

/// number of simulation steps until source blocked
pub fn b(input: &Vec<&str>) -> String {
    let (mut grid, emitter) = parse_input(input, true);
    let mut steps = 0usize;
    while simulate(emitter, &mut grid) { steps += 1; }
    steps.to_string()
}

fn parse_input(input: &Vec<&str>, infinite: bool) -> (Grid, usize) {
    const EMITTER: usize = 500;

    let mut paths = input
        .iter()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.split_once(',').unwrap())
                .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let (mut x_min, mut x_max, mut y_max) = paths
        .iter()
        .flatten()
        .fold((usize::MAX, usize::MIN, usize::MIN), |acc, p| {
            (acc.0.min(p.0), acc.1.max(p.0), acc.2.max(p.1))
        });
    
    if infinite {
        y_max += 2;
        x_min = x_min.min(EMITTER - y_max - 1);
        x_max = x_max.max(EMITTER + y_max + 1);
        paths.push(vec![(x_min, y_max), (x_max, y_max)]);  // floor
    }

    let mut grid = vec![vec![false; x_max - x_min + 3]; y_max + 1];
    for path in paths {
        for points in path.windows(2) {
            draw_line(&points[0], &points[1], x_min - 1, &mut grid);
        }
    }

    (grid, EMITTER + 1 - x_min)
}

fn draw_line(p1: &Point, p2: &Point, x_offset: usize, grid: &mut Grid) {
    let x_min = p1.0.min(p2.0) - x_offset;
    let x_max = p1.0.max(p2.0) - x_offset;
    for y in p1.1.min(p2.1)..=p1.1.max(p2.1) {
        for x in x_min..=x_max {
            grid[y][x] = true;
        }
    }
}

/// emits a single sand unit and simulates flow until rest or boundary reached
fn simulate(emitter: usize, grid: &mut Grid) -> bool {
    let mut p: Point = (emitter, 0);
    while p.1 + 1 < grid.len() {
        if !grid[p.1 + 1][p.0] {
            // move down
            p.1 += 1;
        } else if !grid[p.1 + 1][p.0 - 1] {
            // move down-left
            p.0 -= 1;
            p.1 += 1;
        } else if !grid[p.1 + 1][p.0 + 1] {
            // move down-right
            p.0 += 1;
            p.1 += 1;
        } else if !grid[p.1][p.0] {
            // nowhere to move; rest and terminate
            grid[p.1][p.0] = true;
            return true;
        } else {
            // we're full; terminate
            return false;
        }
    }

    false
}

type Point = (usize, usize);
type Grid = Vec<Vec<bool>>;

#[test]
pub fn test() {
    let input = vec![
        "498,4 -> 498,6 -> 496,6",
        "503,4 -> 502,4 -> 502,9 -> 494,9",
    ];

    assert_eq!(a(&input), "24");
    assert_eq!(b(&input), "93");
}
