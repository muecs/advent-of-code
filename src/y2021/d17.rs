//! Day 17: Trick Shot

type Point = (i32, i32);

#[derive(Debug, PartialEq)]
struct Rect {
    p1: Point,
    p2: Point,
}

impl Rect {
    fn inside(&self, p: &Point) -> bool {
        self.p1.0 <= p.0 && p.0 <= self.p2.0
            && self.p1.1 <= p.1 && p.1 <= self.p2.1
    }
}

/// find highest trajectory
pub fn a(input: &Vec<&str>) -> String {
    let target = parse_input(input);

    let min_v_x = calc_velocity(target.p1.0);
    let max_v_x = calc_velocity(target.p2.0);
    let mut min_v_y = 0;
    let max_v_y = target.p2.0;
    let mut total_max_y = 0;

    for x in min_v_x..=max_v_x {
        for y in min_v_y..=max_v_y {
            let mut max_y = 0;
            if evaluate(&(x, y), &target, &mut max_y) && max_y > total_max_y {
                total_max_y = max_y;
                min_v_y = y;
                println!("{} {} {}", x, y, max_y);
            }
        }
    }

    total_max_y.to_string()
}

/// number of possible initial velocities
pub fn b(input: &Vec<&str>) -> String {
    let target = parse_input(input);

    let min_v_x = calc_velocity(target.p1.0);
    let max_v_x = target.p2.0;
    let min_v_y = target.p1.1;
    let max_v_y = target.p2.0;

    let mut count = 0;
    for x in min_v_x..=max_v_x {
        for y in min_v_y..=max_v_y {
            let mut max_y = 0;
            if evaluate(&(x, y), &target, &mut max_y) {
                count += 1;
            }
        }
    }

    count.to_string()
}

fn parse_input(input: &Vec<&str>) -> Rect {
    assert!(input.len() == 1);
    let (x_range, y_range) = input[0][13..].split_once(", ").unwrap();
    let (x1, x2) = x_range[2..].split_once("..").unwrap();
    let (y1, y2) = y_range[2..].split_once("..").unwrap();
    Rect {
        p1: (x1.parse().unwrap(), y1.parse().unwrap()),
        p2: (x2.parse().unwrap(), y2.parse().unwrap()),
    }
}

fn evaluate(v0: &Point, target: &Rect, max_y: &mut i32) -> bool {
    let mut p = (0, 0);
    let mut v = *v0;
    *max_y = p.1;
    while p.0 < target.p2.0 && p.1 > target.p1.1 {
        p.0 += v.0;
        p.1 += v.1;

        if p.1 > *max_y {
            *max_y = p.1;
        }

        if target.inside(&p) {
            return true;
        }

        if v.0 > 0 { v.0 -= 1 } else if v.0 < 0 { v.0 += 1 }  // drag
        v.1 -= 1;  // gravity

        if v.0 == 0 && p.0 < target.p1.0 {
            return false;
        }
    }
    false
}

// inverse of sum of ascending numbers
fn calc_velocity(x: i32) -> i32 {
    ((2f64 * x as f64 + 0.25).sqrt() - 0.5).round() as i32
}

#[test]
pub fn test() {
    let input = vec!["target area: x=20..30, y=-10..-5"];
    let rect = Rect { p1: (20, -10), p2: (30, -5) };

    assert_eq!(parse_input(&input), rect);
    assert!(rect.inside(&(20, -10)));
    assert!(rect.inside(&(25, -7)));
    assert!(rect.inside(&(30, -5)));
    assert!(!rect.inside(&(10, -7)));
    assert!(!rect.inside(&(25, -2)));

    let mut max_y = 0;
    assert!(evaluate(&(6, 3), &rect, &mut max_y));
    assert!(evaluate(&(9, 0), &rect, &mut max_y));
    assert!(evaluate(&(6, 9), &rect, &mut max_y));
    assert!(!evaluate(&(17, -4), &rect, &mut max_y));

    assert_eq!(a(&input), "45");
    assert_eq!(b(&input), "112");
}
