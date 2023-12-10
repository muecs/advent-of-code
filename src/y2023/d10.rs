//! Day 10: Pipe Maze

/// path length to middle of loop
pub fn a(input: &Vec<&str>) -> String {
    let maze = parse_input(input);
    let path = build_path(&maze);
    (path.len() / 2).to_string()
}

/// number of tiles enclosed by the loop
pub fn b(input: &Vec<&str>) -> String {
    let maze = parse_input(input);
    let path = build_path(&maze);
    calc_area(&path).to_string()
}

fn parse_input(input: &Vec<&str>) -> Maze {
    input.iter().map(|s| s.bytes().collect()).collect()
}

fn build_path(maze: &Maze) -> Path {
    let mut path = vec![maze
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &b)| (b == b'S').then_some((x, y)))
        })
        .unwrap()];
    loop {
        let (x, y) = path.last().unwrap();
        let tile = maze[*y][*x];
        if path.len() >= 4 && tile == b'S' {
            break; // loop closed
        }
        let options: Vec<(isize, isize)> = match tile {
            b'|' => vec![(0, -1), (0, 1)],  // go north or south
            b'-' => vec![(-1, 0), (1, 0)],  // go west or east
            b'L' => vec![(0, -1), (1, 0)],  // go north or east
            b'J' => vec![(-1, 0), (0, -1)], // go west or north
            b'7' => vec![(-1, 0), (0, 1)],  // go west or south
            b'F' => vec![(1, 0), (0, 1)],   // go east or south
            b'S' => {
                // pick just one adjacent tile that connects to start
                if *x > 0 && [b'-', b'L', b'F'].contains(&maze[*y][x - 1]) {
                    vec![(-1, 0)] // go west
                } else if x + 1 < maze[0].len() && [b'-', b'J', b'7'].contains(&maze[*y][x + 1]) {
                    vec![(1, 0)] // go east
                } else if *y > 0 && [b'|', b'7', b'F'].contains(&maze[*y - 1][*x]) {
                    vec![(0, -1)] // go north
                } else if y + 1 < maze.len() && [b'|', b'L', b'J'].contains(&maze[y + 1][*x]) {
                    vec![(0, 1)] // go south
                } else {
                    unreachable!()
                }
            }
            _ => unreachable!(),
        };

        path.push(
            options
                .iter()
                .find_map(|(dx, dy)| {
                    let new_pos = ((*x as isize + *dx) as usize, (*y as isize + *dy) as usize);
                    (path.len() < 2 || path[path.len() - 2] != new_pos).then_some(new_pos)
                })
                .unwrap(),
        );
    }
    path
}

fn calc_area(path: &Path) -> usize {
    let mut area = 0;
    for i in 0..path.len() {
        let (x0, y0) = path[i];
        let (x1, y1) = path[(i + 1) % path.len()];

        // signed area north of (central line through) horizontal tiles
        if y0 == y1 {
            area += (x1 as isize - x0 as isize) * y0 as isize;
        }
    }

    // account for pipe thickness
    area.abs() as usize - path.len() / 2 + 1
}

type Maze = Vec<Vec<u8>>;
type Path = Vec<(usize, usize)>;

#[test]
pub fn test() {
    let input = vec!["7-F7-", ".FJ|7", "SJLL7", "|F--J", "LJ.LJ"];

    let input2 = vec![
        "...........",
        ".S-------7.",
        ".|F-----7|.",
        ".||.....||.",
        ".||.....||.",
        ".|L-7.F-J|.",
        ".|..|.|..|.",
        ".L--J.L--J.",
        "...........",
    ];

    let input3 = vec![
        ".F----7F7F7F7F-7....",
        ".|F--7||||||||FJ....",
        ".||.FJ||||||||L7....",
        "FJL7L7LJLJ||LJ.L-7..",
        "L--J.L7...LJS7F-7L7.",
        "....F-J..F7FJ|L7L7L7",
        "....L7.F7||L7|.L7L7|",
        ".....|FJLJ|FJ|F7|.LJ",
        "....FJL-7.||.||||...",
        "....L---J.LJ.LJLJ...",
    ];

    let input4 = vec![
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];

    assert_eq!(a(&input), "8");
    assert_eq!(b(&input), "1");
    assert_eq!(b(&input2), "4");
    assert_eq!(b(&input3), "8");
    assert_eq!(b(&input4), "10");
}
