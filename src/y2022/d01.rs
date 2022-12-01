//! Day 1: Calorie Counting

/// highest sum of values per group
pub fn a(input: &Vec<&str>) -> String {
    parse_input(input)
        .iter()
        .map(|n| n.iter().sum::<usize>())
        .max()
        .unwrap()
        .to_string()
}

/// sum of 3 largest group sums
pub fn b(input: &Vec<&str>) -> String {
    let mut group_totals = parse_input(input)
        .iter()
        .map(|n| n.iter().sum::<usize>())
        .collect::<Vec<_>>();
    group_totals.sort_unstable_by(|a, b| b.cmp(&a));
    group_totals
        .iter()
        .take(3)
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<usize>> {
    let mut groups = Vec::new();
    let mut current_group = Vec::new();
    for s in input {
        if !s.is_empty() {
            current_group.push(s.parse::<usize>().unwrap());
        } else if !current_group.is_empty() {
            groups.push(current_group);
            current_group = Vec::new();
        }
    }
    if !current_group.is_empty() {
        groups.push(current_group);
    }
    groups
}

#[test]
pub fn test() {
    let input = vec![
        "1000",
        "2000",
        "3000",
        "",
        "4000",
        "",
        "5000",
        "6000",
        "",
        "7000",
        "8000",
        "9000",
        "",
        "10000",
    ];

    assert_eq!(a(&input), "24000");
    assert_eq!(b(&input), "45000");
}
