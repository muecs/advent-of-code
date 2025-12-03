//! Day 3: Lobby

/// Sum of maximum two digit numbers
pub fn a(input: &Vec<&str>) -> String {
    input.iter().map(|s| max_joltage(s.as_bytes(), 2)).sum::<usize>().to_string()
}

/// Sum of maximum twelve digit numbers
pub fn b(input: &Vec<&str>) -> String {
    input.iter().map(|s| max_joltage(s.as_bytes(), 12)).sum::<usize>().to_string()
}

fn max_joltage(digits: &[u8], size: usize) -> usize {
    let mut max_val = vec![0u8; size];
    let mut max_idx = 0usize;
    let mut new_idx = 0usize;
    for j in 0..size {
        for i in new_idx..digits.len() - (size - j - 1) {
            if digits[i] > max_val[j] {
                max_val[j] = digits[i];
                max_idx = i;
            }
            if digits[i] == b'9' {
                break;
            }
        }
        new_idx = max_idx + 1;
    }
    str::from_utf8(&max_val).unwrap().parse::<usize>().unwrap()
}

#[test]
pub fn test() {
    let input = vec![
        "987654321111111",
        "811111111111119",
        "234234234234278",
        "818181911112111",
    ];

    assert_eq!(a(&input), "357");
    assert_eq!(b(&input), "3121910778619");
}
