//! Day 9: Movie Theater

use std::collections::BinaryHeap;

/// area of largest rectangle
pub fn a(input: &Vec<&str>) -> String {
    let points = parse_input(input);
    let mut max_area = 0;
    traverse_rectangles(&points, |area, _, _| {
        if area > max_area {
            max_area = area;
        }
    });
    max_area.to_string()
}

/// area of largest rectangle enclosed in polygon
pub fn b(input: &Vec<&str>) -> String {
    let points = parse_input(input);
    let n = points.len();
    let mut rectangles = BinaryHeap::with_capacity(n * (n - 1) / 2);
    traverse_rectangles(&points, |area, i, j| {
        rectangles.push((area, i, j));
    });

    // check rectangles from largest to smallest
    while let Some((area, i, j)) = rectangles.pop() {
        let x1 = std::cmp::min(points[i].0, points[j].0);
        let y1 = std::cmp::min(points[i].1, points[j].1);
        let x2 = std::cmp::max(points[i].0, points[j].0);
        let y2 = std::cmp::max(points[i].1, points[j].1);

        // check that no polygon line intersects the rectangle
        let mut a = points.last().unwrap();
        let mut inside = true;
        for b in points.iter() {
            if line_intersects_rect(a.0, a.1, b.0, b.1, x1, y1, x2, y2) {
                inside = false;
                break;
            }
            a = b;
        }
        if inside {
            // rectangle fully inside polygon
            return area.to_string();
        }
    }
    String::new()
}

fn parse_input(input: &Vec<&str>) -> Points {
    input
        .iter()
        .map(|line| {
            let coords = line.split_once(',').unwrap();
            (
                coords.0.parse::<isize>().unwrap(),
                coords.1.parse::<isize>().unwrap(),
            )
        })
        .collect()
}

fn traverse_rectangles(points: &Points, mut f: impl FnMut(isize, usize, usize)) {
    for i in 0..points.len() {
        let (x1, y1) = points[i];
        for j in (i + 1)..points.len() {
            let (x2, y2) = points[j];
            if x1 == x2 || y1 == y2 {
                // skip skinny rectangles
                continue;
            }
            let area = ((x2 - x1).abs() + 1) * ((y2 - y1).abs() + 1);
            // println!("{x1},{y1} and {x2},{y2} = {area}");
            f(area, i, j);
        }
    }
}

fn line_intersects_rect(
    px1: isize,
    py1: isize,
    px2: isize,
    py2: isize,
    rx1: isize,
    ry1: isize,
    rx2: isize,
    ry2: isize,
) -> bool {
    // Line points on opposite sides of rectangle or one inside rectangle
    let (px_min, px_max) = if px1 <= px2 { (px1, px2) } else { (px2, px1) };
    let (py_min, py_max) = if py1 <= py2 { (py1, py2) } else { (py2, py1) };

    if px1 == px2 {
        // Vertical: must be strictly inside horizontally and overlap vertically
        px1 > rx1 && px1 < rx2 && py_min < ry2 && py_max > ry1
    } else {
        // Horizontal: must be strictly inside vertically and overlap horizontally
        py1 > ry1 && py1 < ry2 && px_min < rx2 && px_max > rx1
    }
}

type Point = (isize, isize);
type Points = Vec<Point>;

#[test]
pub fn test() {
    let input = vec!["7,1", "11,1", "11,7", "9,7", "9,5", "2,5", "2,3", "7,3"];

    assert_eq!(a(&input), "50");
    assert_eq!(b(&input), "24");
}
