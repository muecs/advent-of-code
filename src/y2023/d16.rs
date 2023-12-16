//! Day 16: The Floor Will Be Lava

/// number of visited tiles
pub fn a(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    count_energized(0, 0, RIGHT, &grid).to_string()
}

/// max number of visited tiles from all directions
pub fn b(input: &Vec<&str>) -> String {
    let grid = parse_input(input);
    assert_eq!(grid.width, grid.height);
    (0..grid.width)
        .map(|i| {
            usize::max(
                usize::max(
                    count_energized(i, 0, DOWN, &grid),
                    count_energized(i, grid.height - 1, UP, &grid),
                ),
                usize::max(
                    count_energized(0, i, RIGHT, &grid),
                    count_energized(grid.width - 1, i, LEFT, &grid),
                ),
            )
        })
        .max()
        .unwrap()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    Grid {
        tiles: input.iter().map(|s| s.bytes()).flatten().collect(),
        width: input[0].len(),
        height: input.len(),
    }
}

fn trace(mut x: usize, mut y: usize, mut dir: u8, grid: &Grid, visited: &mut Vec<u8>) {
    loop {
        let i = x + y * grid.width;
        // println!(
        //     "{x:-2},{y:-2} {} - {dir} {:?}",
        //     grid.tiles[i] as char,
        //     visited[i] & dir != 0
        // );
        if visited[i] & dir != 0 {
            // already been on this tile in this direction
            break;
        }
        visited[i] |= dir;
        dir = match grid.tiles[i] {
            b'/' if dir == RIGHT => UP,
            b'/' if dir == DOWN => LEFT,
            b'/' if dir == LEFT => DOWN,
            b'/' if dir == UP => RIGHT,
            b'\\' if dir == RIGHT => DOWN,
            b'\\' if dir == DOWN => RIGHT,
            b'\\' if dir == LEFT => UP,
            b'\\' if dir == UP => LEFT,
            b'|' if dir == RIGHT || dir == LEFT => {
                if y == 0 {
                    DOWN
                } else if y + 1 == grid.height {
                    UP
                } else {
                    // split
                    trace(x, y - 1, UP, grid, visited);
                    DOWN
                }
            }
            b'-' if dir == DOWN || dir == UP => {
                if x == 0 {
                    RIGHT
                } else if x + 1 == grid.width {
                    LEFT
                } else {
                    // split
                    trace(x - 1, y, LEFT, grid, visited);
                    RIGHT
                }
            }
            _ => dir, // keep going
        };
        match dir {
            RIGHT if x + 1 < grid.width => x += 1,
            DOWN if y + 1 < grid.height => y += 1,
            LEFT if x > 0 => x -= 1,
            UP if y > 0 => y -= 1,
            _ => break, // left grid
        }
    }
    // println!("terminated");
}

fn count_energized(x: usize, y: usize, dir: u8, grid: &Grid) -> usize {
    let mut visited = vec![0u8; grid.tiles.len()];
    trace(x, y, dir, &grid, &mut visited);
    visited.iter().filter(|&&v| v > 0).count()
}

const RIGHT: u8 = 1;
const DOWN: u8 = 2;
const LEFT: u8 = 4;
const UP: u8 = 8;

struct Grid {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

#[test]
pub fn test() {
    let input = vec![
        ".|...\\....",
        "|.-.\\.....",
        ".....|-...",
        "........|.",
        "..........",
        ".........\\",
        "..../.\\\\..",
        ".-.-/..|..",
        ".|....-|.\\",
        "..//.|....",
    ];

    assert_eq!(a(&input), "46");
    assert_eq!(b(&input), "51");
}
