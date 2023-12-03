//! Day 3: Gear Ratios

/// sum of part numbers next to symbols
pub fn a(input: &Vec<&str>) -> String {
    let (symbols, numbers) = parse_input(input);
    numbers
        .iter()
        .filter_map(|(x, y, num)| {
            symbols
                .iter()
                .any(|(sx, sy, _)| {
                    sy + 1 >= *y && *sy <= y + 1 && sx + 1 >= *x && *sx <= x + num.len()
                })
                .then_some(num.parse::<usize>().unwrap())
        })
        .sum::<usize>()
        .to_string()
}

/// sum of product of numbers adjacent to '*'
pub fn b(input: &Vec<&str>) -> String {
    let (symbols, numbers) = parse_input(input);
    symbols
        .iter()
        .filter_map(|(sx, sy, b)| {
            if *b != b'*' {
                return None;
            }
            let gears = numbers
                .iter()
                .filter_map(|(x, y, num)| {
                    (sy + 1 >= *y && *sy <= y + 1 && sx + 1 >= *x && *sx <= x + num.len())
                        .then_some(num.parse::<usize>().unwrap())
                })
                .collect::<Vec<_>>();
            (gears.len() == 2).then_some(gears.iter().product::<usize>())
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> (Symbols, Numbers<'a>) {
    let mut symbols = Symbols::new();
    let mut numbers = Numbers::new();
    for (y, row) in input.iter().enumerate() {
        let mut x0_opt = None;
        for (x, b) in row.bytes().enumerate() {
            match b {
                b'0'..=b'9' => {
                    if x0_opt.is_none() {
                        x0_opt = Some(x)
                    }
                }
                _ => {
                    if b != b'.' {
                        symbols.push((x, y, b));
                    }
                    if let Some(x0) = x0_opt {
                        numbers.push((x0, y, &row[x0..x]));
                        x0_opt = None;
                    }
                }
            }
        }
        if let Some(x0) = x0_opt {
            numbers.push((x0, y, &row[x0..]));
        }
    }
    (symbols, numbers)
}

type Symbols = Vec<(usize, usize, u8)>; // x, y, sym
type Numbers<'a> = Vec<(usize, usize, &'a str)>; // x, y, num

#[test]
pub fn test() {
    let input = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598..",
    ];

    let (symbols, numbers) = parse_input(&input);
    assert_eq!(
        symbols,
        vec![
            (3, 1, b'*'),
            (6, 3, b'#'),
            (3, 4, b'*'),
            (5, 5, b'+'),
            (3, 8, b'$'),
            (5, 8, b'*')
        ]
    );
    assert_eq!(numbers.len(), 10);
    assert_eq!(numbers[0], (0, 0, "467"));
    assert_eq!(numbers[1], (5, 0, "114"));
    assert_eq!(numbers[2], (2, 2, "35"));
    assert_eq!(numbers[3], (6, 2, "633"));

    assert_eq!(a(&input), "4361");
    assert_eq!(b(&input), "467835");
}
