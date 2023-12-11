//! Day 11: Cosmic Expansion

/// sum of distances with 2x expansion
pub fn a(input: &Vec<&str>) -> String {
    let galaxies = parse_input(input);
    calc_distance_sum(&galaxies, 2).to_string()
}

/// sum of distances with 1000000x expansion
pub fn b(input: &Vec<&str>) -> String {
    let galaxies = parse_input(input);
    calc_distance_sum(&galaxies, 1000000).to_string()
}

fn parse_input(input: &Vec<&str>) -> Galaxies {
    let mut col_empty = vec![true; input[0].len()];
    let mut ry = 0;
    let mut galaxies = Galaxies::new();

    for y in 0..input.len() {
        let mut row_empty = true;
        for (x, b) in input[y].bytes().enumerate() {
            if b == b'#' {
                galaxies.push(Galaxy {
                    x: x as u8,
                    y: y as u8,
                    rx: 0,
                    ry,
                });
                col_empty[x] = false;
                row_empty = false;
            }
        }
        if row_empty {
            ry += 1
        }
    }

    galaxies.iter_mut().for_each(|g| {
        g.rx = col_empty
            .iter()
            .take(g.x as usize)
            .fold(0, |acc, b| if *b { acc + 1 } else { acc })
    });

    galaxies
}

fn calc_distance_sum(galaxies: &Galaxies, expansion_factor: usize) -> usize {
    assert!(expansion_factor > 0);
    let expansion_factor = expansion_factor - 1;
    let mut sum = 0;
    for i in 0..galaxies.len() - 1 {
        for j in i + 1..galaxies.len() {
            let (x0, rx0, x1, rx1) = if galaxies[i].x > galaxies[j].x {
                (galaxies[j].x, galaxies[j].rx, galaxies[i].x, galaxies[i].rx)
            } else {
                (galaxies[i].x, galaxies[i].rx, galaxies[j].x, galaxies[j].rx)
            };

            let (y0, ry0, y1, ry1) = if galaxies[i].y > galaxies[j].y {
                (galaxies[j].y, galaxies[j].ry, galaxies[i].y, galaxies[i].ry)
            } else {
                (galaxies[i].y, galaxies[i].ry, galaxies[j].y, galaxies[j].ry)
            };

            let x_dist = (x1 - x0) as usize + (rx1 - rx0) as usize * expansion_factor;
            let y_dist = (y1 - y0) as usize + (ry1 - ry0) as usize * expansion_factor;
            sum += x_dist + y_dist; // Manhattan
        }
    }
    sum
}

#[derive(Debug, PartialEq)]
struct Galaxy {
    x: u8,
    y: u8,
    rx: u8,
    ry: u8,
}
type Galaxies = Vec<Galaxy>;

#[test]
pub fn test() {
    let input = vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#.....",
    ];

    let galaxies = parse_input(&input);
    assert_eq!(galaxies.len(), 9);
    assert_eq!(galaxies[0], Galaxy { x: 3, y: 0, rx: 1, ry: 0 });

    assert_eq!(a(&input), "374");

    assert_eq!(calc_distance_sum(&galaxies, 10), 1030);
    assert_eq!(calc_distance_sum(&galaxies, 100), 8410);

    // assert_eq!(b(&input), "");
}
