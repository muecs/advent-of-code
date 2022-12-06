//! Day 10: Adapter Array

/// product of distinct step counts between sorted numbers
pub fn a(input: &Vec<&str>) -> String {
    let ratings = parse_input(input);
    let mut counts = [0usize; 4];
    counts[3] = 1; // device's rating diff
    ratings
        .windows(2)
        .for_each(|pair| counts[pair[0].abs_diff(pair[1])] += 1);

    (counts[1] * counts[3]).to_string()
}

/// number of distinct paths through ascending numbers max 3 apart
pub fn b(input: &Vec<&str>) -> String {
    let ratings = parse_input(input);
    let len = ratings.len();
    let mut options = vec![0usize; len];
    options[len - 1] = 1;
    for i in (0..len - 1).rev() {
        for j in i + 1..len {
            if ratings[i] + 3 >= ratings[j] {
                options[i] += options[j];
            }
        }
    }
    options[0].to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    let mut ratings: Vec<usize> = input.iter().map(|s| s.parse().unwrap()).collect();
    ratings.push(0); // outlet
    ratings.sort_unstable();
    ratings
}

#[test]
pub fn test() {
    let input1 = vec!["16", "10", "15", "5", "1", "11", "7", "19", "6", "12", "4"];
    let input2 = vec![
        "28", "33", "18", "42", "31", "14", "46", "20", "48", "47", "24", "23", "49", "45", "19",
        "38", "39", "11", "1", "32", "25", "35", "8", "17", "7", "9", "4", "2", "34", "10", "3",
    ];

    assert_eq!(a(&input1), "35");
    assert_eq!(a(&input2), "220");

    assert_eq!(b(&input1), "8");
    assert_eq!(b(&input2), "19208");
}
