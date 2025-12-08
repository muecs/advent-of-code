//! Day 8: Playground

use std::collections::{BinaryHeap, HashMap};

/// product of sizes of 3 largest circuits of 1000 (10) shortest connections
pub fn a(input: &Vec<&str>) -> String {
    #[cfg(test)]
    const MAX_CONN: usize = 10;
    #[cfg(not(test))]
    const MAX_CONN: usize = 1000;

    let points = parse_input(input);
    let (circuits, _) = build_circuits(&points, MAX_CONN);

    // find 3 largest circuits
    let mut freq = HashMap::new();
    for &circuit in &circuits {
        *freq.entry(circuit).or_insert(0usize) += 1;
    }
    let mut freq_vec = freq.values().cloned().collect::<Vec<_>>();
    freq_vec.sort_unstable_by_key(|count| std::cmp::Reverse(*count));    
    freq_vec
        .iter()
        .take(3)
        .product::<usize>()
        .to_string()
}

/// product of x coords of last connection to build full circuit
pub fn b(input: &Vec<&str>) -> String {
    let points = parse_input(input);
    let (_, last_conn) = build_circuits(&points, usize::MAX);
    last_conn.map(|(a, b)| points[a].x * points[b].x).unwrap().to_string()
}

fn parse_input(input: &Vec<&str>) -> Points {
    input
        .iter()
        .map(|line| {
            let parts: Vec<isize> = line
                .split(',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect();
            Point {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect()
}

fn build_circuits(points: &Points, max_conn: usize) -> (Circuits, Option<(usize, usize)>) {
    let n = points.len();
    let mut connections = BinaryHeap::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = distance_sq(&points[i], &points[j]);
            connections.push((std::cmp::Reverse(dist), i, j));
        }
    }

    // build circuits from shortest connections
    // (basically Kruskal's algorithm)
    let mut circuits = (0..n).collect::<Circuits>();
    let mut count = 0;
    while count < max_conn && let Some((_, p1, p2)) = connections.pop() {
        count += 1;
        let mut c1 = circuits[p1];
        let mut c2 = circuits[p2];
        if c1 == c2 {
            continue;
        }

        // merge circuits
        if c1 > c2 {
            std::mem::swap(&mut c1, &mut c2);
        }
        let mut size = 0;
        for c in circuits.iter_mut() {
            if *c == c2 {
                *c = c1;
                size += 1;
            } else if *c == c1 {
                size += 1;
            }
        }

        if size == n {
            return (circuits, Some((p1, p2)));
        }
    }

    (circuits, None)
}

#[inline]
fn distance_sq(p1: &Point, p2: &Point) -> isize {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    let dz = p1.z - p2.z;
    dx * dx + dy * dy + dz * dz
}

struct Point {
    x: isize,
    y: isize,
    z: isize,
}
type Points = Vec<Point>;
type Circuits = Vec<usize>;

#[test]
pub fn test() {
    let input = vec![
        "162,817,812",
        "57,618,57",
        "906,360,560",
        "592,479,940",
        "352,342,300",
        "466,668,158",
        "542,29,236",
        "431,825,988",
        "739,650,466",
        "52,470,668",
        "216,146,977",
        "819,987,18",
        "117,168,530",
        "805,96,715",
        "346,949,466",
        "970,615,88",
        "941,993,340",
        "862,61,35",
        "984,92,344",
        "425,690,689",
    ];

    assert_eq!(a(&input), "40");
    assert_eq!(b(&input), "25272");
}
