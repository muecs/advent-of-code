//! Day 11: Seating System

/// number of occupied seats at equilibrium, considering adjacency
pub fn a(input: &Vec<&str>) -> String {
    let mut grid = parse_input(input);
    while simulate(&mut grid, false) > 0 {}
    count_occupied(&grid).to_string()
}

/// number of occupied seats at equilibrium, considering visibility
pub fn b(input: &Vec<&str>) -> String {
    let mut grid = parse_input(input);
    while simulate(&mut grid, true) > 0 {}
    count_occupied(&grid).to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => Seat::None,
                    'L' => Seat::Empty,
                    '#' => Seat::Occupied,
                    _ => unreachable!("invalid position symbol"),
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn count_adjacent_occupied(grid: &Grid, x: usize, y: usize, visible: bool) -> usize {
    let height = grid.len() as isize;
    let width = grid[0].len() as isize;
    let mut count = 0;
    for j in -1..=1 {
        for i in -1..=1 {
            if i == 0 && j == 0 {
                continue;
            }
            let (mut x, mut y) = (x as isize, y as isize);
            loop {
                x += i;
                y += j;
                if x < 0 || y < 0 || x >= width || y >= height {
                    break;
                }
                match grid[y as usize][x as usize] {
                    Seat::None => if !visible { break; },
                    Seat::Empty => break,
                    Seat::Occupied => { count += 1; break; },
                    
                }
            }
        }
    }
    count
}

fn count_occupied(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .map(|&seat| (seat == Seat::Occupied) as usize)
                .sum::<usize>()
        })
        .sum()
}

fn simulate(grid: &mut Grid, visible: bool) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let tolerance = if visible { 5 } else { 4 };
    let mut next_grid = grid.clone();
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            let occ = count_adjacent_occupied(grid, x, y, visible);
            next_grid[y][x] = match &grid[y][x] {
                Seat::Empty if occ == 0 => {
                    count += 1;
                    Seat::Occupied
                }
                Seat::Occupied if occ >= tolerance => {
                    count += 1;
                    Seat::Empty
                }
                seat => *seat,
            };
        }
    }
    *grid = next_grid;
    count
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Seat {
    None,
    Empty,
    Occupied,
}

type Grid = Vec<Vec<Seat>>;

#[test]
pub fn test() {
    let input = vec![
        "L.LL.LL.LL",
        "LLLLLLL.LL",
        "L.L.L..L..",
        "LLLL.LL.LL",
        "L.LL.LL.LL",
        "L.LLLLL.LL",
        "..L.L.....",
        "LLLLLLLLLL",
        "L.LLLLLL.L",
        "L.LLLLL.LL",
    ];

    let input_first = vec![
        "#.##.##.##",
        "#######.##",
        "#.#.#..#..",
        "####.##.##",
        "#.##.##.##",
        "#.#####.##",
        "..#.#.....",
        "##########",
        "#.######.#",
        "#.#####.##",
    ];

    {
        let input_second = vec![
            "#.LL.L#.##",
            "#LLLLLL.L#",
            "L.L.L..L..",
            "#LLL.LL.L#",
            "#.LL.LL.LL",
            "#.LLLL#.##",
            "..L.L.....",
            "#LLLLLLLL#",
            "#.LLLLLL.L",
            "#.#LLLL.##",
        ];

        let mut grid = parse_input(&input);
        assert_eq!(simulate(&mut grid, false), 71);
        assert_eq!(grid, parse_input(&input_first));
        assert_eq!(simulate(&mut grid, false), 51);
        assert_eq!(grid, parse_input(&input_second));
    }

    assert_eq!(a(&input), "37");

    {
        let input_second = vec![
            "#.LL.LL.L#",
            "#LLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLL#",
            "#.LLLLLL.L",
            "#.LLLLL.L#",
        ];

        let input_third = vec![
            "#.L#.##.L#",
            "#L#####.LL",
            "L.#.#..#..",
            "##L#.##.##",
            "#.##.#L.##",
            "#.#####.#L",
            "..#.#.....",
            "LLL####LL#",
            "#.L#####.L",
            "#.L####.L#",
        ];

        let mut grid = parse_input(&input);
        assert_eq!(simulate(&mut grid, true), 71);
        assert_eq!(grid, parse_input(&input_first));
        assert_eq!(simulate(&mut grid, true), 64);
        assert_eq!(grid, parse_input(&input_second));
        assert_eq!(simulate(&mut grid, true), 46);
        assert_eq!(grid, parse_input(&input_third));
    }

    assert_eq!(b(&input), "26");
}
