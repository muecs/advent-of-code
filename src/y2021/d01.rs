//! Day 1: Sonar Sweep

/// count the number of times a depth measurement increases
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|d| d[0] < d[1])
        .count()
        .to_string()
}

/// count the number of times the sum of a three-measurement sliding window increases
pub fn b(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|triple| triple.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|d| d[0] < d[1])
        .count()
        .to_string()
}

#[test]
pub fn test() {
    let input = "199 200 208 210 200 207 240 269 260 263".split(' ').collect();
    assert_eq!(a(&input), "7");
    assert_eq!(b(&input), "5");
}
