//! Day 25: Code Chronicle

/// matching lock/key pairs
pub fn a(input: &Vec<&str>) -> String {
    let (locks, keys) = parse_input(input);
    let mut matches = 0;
    for lock in &locks {
        for key in &keys {
            if (0..5).all(|i| lock[i] + key[i] <= 5) {
                matches += 1;
            }
        }
    }
    matches.to_string()
}

fn parse_input(input: &Vec<&str>) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in input.chunks(8) {
        let mut heights = [0; 5];
        for i in 1..=5 {
            for j in 0..5 {
                if schematic[i].as_bytes()[j] == b'#' {
                    heights[j] += 1;
                }
            }
        }
        if schematic[0] == "#####" {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }
    (locks, keys)
}

#[test]
pub fn test() {
    let input = vec![
        "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
        ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
        "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
        "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
    ];

    assert_eq!(a(&input), "3");
}
