//! Day 2: Cube Conundrum

/// sum of IDs of possible games
pub fn a(input: &Vec<&str>) -> String {
    let games = parse_input(input);
    games
        .iter()
        .filter_map(|(id, subsets)| {
            let max_rgb = subsets.iter().fold([0; 3], |mut acc, subset| {
                (0..3).for_each(|i| acc[i] = acc[i].max(subset[i]));
                acc
            });
            (max_rgb[0] <= 12 && max_rgb[1] <= 13 && max_rgb[2] <= 14).then_some(id)
        })
        .sum::<usize>()
        .to_string()
}

/// sum of products of min possible cube counts
pub fn b(input: &Vec<&str>) -> String {
    let games = parse_input(input);
    games
        .iter()
        .map(|(_, subsets)| {
            let max_rgb = subsets.iter().fold([0; 3], |mut acc, subset| {
                (0..3).for_each(|i| acc[i] = acc[i].max(subset[i]));
                acc
            });
            max_rgb.iter().map(|x| *x as usize).product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Games {
    input
        .iter()
        .map(|s| {
            s.split_once(": ")
                .map(|(s1, s2)| {
                    (
                        s1[5..].parse::<usize>().unwrap(),
                        s2.split("; ")
                            .map(|subsets| {
                                let mut rgb: RGB = [0; 3];
                                subsets
                                    .split(", ")
                                    .for_each(|cubes| match cubes.split_once(' ') {
                                        Some((n, "red")) => rgb[0] += n.parse::<u8>().unwrap(),
                                        Some((n, "green")) => rgb[1] += n.parse::<u8>().unwrap(),
                                        Some((n, "blue")) => rgb[2] += n.parse::<u8>().unwrap(),
                                        Some((_, _)) => unreachable!(),
                                        None => unreachable!(),
                                    });
                                rgb
                            })
                            .collect::<Vec<RGB>>(),
                    )
                })
                .unwrap()
        })
        .collect()
}

type RGB = [u8; 3];
type Games = Vec<(usize, Vec<RGB>)>;

#[test]
pub fn test() {
    let input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    assert_eq!(a(&input), "8");
    assert_eq!(b(&input), "2286");
}
