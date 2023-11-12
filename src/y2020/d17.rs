//! Day 17: Conway Cubes

use std::collections::{HashMap, HashSet};

/// cube configuration after 6 cycles
pub fn a(input: &Vec<&str>) -> String {
    let mut cubes = parse_input(input);
    for _ in 0..6 {
        simulate(&mut cubes, false);
    }

    cubes.len().to_string()
}

/// hypercube configuration after 6 cycles
pub fn b(input: &Vec<&str>) -> String {
    let mut cubes = parse_input(input);
    for _ in 0..6 {
        simulate(&mut cubes, true);
    }

    cubes.len().to_string()
}

fn parse_input(input: &Vec<&str>) -> CubeSet {
    input
        .iter()
        .enumerate()
        .map(|(y, s)| {
            s.chars().enumerate().filter_map(move |(x, c)| {
                (c == '#').then_some((x as isize, y as isize, 0isize, 0isize))
            })
        })
        .flatten()
        .collect()
}

fn simulate(cubes: &mut CubeSet, hyper: bool) {
    let mut activate = CubeVec::new();
    let mut deactivate = CubeVec::new();

    let mut inactives = HashMap::<Point, usize>::new();

    // check current state
    for cube in cubes.iter() {
        let mut neighbors = CubeVec::with_capacity(if hyper { 80 } else { 26 });
        for x in -1..=1 {
            for y in -1..=1 {
                for z in -1..=1 {
                    for w in if hyper { -1..=1 } else { 0..=0 } {
                        if x != 0 || y != 0 || z != 0 || w != 0 {
                            neighbors.push((cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w));
                        }
                    }
                }
            }
        }

        let mut actives = 0usize;
        for neighbor in neighbors.iter() {
            if cubes.contains(neighbor) {
                actives += 1;
            } else {
                inactives
                    .entry(*neighbor)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }

        // active remains active if 2 or 3 neighbors are active
        if actives < 2 || actives > 3 {
            deactivate.push(*cube);
        }
    }

    // inactive becomes active if 3 neighbors are active
    for (cube, n) in inactives.iter() {
        if *n == 3 {
            activate.push(*cube);
        }
    }

    // transition to next state
    for cube in &deactivate {
        cubes.remove(cube);
    }
    for cube in activate {
        cubes.insert(cube);
    }
}

type Point = (isize, isize, isize, isize);
type CubeVec = Vec<Point>;
type CubeSet = HashSet<Point>;

#[test]
pub fn test() {
    let input = vec![".#.", "..#", "###"];

    let cubes = parse_input(&input);
    assert_eq!(cubes.len(), 5);
    assert!([
        (1, 0, 0, 0),
        (2, 1, 0, 0),
        (0, 2, 0, 0),
        (1, 2, 0, 0),
        (2, 2, 0, 0)
    ]
    .iter()
    .all(|p| cubes.contains(p)));

    {
        let mut cubes_a = cubes.clone();

        simulate(&mut cubes_a, false);
        assert_eq!(cubes_a.len(), 11);

        simulate(&mut cubes_a, false);
        assert_eq!(cubes_a.len(), 21);

        simulate(&mut cubes_a, false);
        assert_eq!(cubes_a.len(), 38);
    }

    assert_eq!(a(&input), "112");

    {
        let mut cubes_b = cubes.clone();

        simulate(&mut cubes_b, true);
        assert_eq!(cubes_b.len(), 29);

        simulate(&mut cubes_b, true);
        assert_eq!(cubes_b.len(), 60);
    }

    assert_eq!(b(&input), "848");
}
