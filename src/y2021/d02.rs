//! Day 2: Dive!

/// calculate the horizontal position and depth, then multiply them
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| parse_instruction(s))
        .reduce(|accum, item| (accum.0 + item.0, accum.1 + item.1))
        .map_or(0, |v| v.0 * v.1)
        .to_string()
}

/// up/down modifies aim, forward multiplied by aim is depth increment
pub fn b(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| parse_instruction(s))
        .map(|t| (t.0, t.1, 0))  // pos, aim, depth
        .reduce(|accum, item| (accum.0 + item.0, accum.1 + item.1, accum.2 + item.0 * accum.1))
        .map_or(0, |v| v.0 * v.2)
        .to_string()
}

/// convert textual instruction to (x, y) offset
fn parse_instruction(s: &str) -> (i32, i32) {
    let parts = s.split(' ').collect::<Vec<&str>>();
    let dist = parts.last().map_or(0, |n| n.parse::<i32>().unwrap_or(0));
    match parts.first() {
        Some(&"forward") => (dist, 0),
        Some(&"down") => (0, dist),
        Some(&"up") => (0, -dist),
        _ => (0, 0),
    }
}

#[test]
pub fn test() {
    let input = vec![
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    assert_eq!(parse_instruction(input[0]), (5, 0));
    assert_eq!(parse_instruction(input[1]), (0, 5));
    assert_eq!(parse_instruction(input[3]), (0, -3));

    assert_eq!(a(&input), "150");
    assert_eq!(b(&input), "900");
}
