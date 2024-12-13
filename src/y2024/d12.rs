//! Day 12: Garden Groups

/// sum of products of area and perimeter of each contiguous region
pub fn a(input: &Vec<&str>) -> String {
    calculate_fence_price(input, false).to_string()
}

/// sum of products of area and sides of each contiguous region
pub fn b(input: &Vec<&str>) -> String {
    calculate_fence_price(input, true).to_string()
}

fn calculate_fence_price(input: &Vec<&str>, with_sides: bool) -> usize {
    let w = input[0].len();
    let h = input.len();

    // 2 levels of BFS to flood-fill areas of same type
    let mut regions = vec![(0usize, 0usize)];
    let mut visited = vec![vec![false; input[0].len()]; input.len()];
    let mut cost = 0;
    while let Some(region) = regions.pop() {
        if visited[region.1][region.0] {
            continue;
        }
        let mut pending = vec![region];
        let mut area = 0;
        let mut perimeter = 0;
        let mut sides = 0;
        let region_type = input[region.1].as_bytes()[region.0];
        while let Some(pos) = pending.pop() {
            if visited[pos.1][pos.0] {
                continue;
            }
            area += 1;
            visited[pos.1][pos.0] = true;

            let dir = [(1isize, 0isize), (0, 1), (-1, 0), (0, -1)];
            let mut is_fence = vec![true; 4];
            for (i, &(dx, dy)) in dir.iter().enumerate() {
                // check grid boundaries
                if (pos.0 == 0 && dx < 0) || (pos.0 + 1 == w && dx > 0) {
                    perimeter += 1;
                    continue;
                }
                if (pos.1 == 0 && dy < 0) || (pos.1 + 1 == h && dy > 0) {
                    perimeter += 1;
                    continue;
                }

                let next = (
                    pos.0.checked_add_signed(dx).unwrap(),
                    pos.1.checked_add_signed(dy).unwrap(),
                );
                let next_type = input[next.1].as_bytes()[next.0];
                let next_visited = visited[next.1][next.0];
                if next_type == region_type {
                    // matching plant type
                    is_fence[i] = false;
                    if !next_visited {
                        pending.push(next);
                    }
                } else {
                    // different plant type, need fence
                    perimeter += 1;
                    if !next_visited {
                        regions.push(next);
                    }
                }
            }

            if with_sides {
                // check how many corners this plot has
                for i in 0..4 {
                    let j = (i + 1) % 4;
                    if is_fence[i] && is_fence[j] {
                        // convex corner
                        sides += 1;
                    } else if !is_fence[i]
                        && !is_fence[j]
                        && input[pos.1.checked_add_signed(dir[i].1 + dir[j].1).unwrap()].as_bytes()
                            [pos.0.checked_add_signed(dir[i].0 + dir[j].0).unwrap()]
                            != region_type
                    {
                        // concave corner
                        sides += 1;
                    }
                }
            }
        }
        // println!("region {} has area {area} and perimeter {perimeter} and sides {sides}", region_type as char);
        cost += area * if with_sides { sides } else { perimeter };
    }
    cost
}

#[test]
pub fn test() {
    let input1 = vec!["AAAA", "BBCD", "BBCC", "EEEC"];

    let input2 = vec!["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"];

    let input3 = vec![
        "RRRRIICCFF",
        "RRRRIICCCF",
        "VVRRRCCFFF",
        "VVRCCCJFFF",
        "VVVVCJJCFE",
        "VVIVCCJJEE",
        "VVIIICJJEE",
        "MIIIIIJJEE",
        "MIIISIJEEE",
        "MMMISSJEEE",
    ];

    let input4 = vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"];

    let input5 = vec!["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"];

    assert_eq!(a(&input1), "140");
    assert_eq!(a(&input2), "772");
    assert_eq!(a(&input3), "1930");

    assert_eq!(b(&input1), "80");
    assert_eq!(b(&input2), "436");
    assert_eq!(b(&input4), "236");
    assert_eq!(b(&input5), "368");
    assert_eq!(b(&input3), "1206");
}
