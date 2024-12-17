//! Day 15: Warehouse Woes

/// box coords after applying movements
pub fn a(input: &Vec<&str>) -> String {
    let (mut map, start, moves) = parse_input(input, false);
    simulate(&mut map, &start, &moves).to_string()
}

/// box coords in horizontally duplicated map after applying movements
pub fn b(input: &Vec<&str>) -> String {
    let (mut map, start, moves) = parse_input(input, true);
    simulate(&mut map, &start, &moves).to_string()
}

fn parse_input(input: &Vec<&str>, duplicate: bool) -> (Map, Pos, Moves) {
    let mut start = (0, 0);
    let mut it = input.iter();
    let map = it
        .by_ref()
        .enumerate()
        .map_while(|(y, s)| {
            let start_ref = &mut start;
            (!s.is_empty()).then(move || {
                s.bytes()
                    .enumerate()
                    .flat_map(move |(x, b)| {
                        let tile = match b {
                            b'.' => Tile::Empty,
                            b'#' => Tile::Wall,
                            b'O' => Tile::Box,
                            b'@' => {
                                *start_ref = (x, y);
                                Tile::Empty
                            }
                            _ => unreachable!(),
                        };
                        if !duplicate {
                            vec![tile; 1]
                        } else if tile != Tile::Box {
                            vec![tile; 2]
                        } else {
                            vec![Tile::BoxL, Tile::BoxR]
                        }
                        .into_iter()
                    })
                    .collect()
            })
        })
        .collect();
    let moves = it
        .flat_map(|s| {
            s.bytes().map(|b| match b {
                b'^' => 0,
                b'>' => 1,
                b'v' => 2,
                b'<' => 3,
                _ => unreachable!(),
            })
        })
        .collect();
    if duplicate {
        start.0 *= 2;
    }
    (map, start, moves)
}

fn simulate(map: &mut Map, start: &Pos, moves: &Moves) -> usize {
    let mut pos = *start;
    for dir in moves {
        let dx = [0, 1, 0, -1][*dir as usize];
        let dy = [-1, 0, 1, 0][*dir as usize];
        if try_move(map, &pos, dx, dy, false) {
            pos = (
                pos.0.checked_add_signed(dx).unwrap(),
                pos.1.checked_add_signed(dy).unwrap(),
            );
        }
        // for (y, row) in map.iter().enumerate() {
        //     println!(
        //         "{}",
        //         row.iter()
        //             .enumerate()
        //             .map(|(x, tile)| if (x, y) == pos {
        //                 '@'
        //             } else {
        //                 match tile {
        //                     Tile::Empty => '.',
        //                     Tile::Wall => '#',
        //                     Tile::Box => 'O',
        //                     Tile::BoxL => '[',
        //                     Tile::BoxR => ']',
        //                 }
        //             })
        //             .collect::<String>()
        //     );
        // }
    }
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, tile)| {
                if [Tile::Box, Tile::BoxL].contains(tile) {
                    x + 100 * y
                } else {
                    0
                }
            })
        })
        .sum()
}

fn try_move(map: &mut Map, pos: &Pos, dx: isize, dy: isize, dryrun: bool) -> bool {
    if !dryrun && !try_move(map, pos, dx, dy, true) {
        return false;
    }

    let mut pending = vec![*pos];
    {
        let tile = map[pos.1][pos.0];
        if dy != 0 {
            if tile == Tile::BoxL {
                pending.push((pos.0 + 1, pos.1));
            } else if tile == Tile::BoxR {
                pending.push((pos.0 - 1, pos.1));
            }
        }
    }

    for p in pending {
        let next = (
            p.0.checked_add_signed(dx).unwrap(),
            p.1.checked_add_signed(dy).unwrap(),
        );
        let next_tile = map[next.1][next.0];
        if dryrun {
            if next_tile != Tile::Empty
                && (next_tile == Tile::Wall || !try_move(map, &next, dx, dy, true))
            {
                return false;
            }
        } else {
            if [Tile::Box, Tile::BoxL, Tile::BoxR].contains(&next_tile) {
                try_move(map, &next, dx, dy, false);
            }
            map[next.1][next.0] = map[p.1][p.0];
            map[p.1][p.0] = Tile::Empty;
        }
    }

    true
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Box,
    BoxL,
    BoxR,
}
type Map = Vec<Vec<Tile>>;
type Moves = Vec<u8>;
type Pos = (usize, usize);

#[test]
pub fn test() {
    let input1 = vec![
        "########",
        "#..O.O.#",
        "##@.O..#",
        "#...O..#",
        "#.#.O..#",
        "#...O..#",
        "#......#",
        "########",
        "",
        "<^^>>>vv<v>>v<<",
    ];
    let input2 = vec![
        "##########",
        "#..O..O.O#",
        "#......O.#",
        "#.OO..O.O#",
        "#..O@..O.#",
        "#O#..O...#",
        "#O..O..O.#",
        "#.OO.O.OO#",
        "#....O...#",
        "##########",
        "",
        "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
        "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
        "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
        "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
        "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
        "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
        ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
        "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
        "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
        "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
    ];
    let input3 = vec![
        "#######",
        "#...#.#",
        "#.....#",
        "#..OO@#",
        "#..O..#",
        "#.....#",
        "#######",
        "",
        "<vv<<^^<<^^",
    ];

    assert_eq!(a(&input1), "2028");
    assert_eq!(a(&input2), "10092");
    assert_eq!(b(&input3), "618");
    assert_eq!(b(&input2), "9021");
}
