//! Day 4: Camp Cleanup

/// number of fully contained ranges
pub fn a(input: &Vec<&str>) -> String {
    let pairs = parse_input(input);
    pairs
        .iter()
        .filter(|(range1, range2)| range_contains(range1, range2) || range_contains(range2, range1))
        .count()
        .to_string()
}

/// number of intersecting ranges
pub fn b(input: &Vec<&str>) -> String {
    let pairs = parse_input(input);
    pairs
        .iter()
        .filter(|(range1, range2)| ranges_intersect(range1, range2))
        .count()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<(Range, Range)> {
    fn parse_range(range: &str) -> Range {
        range
            .split_once('-')
            .map(|p| (p.0.parse().unwrap(), p.1.parse().unwrap()))
            .unwrap()
    }

    input
        .iter()
        .map(|s| s.split_once(',').unwrap())
        .map(|(range1, range2)| (parse_range(range1), parse_range(range2)))
        .collect()
}

fn range_contains(range: &Range, other: &Range) -> bool {
    range.0 <= other.0 && range.1 >= other.1
}

fn ranges_intersect(range1: &Range, range2: &Range) -> bool {
    std::cmp::max(range1.0, range2.0) <= std::cmp::min(range1.1, range2.1)
}

type Range = (usize, usize);

#[test]
pub fn test() {
    let input = vec![
        "2-4,6-8",
        "2-3,4-5",
        "5-7,7-9",
        "2-8,3-7",
        "6-6,4-6",
        "2-6,4-8",
    ];

    assert_eq!(a(&input), "2");
    assert_eq!(b(&input), "4");
}
