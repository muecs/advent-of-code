//! Day 18: Boiling Boulders

use std::collections::BTreeSet;

/// surface area of voxel shape
pub fn a(input: &Vec<&str>) -> String {
    let voxels = parse_input(input);
    voxels
        .iter()
        .map(|p| 6 - count_adjacent(p, &voxels))
        .sum::<usize>()
        .to_string()
}

/// exterior surface area of voxel shape
pub fn b(input: &Vec<&str>) -> String {
    let voxels = parse_input(input);
    let (min_voxel, max_voxel) = voxels.iter().fold(((i8::MAX, i8::MAX, i8::MAX), (0, 0, 0)), |(acc_min, acc_max), p| {
        (
            (acc_min.0.min(p.0), acc_min.1.min(p.1), acc_min.2.min(p.2)),
            (acc_max.0.max(p.0), acc_max.1.max(p.1), acc_max.2.max(p.2))
        )
    });
    let mut outer_voxels = Voxels::new();
    let mut unvisited = vec![(min_voxel.0 - 1, min_voxel.1 - 1, min_voxel.2 - 1)];
    while let Some(p) = unvisited.pop() {
        if voxels.contains(&p) || outer_voxels.contains(&p) {
            continue;
        }
        outer_voxels.insert(p);
        let adjacent = get_adjacent(&p);
        for adj in &adjacent {
            if adj.0 >= min_voxel.0 - 1
                && adj.0 <= max_voxel.0 + 1
                && adj.1 >= min_voxel.1 - 1
                && adj.1 <= max_voxel.1 + 1
                && adj.2 >= min_voxel.2 - 1
                && adj.2 <= max_voxel.2 + 1
            {
                unvisited.push(*adj);
            }
        }
    }

    voxels
        .iter()
        .map(|p| count_adjacent(p, &outer_voxels))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Voxels {
    input
        .iter()
        .map(|line| {
            let mut it = line.split(',').map(|s| s.parse().unwrap());
            (it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
        })
        .collect()
}

fn get_adjacent(p: &Point) -> Vec<Point> {
    const OFFSETS: [i8; 2] = [-1, 1];
    let mut points = Vec::new();
    for x in OFFSETS {
        points.push((p.0 + x, p.1, p.2));
    }
    for y in OFFSETS {
        points.push((p.0, p.1 + y, p.2));
    }
    for z in OFFSETS {
        points.push((p.0, p.1, p.2 + z));
    }
    points
}

fn count_adjacent(p: &Point, voxels: &Voxels) -> usize {
    get_adjacent(p)
        .iter()
        .filter(|adj| voxels.contains(adj))
        .count()
}

type Point = (i8, i8, i8);
type Voxels = BTreeSet<Point>;

#[test]
pub fn test() {
    let input = vec![
        "2,2,2", "1,2,2", "3,2,2", "2,1,2", "2,3,2", "2,2,1", "2,2,3", "2,2,4", "2,2,6", "1,2,5",
        "3,2,5", "2,1,5", "2,3,5",
    ];

    let test_input = Voxels::from([(1, 1, 1), (2, 1, 1)]);
    assert_eq!(count_adjacent(&(1, 1, 1), &test_input), 1);
    assert_eq!(count_adjacent(&(2, 1, 1), &test_input), 1);
    assert_eq!(count_adjacent(&(3, 1, 1), &test_input), 1);
    assert_eq!(count_adjacent(&(2, 2, 2), &test_input), 0);

    assert_eq!(a(&input), "64");
    assert_eq!(b(&input), "58");
}
