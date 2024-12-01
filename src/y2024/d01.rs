//! Day 1: Historian Hysteria

use std::collections::HashMap;

/// sum of diffs of paired numbers from sorted lists
pub fn a(input: &Vec<&str>) -> String {
    let (mut left, mut right) = parse_input(input);
    left.sort_unstable();
    right.sort_unstable();
    left.iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum::<isize>()
        .to_string()
}

/// sum of products of number in left list and their occurrences in the right list
pub fn b(input: &Vec<&str>) -> String {
    let (left, right) = parse_input(input);

    let mut counts = HashMap::<isize, isize>::new();
    for n in right {
        counts.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }

    left.iter()
        .map(|n| n * counts.get(n).unwrap_or(&0))
        .sum::<isize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> (Vec<isize>, Vec<isize>) {
    input
        .iter()
        .map(|s| s.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<isize>().unwrap(), b.parse::<isize>().unwrap()))
        .unzip()
}

#[test]
pub fn test() {
    let input = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];

    assert_eq!(a(&input), "11");
    assert_eq!(b(&input), "31");
}
