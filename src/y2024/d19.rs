//! Day 19: Linen Layout

use std::collections::HashMap;

/// possible designs with available patterns
pub fn a(input: &Vec<&str>) -> String {
    let patterns = input[0].split(", ").collect::<Vec<_>>();
    input[2..].iter().filter(|design| check_design(&patterns, design)).count().to_string()
}

/// possible ways to make each design
pub fn b(input: &Vec<&str>) -> String {
    let patterns = input[0].split(", ").collect::<Vec<_>>();
    let mut cache = HashMap::new();
    input[2..].iter().map(|design| count_designs(&patterns, &mut cache, design)).sum::<usize>().to_string()
}

fn check_design(patterns: &Vec<&str>, design: &str) -> bool {
    if design.is_empty() {
        return true;
    }
    for pattern in patterns {
        if pattern.len() <= design.len() && design.starts_with(pattern) && check_design(patterns, &design[pattern.len()..]) {
            return true;
        }
    }
    false
}

fn count_designs<'a>(patterns: &Vec<&str>, cache: &mut HashMap<&'a str, usize>, design: &'a str) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(hit) = cache.get(design) {
        return *hit;
    }
    let mut count = 0;
    for pattern in patterns {
        if pattern.len() <= design.len() && design.starts_with(pattern) {
            count += count_designs(patterns, cache, &design[pattern.len()..]);
        }
    }
    cache.insert(design, count);
    count
}

#[test]
pub fn test() {
    let input = vec![
        "r, wr, b, g, bwu, rb, gb, br",
        "",
        "brwrr",
        "bggr",
        "gbbr",
        "rrbgbr",
        "ubwu",
        "bwurrg",
        "brgr",
        "bbrgwb",
    ];

    assert_eq!(a(&input), "6");
    assert_eq!(b(&input), "16");
}
