//! Day 14: Parabolic Reflector Dish

use std::collections::HashMap;

/// total load after tilting north
pub fn a(input: &Vec<&str>) -> String {
    let mut height = vec![0; input[0].len()];
    let mut weight = 0;
    for y in 0..input.len() {
        for (x, b) in input[y].bytes().enumerate() {
            match b {
                b'O' => {
                    weight += input.len() - height[x];
                    height[x] += 1;
                }
                b'#' => {
                    height[x] = y + 1;
                }
                _ => {}
            }
        }
    }
    weight.to_string()
}

/// total load after 1000000000 N/W/S/E tilt cycles
pub fn b(input: &Vec<&str>) -> String {
    const N: usize = 1000000000;
    let mut grid = parse_input(input);
    let mut cache = HashMap::<Grid, usize>::new();
    for i in 0..N {
        for _ in 0..4 {
            turn_clockwise(&mut grid);
            tilt_east(&mut grid);
        }
        if let Some(cycle) = cache.get(&grid) {
            let length = i - cycle;
            // println!(
            //     "after {} cycles, loop starts at {} with length {}",
            //     i + 1,
            //     cycle,
            //     length
            // );
            let final_cycle = (N - cycle - 1) % length + cycle;
            grid = cache
                .iter()
                .find_map(|(grid, &c)| (c == final_cycle).then_some(grid.clone()))
                .unwrap();
            break;
        } else {
            cache.insert(grid.clone(), i);
        }
    }

    // turn once more as we need the North load
    turn_clockwise(&mut grid);
    calculate_east_load(&grid).to_string()
}

fn parse_input(input: &Vec<&str>) -> Grid {
    input.iter().map(|s| s.bytes().collect()).collect()
}

fn tilt_east(grid: &mut Grid) {
    grid.iter_mut().for_each(|row| {
        row.split_mut(|&b| b == b'#').for_each(|subrow| {
            subrow.sort_unstable();
        });
    });
}

fn turn_clockwise(grid: &mut Grid) {
    assert!(!grid.is_empty());
    let max = grid.len() - 1;
    for y in 0..grid.len() / 2 {
        for x in y..max - y {
            let tmp = grid[y][x];
            grid[y][x] = grid[max - x][y];
            grid[max - x][y] = grid[max - y][max - x];
            grid[max - y][max - x] = grid[x][max - y];
            grid[x][max - y] = tmp;
        }
    }
}

fn calculate_east_load(grid: &Grid) -> usize {
    grid.iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter_map(|(x, &b)| (b == b'O').then_some(x + 1))
                .sum::<usize>()
        })
        .sum::<usize>()
}

type Grid = Vec<Vec<u8>>;

#[test]
pub fn test() {
    let input = vec![
        "O....#....",
        "O.OO#....#",
        ".....##...",
        "OO.#O....O",
        ".O.....O#.",
        "O.#..O.#.#",
        "..O..#O..O",
        ".......O..",
        "#....###..",
        "#OO..#....",
    ];

    let turned_input = vec![
        "##..O.O.OO",
        "O....OO...",
        "O..O#...O.",
        "......#.O.",
        "......O.#.",
        "##.#O..#.#",
        ".#.O...#..",
        ".#O.#O....",
        ".....#....",
        "...O#.O.#.",
    ];

    let cycled_input = vec![
        ".....#....",
        "....#...O#",
        "...OO##...",
        ".OO#......",
        ".....OOO#.",
        ".O#...O#.#",
        "....O#....",
        "......OOOO",
        "#...O###..",
        "#..OO#....",
    ];

    assert_eq!(a(&input), "136");

    let mut grid = parse_input(&input);
    turn_clockwise(&mut grid);
    assert_eq!(grid, parse_input(&turned_input));

    tilt_east(&mut grid);
    assert_eq!(calculate_east_load(&grid), 136);

    for _ in 0..3 {
        turn_clockwise(&mut grid);
        tilt_east(&mut grid);
    }
    assert_eq!(grid, parse_input(&cycled_input));

    assert_eq!(b(&input), "64");
}
