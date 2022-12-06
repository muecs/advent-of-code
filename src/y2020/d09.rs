//! Day 9: Encoding Error

use std::collections::BTreeSet;

/// first number which is not the sum of two of the 25 numbers before it
pub fn a(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    find_outlier(&numbers, 25).unwrap().to_string()
}

/// sum of min and max of range that sums up to above number
pub fn b(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    let outlier = find_outlier(&numbers, 25).unwrap();
    find_range_sum(&numbers, outlier).unwrap().to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

fn find_outlier(numbers: &Vec<usize>, len: usize) -> Option<usize> {
    for window in numbers.windows(len + 1) {
        let mut sums = BTreeSet::new();
        for i in 0..len - 1 {
            for j in i + 1..len {
                if window[i] != window[j] {
                    sums.insert(window[i] + window[j]);
                }
            }
        }
        if !sums.contains(&window[len]) {
            return Some(window[len]);
        }
    }
    None
}

fn find_range_sum(numbers: &Vec<usize>, target_sum: usize) -> Option<usize> {
    let len = numbers.len();
    for i in 0..len - 1 {
        let mut sum = numbers[i];
        let mut min = numbers[i];
        let mut max = numbers[i];
        for j in i + 1..len {
            let n = numbers[j];
            sum += n;
            if n < min {
                min = n;
            }
            if n > max {
                max = n;
            }
            if sum == target_sum {
                return Some(min + max);
            }
        }
    }
    None
}

#[test]
pub fn test() {
    let input = vec![
        "35", "20", "15", "25", "47", "40", "62", "55", "65", "95", "102", "117", "150", "182",
        "127", "219", "299", "277", "309", "576",
    ];

    let numbers = parse_input(&input);
    assert_eq!(numbers.len(), 20);

    let outlier = find_outlier(&numbers, 5);
    assert_eq!(outlier, Some(127));
    assert_eq!(find_range_sum(&numbers, outlier.unwrap()), Some(62));

    // assert_eq!(a(&input), "");
    // assert_eq!(b(&input), "");
}
