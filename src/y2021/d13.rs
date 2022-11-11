//! Day 13: Transparent Origami

type Point = (usize, usize);
type Points = Vec<Point>;

type Fold = (usize, bool);
type Folds = Vec<Fold>;

/// number of unique points after first fold
pub fn a(input: &Vec<&str>) -> String {
    let (mut points, folds) = parse_input(input);
    apply_fold(&mut points, &folds[0]);
    points.len().to_string()
}

/// eight capital letters after all folds
pub fn b(input: &Vec<&str>) -> String {
    let (mut points, folds) = parse_input(input);
    for fold in &folds {
        apply_fold(&mut points, &fold);
    }
    let max = points.iter().fold((0, 0), |acc, p| (
        std::cmp::max(acc.0, p.0),
        std::cmp::max(acc.1, p.1),
    ));
    let mut canvas = vec![vec!['.'; max.0 + 1]; max.1 + 1];
    for point in &points {
        canvas[point.1][point.0] = '#';
    }
    "\n".to_owned() + &canvas
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_input(input: &Vec<&str>) -> (Points, Folds) {
    let mut points = Points::new();
    let mut folds = Folds::new();
    for &s in input {
        if s.starts_with("fold along ") {
            let is_x = s.chars().nth(11).unwrap() == 'x';
            let val = s[13..].parse().unwrap();
            folds.push((val, is_x));
        } else if let Some((x, y)) = s.split_once(',') {
            points.push((
                x.parse().unwrap(),
                y.parse().unwrap(),
            ));
        }
    }

    (points, folds)
}

fn apply_fold(points: &mut Points, fold: &Fold) {
    points.iter_mut().for_each(|point| {
        if fold.1 {
            if point.0 > fold.0 {
                point.0 = 2 * fold.0 - point.0;
            }
        } else {
            if point.1 > fold.0 {
                point.1 = 2 * fold.0 - point.1;
            }
        }
    });

    points.sort_unstable();
    points.dedup();
}

#[test]
pub fn test() {
    let input = vec![
        "6,10",
        "0,14",
        "9,10",
        "0,3",
        "10,4",
        "4,11",
        "6,0",
        "6,12",
        "4,1",
        "0,13",
        "10,12",
        "3,4",
        "3,0",
        "8,4",
        "1,10",
        "2,14",
        "8,10",
        "9,0",
        "",
        "fold along y=7",
        "fold along x=5",
    ];

    assert_eq!(a(&input), "17");
    assert_eq!(b(&input), vec![
        "",
        "#####",
        "#...#",
        "#...#",
        "#...#",
        "#####",
    ].join("\n"));
}
