//! Day 18: Lavaduct Lagoon

/// lagoon volume
pub fn a(input: &Vec<&str>) -> String {
    let mut area = 0i32;
    let mut length = 0i32;
    let mut y = 0i32;

    input.iter().for_each(|s| {
        let mut it = s.split(' ');
        let dir = it.next().unwrap().bytes().next().unwrap();
        let dist = it.next().unwrap().parse::<i32>().unwrap();

        // signed area above horizontal trench
        match dir {
            b'U' => y -= dist,
            b'D' => y += dist,
            b'L' => area -= dist * y,
            b'R' => area += dist * y,
            _ => unreachable!(),
        }
        length += dist;
    });

    // account for trench thickness
    (area.abs() + length / 2 + 1).to_string()
}

/// bigger lagoon volume
pub fn b(input: &Vec<&str>) -> String {
    let mut area = 0i64;
    let mut length = 0i64;
    let mut y = 0i64;

    input.iter().for_each(|s| {
        let code = s.rsplit_once(' ').unwrap().1;
        let dir = code[7..].bytes().next().unwrap();
        let dist = i64::from_str_radix(&code[2..7], 16).unwrap();

        // signed area above horizontal trench
        match dir {
            b'3' /* U */ => y -= dist,
            b'1' /* D */ => y += dist,
            b'2' /* L */ => area -= dist * y,
            b'0' /* R */ => area += dist * y,
            _ => unreachable!(),
        }
        length += dist;
    });

    // account for trench thickness
    (area.abs() + length / 2 + 1).to_string()
}

// fn parse_input(input: &Vec<&str>) {}

#[test]
pub fn test() {
    let input = vec![
        "R 6 (#70c710)",
        "D 5 (#0dc571)",
        "L 2 (#5713f0)",
        "D 2 (#d2c081)",
        "R 2 (#59c680)",
        "D 2 (#411b91)",
        "L 5 (#8ceee2)",
        "U 2 (#caa173)",
        "L 1 (#1b58a2)",
        "U 2 (#caa171)",
        "R 2 (#7807d2)",
        "U 3 (#a77fa3)",
        "L 2 (#015232)",
        "U 2 (#7a21e3)",
    ];

    assert_eq!(a(&input), "62");
    assert_eq!(b(&input), "952408144115");
}
