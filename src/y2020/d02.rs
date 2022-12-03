//! Day 2: Password Philosophy

use std::str::FromStr;

/// number of valid passwords by range
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| s.parse::<Password>().unwrap().validate_range() as usize)
        .sum::<usize>()
        .to_string()
}

/// number of valid passwords by index
pub fn b(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| s.parse::<Password>().unwrap().validate_index() as usize)
        .sum::<usize>()
        .to_string()
}

struct Password {
    first: usize,
    second: usize,
    c: char,
    password: String,
}

impl Password {
    fn validate_range(&self) -> bool {
        let n = self
            .password
            .chars()
            .fold(0usize, |acc, c| acc + (c == self.c) as usize);
        n >= self.first && n <= self.second
    }

    fn validate_index(&self) -> bool {
        let chars = self.password.chars().collect::<Vec<_>>();
        (chars[self.first - 1] == self.c) ^ (chars[self.second - 1] == self.c)
    }
}

impl FromStr for Password {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (range, s) = s.split_once(' ').unwrap();
        let (first, second) = range
            .split_once('-')
            .map(|s| (s.0.parse::<usize>().unwrap(), s.1.parse::<usize>().unwrap()))
            .unwrap();
        Ok(Password {
            first,
            second,
            c: s.chars().next().unwrap(),
            password: s[3..].to_string(),
        })
    }
}

#[test]
pub fn test() {
    let input = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];

    assert_eq!(a(&input), "2");
    assert_eq!(b(&input), "1");
}
