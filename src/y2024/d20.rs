//! Day 20: Race Condition

#[cfg(test)]
const LIMIT: Size = 50;
#[cfg(not(test))]
const LIMIT: Size = 100;

/// cheats of length 2 saving at least 100 steps
pub fn a(input: &Vec<&str>) -> String {
    let (map, start) = parse_input(input);
    find_cheats(&map, &start, 2).to_string()
}

/// cheats of length 20 saving at least 100 steps
pub fn b(input: &Vec<&str>) -> String {
    let (map, start) = parse_input(input);
    find_cheats(&map, &start, 20).to_string()
}

fn parse_input(input: &Vec<&str>) -> (Map, Pos) {
    let mut start = Pos::default();
    let map = input
        .iter()
        .enumerate()
        .map(|(y, s)| {
            s.char_indices()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' | 'E' => true,
                    'S' => {
                        start = (x as Size, y as Size);
                        true
                    }
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    (map, start)
}

fn find_cheats(map: &Map, start: &Pos, max_dist: Size) -> usize {
    let mut course = Vec::with_capacity(10000);
    let mut pending = Some(*start);
    let mut cheats = 0usize;

    while let Some(pos) = pending.take() {
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let next_pos = (
                pos.0.checked_add_signed(dx).unwrap(),
                pos.1.checked_add_signed(dy).unwrap(),
            );
            if map[next_pos.1 as usize][next_pos.0 as usize]
                && course.last().is_none_or(|last_pos| last_pos != &next_pos)
            {
                pending = Some(next_pos);
                break; // only one way
            }
        }

        // count allowable cheats from this tile to previous ones
        let len = course.len() as Size;
        for i in 0..len.saturating_sub(LIMIT + 1) {
            let dist = distance(&pos, &course[i as usize]);
            if dist <= max_dist && i + dist + LIMIT <= len {
                cheats += 1;
            }
        }

        course.push(pos);
    }

    cheats
}

/// calculates Manhattan Distance between two points
fn distance(p1: &Pos, p2: &Pos) -> Size {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

type Size = u16;
type Pos = (Size, Size);
type Map = Vec<Vec<bool>>;

#[test]
pub fn test() {
    let input = vec![
        "###############",
        "#...#...#.....#",
        "#.#.#.#.#.###.#",
        "#S#...#.#.#...#",
        "#######.#.#.###",
        "#######.#.#...#",
        "#######.#.###.#",
        "###..E#...#...#",
        "###.#######.###",
        "#...###...#...#",
        "#.#####.#.###.#",
        "#.#...#.#.#...#",
        "#.#.#.#.#.#.###",
        "#...#...#...###",
        "###############",
    ];

    assert_eq!(a(&input), "1");
    assert_eq!(b(&input), "285");
}
