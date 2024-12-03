//! Day 3: Mull It Over

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref MUL_REGEX: Regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
}

/// sum of valid multiplication instructions
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| {
            MUL_REGEX
                .find_iter(s)
                .map(|m| multiply(m.as_str()))
                .sum::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

/// sum of valid multiplication instructions with conditionals
pub fn b(input: &Vec<&str>) -> String {
    let mut enabled = true;
    input
        .iter()
        .map(|s| {
            let mut curr = 0;
            let mut sum = 0;
            while curr < s.len() {
                if enabled {
                    let next = s[curr..].find("don't()").map_or(s.len(), |i| i + curr);
                    // println!("enabled: {}", &s[curr..next]);
                    sum += MUL_REGEX
                        .find_iter(&s[curr..next])
                        .map(|m| multiply(m.as_str()))
                        .sum::<usize>();
                    curr = next + 7;
                } else {
                    let next = s[curr..].find("do()").map_or(s.len(), |i| i + curr);
                    // println!("disabled: {}", &s[curr..next]);
                    curr = next + 4;
                }
                if curr < s.len() {
                    enabled = !enabled;
                }
            }
            sum
        })
        .sum::<usize>()
        .to_string()
}

/// executes a `mul(a,b)` instruction
fn multiply(instr: &str) -> usize {
    let (a, b) = instr[4..instr.len() - 1].split_once(',').unwrap();
    a.parse::<usize>().unwrap() * b.parse::<usize>().unwrap()
}

#[test]
pub fn test() {
    let input = vec![
        // "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
    ];

    assert_eq!(a(&input), "161");
    assert_eq!(b(&input), "48");
}
