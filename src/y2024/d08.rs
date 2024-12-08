//! Day 8: Resonant Collinearity

use std::collections::HashSet;

/// unique locations of each antenna reflected past each other matching antenna
pub fn a(input: &Vec<&str>) -> String {
    let w = input[0].len() as i16;
    let h = input.len() as i16;
    let antennas = parse_input(input);
    let antinodes = antennas
        .iter()
        .flat_map(|(p1, c1)| {
            antennas.iter().filter_map(move |(p2, c2)| {
                if c1 == c2 && p1 != p2 {
                    let a = Pos {
                        x: 2 * p2.x - p1.x,
                        y: 2 * p2.y - p1.y,
                    };
                    ((0..w).contains(&a.x) && (0..h).contains(&a.y)).then_some(a)
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    antinodes.len().to_string()
}

/// unique locations of repeatedly reflected antennas
pub fn b(input: &Vec<&str>) -> String {
    let w = input[0].len() as i16;
    let h = input.len() as i16;
    let antennas = parse_input(input);
    let mut antinodes = HashSet::new();

    for (p1, c1) in &antennas {
        for (p2, c2) in &antennas {
            if c1 != c2 || p1 == p2 {
                continue;
            }
            let dx = p2.x - p1.x;
            let dy = p2.y - p1.y;
            for i in 1.. {
                let a = Pos {
                    x: p1.x + i * dx,
                    y: p1.y + i * dy,
                };
                if (0..w).contains(&a.x) && (0..h).contains(&a.y) {
                    antinodes.insert(a);
                } else {
                    break;
                }
            }
        }
    }

    antinodes.len().to_string()
}

fn parse_input(input: &Vec<&str>) -> Antennas {
    input
        .iter()
        .enumerate()
        .flat_map(|(y, s)| {
            s.char_indices().filter_map(move |(x, c)| {
                (c != '.').then(|| {
                    (
                        Pos {
                            x: x as i16,
                            y: y as i16,
                        },
                        c,
                    )
                })
            })
        })
        .collect()
}

type Antennas = Vec<(Pos, char)>;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Pos {
    x: i16,
    y: i16,
}

#[test]
pub fn test() {
    let input = vec![
        "............",
        "........0...",
        ".....0......",
        ".......0....",
        "....0.......",
        "......A.....",
        "............",
        "............",
        "........A...",
        ".........A..",
        "............",
        "............",
    ];

    assert_eq!(a(&input), "14");
    assert_eq!(b(&input), "34");
}
