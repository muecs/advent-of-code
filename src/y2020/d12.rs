//! Day 12: Rain Risk

/// Manhattan distance to final position
pub fn a(input: &Vec<&str>) -> String {
    let instructions = process_input(input);
    let position = calculate_position(&instructions);
    (position.0.abs() + position.1.abs()).to_string()
}

/// Manhattan distance to final position, using waypoint
pub fn b(input: &Vec<&str>) -> String {
    let instructions = process_input(input);
    let position = calculate_position_with_waypoint(&instructions);
    (position.0.abs() + position.1.abs()).to_string()
}

fn process_input(input: &Vec<&str>) -> Vec<Instruction> {
    input
        .iter()
        .map(|&s| {
            (
                match &s[0..1] {
                    "N" => Action::North,
                    "S" => Action::South,
                    "E" => Action::East,
                    "W" => Action::West,
                    "L" => Action::Left,
                    "R" => Action::Right,
                    "F" => Action::Forward,
                    _ => unreachable!("invalid action symbol"),
                },
                s[1..].parse().unwrap(),
            )
        })
        .collect()
}

fn calculate_position(instructions: &Vec<Instruction>) -> (isize, isize) {
    let mut angle = 0.0;
    let mut position = (0, 0);

    for (action, value) in instructions {
        match action {
            Action::North => position.1 += value,
            Action::South => position.1 -= value,
            Action::East => position.0 += value,
            Action::West => position.0 -= value,
            Action::Left => angle += *value as f64,
            Action::Right => angle -= *value as f64,
            Action::Forward => {
                let (sin, cos) = angle.to_radians().sin_cos();
                position.0 += (cos * *value as f64) as isize;
                position.1 += (sin * *value as f64) as isize;
            }
        }
        #[cfg(test)]
        println!("{:?}", position);
    }

    position
}

fn calculate_position_with_waypoint(instructions: &Vec<Instruction>) -> (isize, isize) {
    fn rotate(p: &mut Point, angle: f64) {
        let (sin, cos) = angle.to_radians().sin_cos();
        *p = (
            (cos * p.0 as f64 - sin * p.1 as f64).round() as isize,
            (sin * p.0 as f64 + cos * p.1 as f64).round() as isize,
        )
    }

    let mut vector = (10, 1);
    let mut position = (0, 0);

    for (action, value) in instructions {
        match action {
            Action::North => vector.1 += value,
            Action::South => vector.1 -= value,
            Action::East => vector.0 += value,
            Action::West => vector.0 -= value,
            Action::Left => rotate(&mut vector, *value as f64),
            Action::Right => rotate(&mut vector, -*value as f64),
            Action::Forward => {
                position.0 += vector.0 * value;
                position.1 += vector.1 * value;
            }
        }
        #[cfg(test)]
        println!("{:?}", position);
    }

    position
}

enum Action {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

type Instruction = (Action, isize);
type Point = (isize, isize);

#[test]
pub fn test() {
    let input = vec!["F10", "N3", "F7", "R90", "F11"];

    assert_eq!(a(&input), "25");
    assert_eq!(b(&input), "286");
}
