//! Day 3: Rucksack Reorganization

use std::collections::BTreeSet;

/// sum of priorities of items in both halves of each string
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| find_shared_item(s))
        .map(|c| get_item_priority(c))
        .sum::<usize>()
        .to_string()
}

/// sum of priorities of shared item per group of 3
pub fn b(input: &Vec<&str>) -> String {
    input
        .chunks_exact(3)
        .map(|group| {
            group
                .iter()
                .map(|s| BTreeSet::from_iter(s.chars()))
                .reduce(|acc, set| BTreeSet::from_iter(acc.intersection(&set).cloned()))
                .unwrap()
                .iter()
                .next()
                .map(|c| get_item_priority(*c))
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

fn find_shared_item(s: &str) -> char {
    let (a, b) = s.split_at(s.len() / 2);
    let set_a = BTreeSet::from_iter(a.chars());
    let set_b = BTreeSet::from_iter(b.chars());
    *set_a.intersection(&set_b).next().unwrap()
}

fn get_item_priority(c: char) -> usize {
    let b = c as u8;
    match b {
        b'a'..=b'z' => (b - b'a' + 1).into(),
        b'A'..=b'Z' => (b - b'A' + 27).into(),
        _ => unreachable!("invalid item"),
    }
}

#[test]
pub fn test() {
    let input = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    ];

    assert_eq!(find_shared_item(input[0]), 'p');
    assert_eq!(get_item_priority('p'), 16);
    assert_eq!(get_item_priority('L'), 38);

    assert_eq!(a(&input), "157");
    assert_eq!(b(&input), "70");
}
