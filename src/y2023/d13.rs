//! Day 13: Point of Incidence

/// sum of reflections
pub fn a(input: &Vec<&str>) -> String {
    let patterns = parse_input(input);
    summarize(&patterns, false).to_string()
}

/// sum of reflections with one symbol flipped
pub fn b(input: &Vec<&str>) -> String {
    let patterns = parse_input(input);
    summarize(&patterns, true).to_string()
}

fn parse_input(input: &Vec<&str>) -> Patterns {
    input
        .split(|line| line.is_empty())
        .map(|pattern| {
            let mut columns = vec![0u32; pattern[0].len()];
            let rows = pattern
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.bytes().enumerate().fold(0u32, |acc, (x, b)| {
                        if b == b'#' {
                            columns[x] += (1 << y) as u32;
                            acc + (1 << x) as u32
                        } else {
                            acc
                        }
                    })
                })
                .collect();
            Pattern { rows, columns }
        })
        .collect()
}

fn find_reflection(checksums: &Checksums, flips: u32) -> Option<usize> {
    // println!("find reflection in {:?}", checksums);
    (1..checksums.len()).find_map(|i| {
        ((0..i.min(checksums.len() - i))
            .map(|j| (checksums[i + j] ^ checksums[i - 1 - j]).count_ones())
            .sum::<u32>()
            == flips)
            .then_some(i)
    })
}

fn summarize(patterns: &Patterns, with_smudge: bool) -> usize {
    let flips = if with_smudge { 1 } else { 0 };
    patterns.iter().fold(0, |acc, pattern| {
        if let Some(x) = find_reflection(&pattern.columns, flips) {
            acc + x
        } else if let Some(y) = find_reflection(&pattern.rows, flips) {
            acc + 100 * y
        } else {
            0
        }
    })
}

type Checksums = Vec<u32>;
struct Pattern {
    rows: Checksums,
    columns: Checksums,
}
type Patterns = Vec<Pattern>;

#[test]
pub fn test() {
    assert_eq!(find_reflection(&vec![1, 2, 3], 0), None);
    assert_eq!(find_reflection(&vec![1, 1, 2, 3], 0), Some(1));
    assert_eq!(find_reflection(&vec![1, 2, 2, 1], 0), Some(2));
    assert_eq!(find_reflection(&vec![1, 2, 3, 3], 0), Some(3));

    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];

    let patterns = parse_input(&input);
    assert_eq!(patterns.len(), 2);
    assert_eq!(patterns[0].rows.len(), 7);
    assert_eq!(patterns[0].columns.len(), 9);
    assert_eq!(patterns[0].rows[0], 0b011001101);
    assert_eq!(patterns[0].columns[0], 0b1001101);
    assert_eq!(patterns[1].rows.len(), 7);
    assert_eq!(patterns[1].columns.len(), 9);
    assert_eq!(patterns[1].rows[0], 0b100110001);
    assert_eq!(patterns[1].columns[0], 0b1011011);

    assert_eq!(a(&input), "405");
    assert_eq!(b(&input), "400");
}
