//! Day 6: Wait For It

/// number of ways to beat the record (multiple races)
pub fn a(input: &Vec<&str>) -> String {
    let times_it = input[0][9..]
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap());
    let distances_it = input[1][9..]
        .split_ascii_whitespace()
        .map(|n| n.parse::<usize>().unwrap());
    times_it
        .zip(distances_it)
        .map(|(time, distance)| count_winning_timings(time, distance))
        .product::<usize>()
        .to_string()
}

/// number of ways to beat the record (single race)
pub fn b(input: &Vec<&str>) -> String {
    let time = input[0][9..]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = input[1][9..]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    count_winning_timings(time, distance).to_string()
}

fn count_winning_timings(time: usize, distance: usize) -> usize {
    // d_min = (t_max - t) * t
    let t_max = time as f64;
    let d_min = distance as f64;
    let r = (t_max * t_max - 4.0 * d_min).sqrt();
    let low = ((t_max - r) / 2.0).floor() as usize + 1;
    let high = ((t_max + r) / 2.0).ceil() as usize - 1;
    high + 1 - low
}

#[test]
pub fn test() {
    let input = vec!["Time:      7  15   30", "Distance:  9  40  200"];

    assert_eq!(a(&input), "288");
    assert_eq!(b(&input), "71503");
}
