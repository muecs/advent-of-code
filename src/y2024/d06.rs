//! Day 6: Guard Gallivant

use std::collections::HashSet;

/// distinct map positions visited by guard
pub fn a(input: &Vec<&str>) -> String {
    let (mut map, start) = parse_input(input);
    simulate(&mut map, &start, false, &mut HashSet::new()).to_string()
}

/// possible obstruction locations to cause loop
pub fn b(input: &Vec<&str>) -> String {
    let (mut map, start) = parse_input(input);
    let mut visited = HashSet::new();

    // first run to find original route
    simulate(&mut map, &start, true, &mut visited);
    let visited = visited.iter().map(|(pos, _)| pos).collect::<HashSet<_>>();

    // then try obstructing each visited tile and check for loop
    let mut count = 0;
    for pos in visited {
        if *pos != start {
            map[pos.y as usize][pos.x as usize] = Tile::Wall;
            if simulate(&mut map, &start, true, &mut HashSet::new()) == 0 {
                count += 1;
            }
            map[pos.y as usize][pos.x as usize] = Tile::Visited; // revert
        }
    }

    count.to_string()
}

fn parse_input(input: &Vec<&str>) -> (Map, Pos) {
    let mut start = Pos::default();
    (
        input
            .iter()
            .enumerate()
            .map(|(y, s)| {
                s.char_indices()
                    .map(|(x, c)| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        '^' => {
                            start.x = x as i16;
                            start.y = y as i16;
                            Tile::Visited
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        start,
    )
}

fn simulate(
    map: &mut Map,
    start: &Pos,
    track_visited: bool,
    visited: &mut HashSet<(Pos, u8)>,
) -> usize {
    let w = map[0].len() as i16;
    let h = map.len() as i16;
    let mut pos = start.to_owned();
    let mut dir = 0; // up
    let mut visited_count = 1;
    loop {
        if track_visited && !visited.insert((pos.to_owned(), dir)) {
            // detected cycle
            return 0;
        }

        let new_pos = Pos {
            x: pos.x + [0, 1, 0, -1][dir as usize],
            y: pos.y + [-1, 0, 1, 0][dir as usize],
        };

        if !(0..w).contains(&new_pos.x) || !(0..h).contains(&new_pos.y) {
            // reached boundary
            return visited_count;
        }

        let new_tile = &mut map[new_pos.y as usize][new_pos.x as usize];
        match *new_tile {
            Tile::Empty => {
                // visiting a new tile
                *new_tile = Tile::Visited;
                visited_count += 1;
                pos = new_pos;
            }
            Tile::Wall => dir = (dir + 1) % 4, // turn right
            Tile::Visited => pos = new_pos,
        }
    }
}

#[derive(PartialEq)]
enum Tile {
    Empty,
    Wall,
    Visited,
}

type Map = Vec<Vec<Tile>>;

#[derive(Clone, Default, Eq, Hash, PartialEq)]
struct Pos {
    x: i16,
    y: i16,
}

#[test]
pub fn test() {
    let input = vec![
        "....#.....",
        ".........#",
        "..........",
        "..#.......",
        ".......#..",
        "..........",
        ".#..^.....",
        "........#.",
        "#.........",
        "......#...",
    ];

    assert_eq!(a(&input), "41");
    assert_eq!(b(&input), "6");
}
