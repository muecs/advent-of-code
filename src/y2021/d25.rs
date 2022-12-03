//! Day 25: Sea Cucumber

/// steps until deadlock
pub fn a(input: &Vec<&str>) -> String {
    let mut grid = parse_input(input);

    let mut steps = 0usize;
    loop {
        let moves =
            advance_herd(&mut grid, HerdType::East) + 
            advance_herd(&mut grid, HerdType::South);
        steps += 1;
        if moves == 0 {
            break;
        }
    }

    steps.to_string()
}

/// n/a
pub fn b(_input: &Vec<&str>) -> String {
    String::new()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input
        .iter()
        .map(|s| s.chars().map(|c| c.into()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn advance_herd(grid: &mut Grid, herd_type: HerdType) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let dx = (herd_type == HerdType::East) as usize;
    let dy = (herd_type == HerdType::South) as usize;

    let mut candidates = Vec::<Point>::new();
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == herd_type
                && grid[(y + dy) % height][(x + dx) % width] == HerdType::Empty
            {
                candidates.push((x, y));
            }
        }
    }

    let moves = candidates.len();

    for (x, y) in candidates {
        grid[y][x] = HerdType::Empty;
        grid[(y + dy) % height][(x + dx) % width] = herd_type;
    }

    moves
}

#[derive(Clone, Copy, PartialEq)]
enum HerdType {
    Empty,
    East,
    South,
}

impl From<char> for HerdType {
    fn from(c: char) -> Self {
        match c {
            '>' => HerdType::East,
            'v' => HerdType::South,
            _ => HerdType::Empty,
        }
    }
}

type Grid = Vec<Vec<HerdType>>;
type Point = (usize, usize);

#[test]
pub fn test() {
    let input = vec![
        "v...>>.vv>",
        ".vv>>.vv..",
        ">>.>v>...v",
        ">>v>>.>.v.",
        "v>v.vv.v..",
        ">.>>..v...",
        ".vv..>.>v.",
        "v.v..>>v.v",
        "....v..v.>",
    ];

    assert_eq!(a(&input), "58");
    assert_eq!(b(&input), "");
}
