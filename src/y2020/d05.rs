//! Day 5: Binary Boarding

use std::collections::BTreeSet;

/// highest seat ID
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| decode_seat(s))
        .max()
        .unwrap()
        .to_string()
}

/// missing seat ID
pub fn b(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| decode_seat(s))
        .collect::<BTreeSet<_>>()
        .iter()
        .collect::<Vec<_>>()
        .windows(2)
        .fold(0usize, |acc, pair| {
            if *pair[0] + 2 == *pair[1] {
                *pair[0] + 1
            } else {
                acc
            }
        })
        .to_string()
}

fn decode_seat(s: &str) -> usize {
    let row_bin = s[0..7].replace("F", "0").replace("B", "1");
    let row = usize::from_str_radix(&row_bin, 2).unwrap();
    let col_bin = s[7..].replace("L", "0").replace("R", "1");
    let col = usize::from_str_radix(&col_bin, 2).unwrap();
    row * 8 + col
}

#[test]
pub fn test() {
    let input = vec![
        "FBFBBFFRLR",
        "BFFFBBFRRR",
        "FFFBBBFRRR",
        "BBFFBBFRLL",
        "BBFFBBFRRL",
    ];

    assert_eq!(decode_seat(input[0]), 357);
    assert_eq!(decode_seat(input[1]), 567);
    assert_eq!(decode_seat(input[2]), 119);
    assert_eq!(decode_seat(input[3]), 820);
    assert_eq!(decode_seat(input[4]), 822);

    assert_eq!(a(&input), "822");
    assert_eq!(b(&input), "821");
}
