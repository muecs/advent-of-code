//! Day 20: Trench Map

type Lookup = Vec<bool>;
type Grid = Vec<Vec<bool>>;
type Point = (i32, i32);

/// number of set pixels after 2 enhancements
pub fn a(input: &Vec<&str>) -> String {
    let (lookup, image) = parse_input(input);
    let enhanced_image = enhance_image(&image, &lookup, 2);
    count_set_pixels(&enhanced_image).to_string()
}

/// number of set pixels after 50 enhancements
pub fn b(input: &Vec<&str>) -> String {
    let (lookup, image) = parse_input(input);
    let enhanced_image = enhance_image(&image, &lookup, 50);
    count_set_pixels(&enhanced_image).to_string()
}

fn parse_input(input: &Vec<&str>) -> (Lookup, Grid) {
    let mut it = input.iter();
    let lookup = it
        .next()
        .unwrap()
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>();
    assert_eq!(lookup.len(), 512);
    it.next();  // empty line
    let image = it
        .map(|&s| s.chars().map(|c| c == '#').collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (lookup, image)
}

fn get_pixel_index(image: &Grid, point: &Point) -> usize {
    let height = image.len() as i32;
    let width = image[0].len() as i32;
    let mut window = Vec::with_capacity(9);
    for y in point.1-1..=point.1+1 {
        for x in point.0-1..=point.0+1 {
            window.push(
                x >= 0 
                && x < width 
                && y >= 0 
                && y < height 
                && image[y as usize][x as usize]
            );
        }
    }
    window.iter().fold(0usize, |acc, &bit| (acc << 1) + bit as usize)
}

fn enhance_image(image: &Grid, lookup: &Lookup, steps: u8) -> Grid {
    let height = image.len() as i32;
    let width = image[0].len() as i32;
    let padding = 2 * steps as i32;

    let mut new_image = Grid::new();
    for y in -padding .. height + padding {
        let mut row = Vec::new();
        for x in -padding .. width + padding {
            let pixel = lookup[get_pixel_index(&image, &(x, y))];
            row.push(pixel);
        }
        new_image.push(row);
    }

    for i in 1..steps as i32 {
        let image = new_image;
        new_image = Grid::new();

        for y in 1 .. height + 2 * (padding - i) {
            let mut row = Vec::new();
            for x in 1 .. width + 2 * (padding - i) {
                let pixel = lookup[get_pixel_index(&image, &(x, y))];
                row.push(pixel);
            }
            new_image.push(row);
        }

        // println!("Enhancement #{}:", i + 1);
        // new_image
        //     .iter()
        //     .map(|row| row.iter().map(|&px| if px { '#' } else { '.' }).collect::<String>())
        //     .for_each(|s| println!("  {}", s));
    }

    new_image
}

fn count_set_pixels(image: &Grid) -> usize {
    image
        .iter()
        .map(|row| row.iter().fold(0usize, |acc, px| acc + *px as usize))
        .sum::<usize>()
}

#[test]
pub fn test() {
    let input = vec![
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
         #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
         .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
         .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
         .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
         ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
         ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
        "",
        "#..#.",
        "#....",
        "##..#",
        "..#..",
        "..###",
    ];

    let (lookup, image) = parse_input(&input);

    assert_eq!(get_pixel_index(&image, &(2, 2)), 34);
    assert!(lookup[34]);

    assert_eq!(a(&input), "35");
    assert_eq!(b(&input), "3351");
}
