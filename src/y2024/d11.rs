//! Day 11: Plutonian Pebbles

use std::collections::HashMap;

/// number of stones after applying rules 25 times
pub fn a(input: &Vec<&str>) -> String {
    let stones = parse_input(input);
    stones
        .iter()
        .map(|stone| simulate(*stone, 25, &mut Cache::new()))
        .sum::<usize>()
        .to_string()
}

/// number of stones after applying rules 75 times
pub fn b(input: &Vec<&str>) -> String {
    let stones = parse_input(input);
    stones
        .iter()
        .map(|stone| simulate(*stone, 75, &mut Cache::new()))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    input
        .first()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

/// determines how many stones this one turns into after given amount of blinks
fn simulate(stone: usize, blinks: usize, cache: &mut Cache) -> usize {
    if blinks == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, blinks)) {
        return *result;
    }

    let result = if stone == 0 {
        simulate(1, blinks - 1, cache)
    } else if (stone.ilog10() + 1) % 2 == 0 {
        let s = format!("{}", stone);
        let halves = s.split_at(s.len() / 2);
        simulate(halves.0.parse().unwrap(), blinks - 1, cache)
            + simulate(halves.1.parse().unwrap(), blinks - 1, cache)
    } else {
        simulate(stone * 2024, blinks - 1, cache)
    };

    cache.insert((stone, blinks), result);
    result
}

type Cache = HashMap<(usize, usize), usize>;

#[test]
pub fn test() {
    let input = vec!["125 17"];

    assert_eq!(a(&input), "55312");
    // assert_eq!(b(&input), "");
}
