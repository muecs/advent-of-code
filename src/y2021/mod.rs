mod d01;
mod d02;
mod d03;

use crate::args::PartValues::{self, *};

pub fn solve(day: u8, part: PartValues, input: &Vec<&str>) -> String {
    match (day, part) {
        (01, A) => d01::a(&input),
        (01, B) => d01::b(&input),
        (02, A) => d02::a(&input),
        (02, B) => d02::b(&input),
        (03, A) => d03::a(&input),
        (03, B) => d03::b(&input),
        _ => { println!("Unsupported puzzle: {day} {part:?}"); String::new() }
    }
}
