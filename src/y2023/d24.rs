//! Day 24: Never Tell Me The Odds

const MIN: f64 = 200000000000000.0;
const MAX: f64 = 400000000000000.0;

/// number of future path intersections
pub fn a(input: &Vec<&str>) -> String {
    let lines = parse_input(input);
    count_intersections_2d(&lines, MIN, MAX).to_string()
}

/// sum of initial position coords of line intersecting all input lines
pub fn b(input: &Vec<&str>) -> String {
    let lines = parse_input(input);

    let solutions = [(0, 1, 3, 4), (2, 1, 5, 4)].map(|(x, y, dx, dy)| {
        let matrix = lines
            .iter()
            .map(|line| [line.0.x, line.0.y, line.0.z, line.1.x, line.1.y, line.1.z])
            .map(|v| vec![-v[dy], v[dx], v[y], -v[x], v[y] * v[dx] - v[x] * v[dy]])
            .collect::<Matrix>();
        let last = matrix.last().unwrap();
        let mut matrix = matrix
            .iter()
            .take(4)
            .map(|row| row.iter().zip(last.iter()).map(|(a, b)| a - b).collect())
            .collect::<Matrix>();
        gaussian_elimination(&mut matrix)
    });

    // println!("{solutions:?}");

    ((solutions[0][0] + solutions[0][1] + solutions[1][0]) as i64).to_string()
}

fn parse_input(input: &Vec<&str>) -> Lines {
    input
        .iter()
        .map(|s| {
            s.split_once(" @ ")
                .map(|(p, v)| (parse_point(p), parse_point(v)))
                .unwrap()
        })
        .collect()
}

fn parse_point(s: &str) -> Point {
    let mut it = s.split(", ");
    Point {
        x: it.next().unwrap().parse().unwrap(),
        y: it.next().unwrap().parse().unwrap(),
        z: it.next().unwrap().parse().unwrap(),
    }
}

fn intersect_within_2d(a: &Line, b: &Line, min: f64, max: f64) -> bool {
    let x1 = a.0.x;
    let y1 = a.0.y;
    let x2 = a.0.x + a.1.x;
    let y2 = a.0.y + a.1.y;
    let x3 = b.0.x;
    let y3 = b.0.y;
    let x4 = b.0.x + b.1.x;
    let y4 = b.0.y + b.1.y;

    let denom = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
    // let denom = -a.1.x * -b.1.y - -a.0.y * -b.1.x;
    if denom == 0.0 {
        return false; // parallel
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) / denom;
    let u = ((x1 - x3) * (y1 - y2) - (y1 - y3) * (x1 - x2)) / denom;

    if t < 0.0 || u < 0.0 {
        return false; // crossed in past
    }

    let int_x = a.0.x + t * a.1.x;
    let int_y = a.0.y + t * a.1.y;
    // println!("{t}, {u} - {int_x}, {int_y}");
    min <= int_x && int_x <= max && min <= int_y && int_y <= max
}

fn count_intersections_2d(lines: &Lines, min: f64, max: f64) -> usize {
    let mut intersections = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            if intersect_within_2d(&lines[i], &lines[j], min, max) {
                intersections += 1;
            }
        }
    }
    intersections
}

fn gaussian_elimination(matrix: &mut Matrix) -> Vec<f64> {
    for i in 0..matrix.len() {
        let t = matrix[i][i];
        matrix[i].iter_mut().for_each(|x| *x /= t);
        for j in (i + 1)..matrix.len() {
            let t = matrix[j][i];
            for k in 0..matrix[i].len() {
                matrix[j][k] -= t * matrix[i][k];
            }
        }
    }

    for i in (0..matrix.len()).rev() {
        for j in 0..i {
            let t = matrix[j][i];
            for k in 0..matrix[i].len() {
                matrix[j][k] -= t * matrix[i][k];
            }
        }
    }

    matrix.iter().map(|v| *v.last().unwrap()).collect()
}

struct Point {
    x: f64,
    y: f64,
    z: f64,
}
type Line = (Point, Point);
type Lines = Vec<Line>;
type Matrix = Vec<Vec<f64>>;

#[test]
pub fn test() {
    let input = vec![
        "19, 13, 30 @ -2, 1, -2",
        "18, 19, 22 @ -1, -1, -2",
        "20, 25, 34 @ -2, -2, -4",
        "12, 31, 28 @ -1, -2, -1",
        "20, 19, 15 @ 1, -5, -3",
    ];

    let lines = parse_input(&input);
    assert!(intersect_within_2d(&lines[0], &lines[1], 7.0, 27.0));
    assert!(intersect_within_2d(&lines[0], &lines[2], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[0], &lines[3], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[0], &lines[4], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[1], &lines[2], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[1], &lines[3], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[1], &lines[4], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[2], &lines[3], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[2], &lines[4], 7.0, 27.0));
    assert!(!intersect_within_2d(&lines[3], &lines[4], 7.0, 27.0));
    assert_eq!(count_intersections_2d(&lines, 7.0, 27.0), 2);

    // assert_eq!(a(&input), "2");
    // assert_eq!(b(&input), "47");
}
