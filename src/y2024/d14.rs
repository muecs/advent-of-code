//! Day 14: Restroom Redoubt

/// safety factor after 100 robot movements
pub fn a(input: &Vec<&str>) -> String {
    let robots = parse_input(input);
    simulate(&robots, 100).to_string()
}

/// number of steps until xmas tree arrangement
pub fn b(input: &Vec<&str>) -> String {
    let robots = parse_input(input);
    // calculate 10k safety scores and find outlier
    let scores = (0..10000).map(|steps| simulate(&robots, steps)).collect::<Vec<_>>();
    let mean = (scores.iter().map(|val| *val as usize).sum::<usize>() / scores.len()) as isize;
    let (step, _) = scores.iter().enumerate().fold((0, 0), |acc, (i, score)| {
        let dist = (mean - score).abs();
        if dist > acc.1 { (i, dist) } else { acc }
    });
    step.to_string()
}

fn parse_input(input: &Vec<&str>) -> Robots {
    input.iter().map(|s| s.split_once(' ').map(|(p, v)| (parse_coord(p), parse_coord(v))).unwrap()).collect()
}

fn parse_coord(coord: &str) -> Pos {
    coord.split_once(',').map(|(x, y)| (x[2..].parse().unwrap(), y.parse().unwrap())).unwrap()
}

fn simulate(robots: &Robots, steps: isize) -> isize {
    let (w, h) = if robots.len() == 12 { (11, 7) } else { (101, 103)};
    let mut quadrants = [0; 4];
    for robot in robots {
        let x = (robot.0.0 + robot.1.0 * steps).rem_euclid(w);
        let y = (robot.0.1 + robot.1.1 * steps).rem_euclid(h);
        let col = if x < w / 2 { 0 } else if x > w / 2 { 1 } else { continue };
        let row = if y < h / 2 { 0 } else if y > h / 2 { 1 } else { continue };
        quadrants[(col + row * 2) as usize] += 1;
    }
    quadrants.iter().product()
}

type Pos = (isize, isize);
type Robots = Vec<(Pos, Pos)>;

#[test]
pub fn test() {
    let input = vec![
        "p=0,4 v=3,-3",
        "p=6,3 v=-1,-3",
        "p=10,3 v=-1,2",
        "p=2,0 v=2,-1",
        "p=0,0 v=1,3",
        "p=3,0 v=-2,-2",
        "p=7,6 v=-1,-3",
        "p=3,0 v=-1,-2",
        "p=9,3 v=2,3",
        "p=7,3 v=-1,2",
        "p=2,4 v=2,-3",
        "p=9,5 v=-3,-3",
    ];

    assert_eq!(a(&input), "12");
    // assert_eq!(b(&input), "");
}
