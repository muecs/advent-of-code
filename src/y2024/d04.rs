//! Day 4: Ceres Search

/// occurences of `XMAS` in 8 directions
pub fn a(input: &Vec<&str>) -> String {
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
    const WORD: &str = "XMAS";
    const LEN: isize = WORD.len() as isize;
    let w = input[0].len() as isize;
    let h = input.len() as isize;
    let mut n = 0;

    for y in 0..h {
        for x in 0..w {
            for (dx, dy) in DIRS {
                if !(0..w).contains(&(x + (LEN - 1) * dx))
                    || !(0..h).contains(&(y + (LEN - 1) * dy))
                {
                    continue;
                }
                for i in 0..LEN {
                    let cx = (x + i * dx) as usize;
                    let cy = (y + i * dy) as usize;
                    if input[cy][cx..=cx] != WORD[i as usize..=i as usize] {
                        break;
                    }
                    if i + 1 == LEN {
                        n += 1;
                    }
                }
            }
        }
    }

    n.to_string()
}

/// occurences of `MAS` in X shape
pub fn b(input: &Vec<&str>) -> String {
    let w = input[0].len();
    let h = input.len();
    let mut n = 0;

    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if &input[y][x..=x] != "A" {
                continue;
            }
            let tl = &input[y - 1][x - 1..=x - 1];
            let tr = &input[y - 1][x + 1..=x + 1];
            let bl = &input[y + 1][x - 1..=x - 1];
            let br = &input[y + 1][x + 1..=x + 1];
            if ((tl == "M" && br == "S") || (tl == "S" && br == "M"))
                && ((bl == "M" && tr == "S") || (bl == "S" && tr == "M"))
            {
                n += 1;
            }
        }
    }

    n.to_string()
}

#[test]
pub fn test() {
    let input1 = vec!["..X...", ".SAMX.", ".A..A.", "XMAS.S", ".X...."];
    let input2 = vec![
        "MMMSXXMASM",
        "MSAMXMSMSA",
        "AMXSXMAAMM",
        "MSAMASMSMX",
        "XMASAMXAMM",
        "XXAMMXXAMA",
        "SMSMSASXSS",
        "SAXAMASAAA",
        "MAMMMXMMMM",
        "MXMXAXMASX",
    ];

    assert_eq!(a(&input1), "4");
    assert_eq!(a(&input2), "18");
    assert_eq!(b(&input2), "9");
}
