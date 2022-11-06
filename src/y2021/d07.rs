//! Day 7: The Treachery of Whales

/// part a
pub fn a(input: &Vec<&str>) -> String {
    let mut positions = parse_input(input);
    positions.sort_unstable();
    let pivot = positions[positions.len() / 2];
    let cost = total_cost(&positions, pivot);

    cost.to_string()
}

/// part b
pub fn b(input: &Vec<&str>) -> String {
    let positions = parse_input(input);
    let mut min = (usize::MAX, 0);
    // brute force - O(n²)
    for i in 0..positions.len() {
        let cost = total_inc_cost(&positions, i);
        if cost < min.0 {
            min = (cost, i);
        }
    }

    min.0.to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    input
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

/// sum of distances to target
fn total_cost(positions: &Vec<usize>, target: usize) -> usize {
    positions
        .iter()
        .map(|&x| (target as i64 - x as i64).abs() as usize)
        .sum()
}

// sum of ascending numbers
fn to_inc_cost(x: usize) -> usize {
    x * (x + 1) / 2
}

// inverse of sum of ascending numbers
#[allow(dead_code)]
fn from_inc_cost(x: usize) -> usize {
    ((2f64 * x as f64 + 0.25).sqrt() - 0.5).round() as usize
}

/// sum of incremental fuel costs
fn total_inc_cost(positions: &Vec<usize>, target: usize) -> usize {
    positions
        .iter()
        .map(|&x| to_inc_cost((target as i64 - x as i64).abs() as usize))
        .sum()
}

#[test]
pub fn test() {
    let input = vec!["16,1,2,0,4,2,7,1,2,14"];

    let positions = parse_input(&input);
    assert_eq!(positions, vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]);
    assert_eq!(total_cost(&positions, 2), 37);

    assert_eq!(to_inc_cost(10), 55);
    assert_eq!(from_inc_cost(55), 10);
    assert_eq!(total_inc_cost(&positions, 2), 206);
    assert_eq!(total_inc_cost(&positions, 5), 168);

    assert_eq!(a(&input), "37");
    assert_eq!(b(&input), "168");
}
