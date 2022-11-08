//! Day 11: Dumbo Octopus

type Grid = Vec<Vec<i8>>;
type Point = (usize, usize);
type Points = Vec<Point>;

/// number of flashes after 100 steps
pub fn a(input: &Vec<&str>) -> String {
    let mut grid = parse_input(input);
    let flashes = process(&mut grid, 100);
    flashes.to_string()
}

/// steps until synchronized flash
pub fn b(input: &Vec<&str>) -> String {
    let mut grid = parse_input(input);
    let steps = process(&mut grid, usize::MAX);
    steps.to_string()
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

/// executes simulation steps
fn process(grid: &mut Grid, steps: usize) -> usize {
    let mut total_flash_count = 0;

    for step in 1..=steps {
        let mut flash_points = Points::new();
        for x in 0..=9 {
            for y in 0..=9 {
                increment_point(&(x, y), grid, &mut flash_points);
            }
        }

        while let Some((x, y)) = flash_points.pop() {
            if y > 0 {
                if x > 0 {
                    increment_point(&(x - 1, y - 1), grid, &mut flash_points);
                }
                increment_point(&(x, y - 1), grid, &mut flash_points);
                if x < 9 {
                    increment_point(&(x + 1, y - 1), grid, &mut flash_points);
                }
            }
            if x > 0 {
                increment_point(&(x - 1, y), grid, &mut flash_points);
            }
            if x < 9 {
                increment_point(&(x + 1, y), grid, &mut flash_points);
            }
            if y < 9 {
                if x > 0 {
                    increment_point(&(x - 1, y + 1), grid, &mut flash_points);
                }
                increment_point(&(x, y + 1), grid, &mut flash_points);
                if x < 9 {
                    increment_point(&(x + 1, y + 1), grid, &mut flash_points);
                }
            }
        } 

        let mut flash_count = 0;
        for x in 0..=9 {
            for y in 0..=9 {
                if grid[x][y] > 9 {
                    grid[x][y] = 0;
                    flash_count += 1;
                }
            }
        }
        if flash_count == 100 {
            // first synchronized flash
            return step;
        }
        total_flash_count += flash_count;
    }

    total_flash_count
}

fn increment_point(point: &Point, grid: &mut Grid, flash_points: &mut Points) {
    grid[point.0][point.1] += 1;

    if grid[point.0][point.1] == 10 {
        flash_points.push(*point);
    }
}

#[test]
pub fn test() {
    let input = vec![
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    let mut grid = parse_input(&input);
    assert_eq!(process(&mut grid, 10), 204);

    assert_eq!(a(&input), "1656");
    assert_eq!(b(&input), "195");
}
