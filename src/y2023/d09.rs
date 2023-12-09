//! Day 9: Mirage Maintenance

/// sum of forward extrapolated values
pub fn a(input: &Vec<&str>) -> String {
    let sequences = parse_input(input);
    sequences
        .iter()
        .map(|s| extrapolate(s, false))
        .sum::<i64>()
        .to_string()
}

/// sum of backward extrapolated values
pub fn b(input: &Vec<&str>) -> String {
    let sequences = parse_input(input);
    sequences
        .iter()
        .map(|s| extrapolate(s, true))
        .sum::<i64>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Sequence> {
    input
        .iter()
        .map(|s| s.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn extrapolate(values: &Sequence, backward: bool) -> i64 {
    //  10  13  16  21  30  45  [68]
    //     3   3   5   9  15  [23]
    //       0   2   4   6  [8]
    //         2   2   2  [2]
    //           0   0  [0]
    let mut diffs = values.clone();
    if backward {
        diffs.reverse();
    }
    for i in 1..diffs.len() {
        for j in 0..diffs.len() - i {
            diffs[j] = diffs[j + 1] - diffs[j];
        }
        // println!("{} {:?}", i, diffs);
    }
    diffs.iter().sum::<i64>()
}

type Sequence = Vec<i64>;

#[test]
pub fn test() {
    let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];

    let sequences = parse_input(&input);
    assert_eq!(sequences.len(), 3);
    assert_eq!(extrapolate(&sequences[0], false), 18);
    assert_eq!(extrapolate(&sequences[1], false), 28);
    assert_eq!(extrapolate(&sequences[2], false), 68);
    assert_eq!(extrapolate(&sequences[0], true), -3);
    assert_eq!(extrapolate(&sequences[1], true), 0);
    assert_eq!(extrapolate(&sequences[2], true), 5);

    assert_eq!(a(&input), "114");
    assert_eq!(b(&input), "2");
}
