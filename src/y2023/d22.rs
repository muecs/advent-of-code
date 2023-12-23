//! Day 22: Sand Slabs

use std::collections::HashSet;

/// number of bricks not solely supporting any others
pub fn a(input: &Vec<&str>) -> String {
    let bricks = parse_input(input);
    let (supports, supported_by) = simulate(&bricks);

    (0..bricks.len())
        .filter(|i| {
            supports[*i]
                .iter()
                .all(|j| supported_by[*j as usize].len() > 1)
        })
        .count()
        .to_string()
}

/// sum of mutually supporting bricks supported by stable base
pub fn b(input: &Vec<&str>) -> String {
    let bricks = parse_input(input);
    let (supports, supported_by) = simulate(&bricks);

    let support_counts = supported_by.iter().map(|s| s.len()).collect::<Vec<_>>();
    (0..bricks.len())
        .map(|i| {
            let mut supports_left = support_counts.clone();
            let mut stack = vec![i];
            let mut n = 0;
            while let Some(j) = stack.pop() {
                n += 1;
                for k in &supports[j] {
                    let k = *k as usize;
                    supports_left[k] -= 1;
                    if supports_left[k] == 0 {
                        stack.push(k);
                    }
                }
            }
            n - 1
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Bricks {
    let mut bricks: Bricks = input
        .iter()
        .map(|s| {
            s.split_once('~')
                .map(|(s1, s2)| (parse_point(s1), parse_point(s2)))
                .unwrap()
        })
        .collect();
    bricks.sort_unstable_by_key(|b| b.0.z);
    bricks
}

fn parse_point(s: &str) -> Point {
    Point {
        x: s[0..1].parse().unwrap(),
        y: s[2..3].parse().unwrap(),
        z: s[4..].parse().unwrap(),
    }
}

fn simulate(bricks: &Bricks) -> (Vec<Support>, Vec<Support>) {
    const DIM: u8 = 10;
    let mut heightmap = [(0u16, u16::MAX); (DIM * DIM) as usize];
    let mut supports = vec![Support::new(); bricks.len()];
    let mut supported_by = vec![Support::new(); bricks.len()];

    for (idx, brick) in bricks.iter().enumerate() {
        assert!(brick.0.x <= brick.1.x);
        assert!(brick.0.y <= brick.1.y);
        assert!(brick.0.z <= brick.1.z);

        // first find highest 'peaks' under the brick
        let mut max_z = 0;
        for x in brick.0.x..=brick.1.x {
            for y in brick.0.y..=brick.1.y {
                let z = heightmap[(x + y * DIM) as usize].0;
                if z > max_z {
                    max_z = z;
                }
            }
        }

        // then add the brick on top and update support links
        let height = brick.1.z + 1 - brick.0.z;
        for x in brick.0.x..=brick.1.x {
            for y in brick.0.y..=brick.1.y {
                let (z, prev_idx) = &mut heightmap[(x + y * DIM) as usize];
                if *z == max_z && *prev_idx != u16::MAX {
                    supported_by[idx].insert(*prev_idx);
                    supports[*prev_idx as usize].insert(idx as u16);
                }
                *z = max_z + height;
                *prev_idx = idx as u16;
            }
        }
    }

    (supports, supported_by)
}

struct Point {
    x: u8,
    y: u8,
    z: u16,
}
type Brick = (Point, Point);
type Bricks = Vec<Brick>;
type Support = HashSet<u16>;

#[test]
pub fn test() {
    let input = vec![
        "1,0,1~1,2,1",
        "0,0,2~2,0,2",
        "0,2,3~2,2,3",
        "0,0,4~0,2,4",
        "2,0,5~2,2,5",
        "0,1,6~2,1,6",
        "1,1,8~1,1,9",
    ];

    assert_eq!(a(&input), "5");
    assert_eq!(b(&input), "7");
}
