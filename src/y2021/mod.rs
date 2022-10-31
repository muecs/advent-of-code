mod d01;

use crate::args::PartValues::{self, *};

pub fn solve(day: u8, part: PartValues, input: &Vec<&str>) -> String {
    match (day, part) {
        (01, A) => d01::a(&input),
        (01, B) => d01::b(&input),
        _ => format!("Unsupported puzzle: {} {:?}", day, part)
    }
}
