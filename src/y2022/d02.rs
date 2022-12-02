//! Day 2: Rock Paper Scissors

/// total score given move
pub fn a(input: &Vec<&str>) -> String {
    parse_input(input, false)
        .iter()
        .map(|shapes| score_round(shapes))
        .sum::<usize>()
        .to_string()
}

/// total score given strategy
pub fn b(input: &Vec<&str>) -> String {
    parse_input(input, true)
        .iter()
        .map(|shapes| score_round(shapes))
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>, is_strategy: bool) -> Vec<(Shape, Shape)> {
    input
        .iter()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(s1, s2)| {
            let move1 = Shape::try_from(s1.bytes().next().unwrap() - b'A').unwrap();
            let move2 = if is_strategy {
                match s2 {
                    "X" => Shape::try_from((move1 as u8 + 2) % 3).unwrap(),
                    "Y" => move1,
                    "Z" => Shape::try_from((move1 as u8 + 1) % 3).unwrap(),
                    _ => unreachable!("invalid shape"),
                }
            } else {
                Shape::try_from(s2.bytes().next().unwrap() - b'X').unwrap()
            };
            (move1, move2)
        })
        .collect::<Vec<_>>()
}

fn score_round(shapes: &(Shape, Shape)) -> usize {
    // Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock
    let outcome = if shapes.0 == shapes.1 {
        3
    } else {
        match shapes.1 {
            Shape::Rock => if shapes.0 == Shape::Scissors { 6 } else { 0 },
            Shape::Paper => if shapes.0 == Shape::Rock { 6 } else { 0 },
            Shape::Scissors => if shapes.0 == Shape::Paper { 6 } else { 0 },
        }
    };

    outcome + shapes.1 as usize + 1
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl TryFrom<u8> for Shape {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Rock),
            1 => Ok(Self::Paper),
            2 => Ok(Self::Scissors),
            _ => Err("invalid shape index"),
        }
    }
}

#[test]
pub fn test() {
    let input = vec![
        "A Y",
        "B X",
        "C Z",
    ];

    assert_eq!(a(&input), "15");
    assert_eq!(b(&input), "12");
}
