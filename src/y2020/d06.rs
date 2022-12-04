//! Day 6: Custom Customs

use std::collections::BTreeSet;

/// sum of number of distinct characters per group
pub fn a(input: &Vec<&str>) -> String {
    let groups = parse_input(input);
    groups
        .iter()
        .map(|g| {
            g.iter()
                .flatten()
                .copied()
                .collect::<BTreeSet<char>>()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

/// sum of number of common characters per group
pub fn b(input: &Vec<&str>) -> String {
    let groups = parse_input(input);
    groups
        .iter()
        .map(|g| {
            g.iter()
                .map(|chars| BTreeSet::from_iter(chars.iter().copied()))
                .reduce(|acc, set| BTreeSet::from_iter(acc.intersection(&set).copied()))
                .unwrap()
                .len()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<Vec<char>>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for line in input {
        if line.is_empty() {
            if !current_group.is_empty() {
                groups.push(current_group);
                current_group = Vec::new();
            }
            continue;
        }
        current_group.push(line.chars().collect::<Vec<_>>());
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }

    groups
}

#[test]
pub fn test() {
    let input = vec![
        "abc", "", "a", "b", "c", "", "ab", "ac", "", "a", "a", "a", "a", "", "b",
    ];

    assert_eq!(a(&input), "11");
    assert_eq!(b(&input), "6");
}
