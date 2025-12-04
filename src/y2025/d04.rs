//! Day 4: Printing Department

/// number of removable tiles after one pass
pub fn a(input: &Vec<&str>) -> String {
    let map = input.iter().map(|&s| s.as_bytes()).collect::<Vec<&[u8]>>();
    removable_tiles(&map).len().to_string()
}

/// number of removable tiles after repeated passes
pub fn b(input: &Vec<&str>) -> String {
    let mut map = input
        .iter()
        .map(|&s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut total = 0;
    loop {
        let map_refs: Vec<&[u8]> = map.iter().map(|v| v.as_slice()).collect();
        let removable = removable_tiles(&map_refs);
        let n = removable.len();
        if n == 0 {
            return total.to_string();
        }
        total += n;
        for (row, col) in removable {
            map[row][col] = b'.';
        }
    }
}

/// lists non-empty tiles with fewer than 4 non-empty adjacent tiles
fn removable_tiles(map: &Vec<&[u8]>) -> Vec<(usize, usize)> {
    const DIRS: [(isize, isize); 8] = [
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
    ];
    let mut tiles = Vec::new();
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == b'.' {
                continue;
            }
            let mut adjacent = 0;
            for (dx, dy) in DIRS {
                let icol = col as isize + dx;
                let irow = row as isize + dy;
                if icol < 0
                    || icol >= map[0].len() as isize
                    || irow < 0
                    || irow >= map.len() as isize
                {
                    continue;
                }
                if map[irow as usize][icol as usize] != b'.' {
                    adjacent += 1;
                }
            }
            if adjacent < 4 {
                tiles.push((row, col));
            }
        }
    }
    tiles
}

#[test]
pub fn test() {
    let input = vec![
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    assert_eq!(a(&input), "13");
    assert_eq!(b(&input), "43");
}
