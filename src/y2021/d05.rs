//! Day 5: Hydrothermal Venture

use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

type Point = (u16, u16);

#[derive(Clone, Debug, PartialEq)]
struct Line {
    pub p1: Point,
    pub p2: Point,
}
type Lines = Vec<Line>;

type Canvas = Vec<Vec<u16>>;

/// number of points where horizontal and vertical lines overlap
pub fn a(input: &Vec<&str>) -> String {
    let lines = input
        .iter()
        .map(|&s| Line::from_str(s).unwrap())
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .collect::<Lines>();

    let canvas = draw_lines(&lines);
    let overlaps = count_overlaps(&canvas);

    overlaps.to_string()
}

/// number of points where horizontal/vertical/diagonal lines overlap
pub fn b(input: &Vec<&str>) -> String {
    let lines = input
        .iter()
        .map(|&s| Line::from_str(s).unwrap())
        .collect::<Lines>();

    let canvas = draw_lines(&lines);
    let overlaps = count_overlaps(&canvas);

    overlaps.to_string()
}

impl FromStr for Line {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"^([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$"
            ).unwrap();
        }
        let cap = RE.captures(s).ok_or::<Self::Err>("Not a valid line")?;
        Ok(Line {
            p1: (
                cap.get(1).map_or("", |m| m.as_str()).parse().unwrap(),
                cap.get(2).map_or("", |m| m.as_str()).parse().unwrap(),
            ),
            p2: (
                cap.get(3).map_or("", |m| m.as_str()).parse().unwrap(),
                cap.get(4).map_or("", |m| m.as_str()).parse().unwrap(),
            ),
        })
    }
}

impl Line {
    fn is_horizontal(&self) -> bool { self.p1.1 == self.p2.1 }
    fn is_vertical(&self) -> bool { self.p1.0 == self.p2.0 }
}

fn get_line_max(lines: &Lines) -> (u16, u16) {
    lines.iter().fold((0u16, 0u16), |accum, item| (
        std::cmp::max(accum.0, std::cmp::max(item.p1.0, item.p2.0)),
        std::cmp::max(accum.1, std::cmp::max(item.p1.1, item.p2.1)),
    ))
}

fn draw_lines(lines: &Lines) -> Canvas {
    let (max_x, max_y) = get_line_max(lines);
    let mut canvas = vec![vec![0; (max_y + 1).into()]; (max_x + 1).into()];

    for line in lines {
        let mut x = line.p1.0;
        let mut y = line.p1.1;
        loop {
            canvas[usize::from(x)][usize::from(y)] += 1;
            if x == line.p2.0 && y == line.p2.1 {
                break;
            }
            if x < line.p2.0 {
                x += 1;
            } else if x > line.p2.0 {
                x -= 1;
            }
            if y < line.p2.1 {
                y += 1;
            } else if y > line.p2.1 {
                y -= 1;
            }
        }

        // if line.is_horizontal() {
        //     for x in line.p1.0..=line.p2.0 {
        //         canvas[usize::from(x)][usize::from(line.p1.1)] += 1;
        //     }
        // } else if line.is_vertical() {
        //     for y in line.p1.1..=line.p2.1 {
        //         canvas[usize::from(line.p1.0)][usize::from(y)] += 1;
        //     }
        // }
    }

    canvas
}

fn count_overlaps(canvas: &Canvas) -> usize {
    canvas
        .iter()
        .fold(0, |acc, row| acc + row.iter().filter(|&&x| x > 1).count())
}

#[test]
pub fn test() {
    let input = vec![
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    assert_eq!(Line::from_str(input[0]), Ok(Line{ p1: (0, 9), p2: (5, 9) }));
    assert!(Line::from_str(input[0]).unwrap().is_horizontal());
    assert!(!Line::from_str(input[0]).unwrap().is_vertical());
    assert!(!Line::from_str(input[1]).unwrap().is_horizontal());
    assert!(!Line::from_str(input[1]).unwrap().is_vertical());
    assert!(!Line::from_str(input[3]).unwrap().is_horizontal());
    assert!(Line::from_str(input[3]).unwrap().is_vertical());

    assert_eq!(a(&input), "5");
    assert_eq!(b(&input), "12");
}
