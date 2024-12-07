//! Day 7: Bridge Repair

/// sum of results of solvable equations with operators + and *
pub fn a(input: &Vec<&str>) -> String {
    let equations = parse_input(input);
    equations
        .iter()
        .filter_map(|(value, numbers)| find_operators(value, 0, &numbers, false).then_some(value))
        .sum::<usize>()
        .to_string()
}

/// sum of results of solvable equations with operators +, * and concat
pub fn b(input: &Vec<&str>) -> String {
    let equations = parse_input(input);
    equations
        .iter()
        .filter_map(|(value, numbers)| find_operators(value, 0, &numbers, true).then_some(value))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<(usize, Vec<usize>)> {
    input
        .iter()
        .map(|s| {
            s.split_once(": ")
                .map(|(value, numbers)| {
                    (
                        value.parse::<usize>().unwrap(),
                        numbers
                            .split_whitespace()
                            .map(|n| n.parse::<usize>().unwrap())
                            .collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn find_operators(
    value: &usize,
    intermediate: usize,
    numbers: &[usize],
    with_concat: bool,
) -> bool {
    if numbers.is_empty() {
        // done; now check the result
        return *value == intermediate;
    }

    if intermediate > *value {
        // we already overshot the target value
        return false;
    }

    // try operators recursively
    find_operators(value, intermediate + numbers[0], &numbers[1..], with_concat)
        || find_operators(value, intermediate * numbers[0], &numbers[1..], with_concat)
        || (with_concat
            && find_operators(
                value,
                concat(intermediate, numbers[0]),
                &numbers[1..],
                with_concat,
            ))
}

fn concat(a: usize, b: usize) -> usize {
    format!("{a}{b}").parse().unwrap()
}

#[test]
pub fn test() {
    let input = vec![
        "190: 10 19",
        "3267: 81 40 27",
        "83: 17 5",
        "156: 15 6",
        "7290: 6 8 6 15",
        "161011: 16 10 13",
        "192: 17 8 14",
        "21037: 9 7 18 13",
        "292: 11 6 16 20",
    ];

    assert_eq!(a(&input), "3749");
    assert_eq!(b(&input), "11387");
}
