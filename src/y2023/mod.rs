mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
// mod d13;
// mod d14;
// mod d15;
// mod d16;
// mod d17;
// mod d18;
// mod d19;
// mod d20;
// mod d21;
// mod d22;
// mod d23;
// mod d24;
// mod d25;

use crate::args::PartValues::{self, *};

pub fn solve(day: u8, part: PartValues, input: &Vec<&str>) -> String {
    match (day, part) {
        (01, A) => d01::a(&input),
        (01, B) => d01::b(&input),
        (02, A) => d02::a(&input),
        (02, B) => d02::b(&input),
        (03, A) => d03::a(&input),
        (03, B) => d03::b(&input),
        (04, A) => d04::a(&input),
        (04, B) => d04::b(&input),
        (05, A) => d05::a(&input),
        (05, B) => d05::b(&input),
        (06, A) => d06::a(&input),
        (06, B) => d06::b(&input),
        (07, A) => d07::a(&input),
        (07, B) => d07::b(&input),
        (08, A) => d08::a(&input),
        (08, B) => d08::b(&input),
        (09, A) => d09::a(&input),
        (09, B) => d09::b(&input),
        (10, A) => d10::a(&input),
        (10, B) => d10::b(&input),
        (11, A) => d11::a(&input),
        (11, B) => d11::b(&input),
        (12, A) => d12::a(&input),
        (12, B) => d12::b(&input),
        // (13, A) => d13::a(&input),
        // (13, B) => d13::b(&input),
        // (14, A) => d14::a(&input),
        // (14, B) => d14::b(&input),
        // (15, A) => d15::a(&input),
        // (15, B) => d15::b(&input),
        // (16, A) => d16::a(&input),
        // (16, B) => d16::b(&input),
        // (17, A) => d17::a(&input),
        // (17, B) => d17::b(&input),
        // (18, A) => d18::a(&input),
        // (18, B) => d18::b(&input),
        // (19, A) => d19::a(&input),
        // (19, B) => d19::b(&input),
        // (20, A) => d20::a(&input),
        // (20, B) => d20::b(&input),
        // (21, A) => d21::a(&input),
        // (21, B) => d21::b(&input),
        // (22, A) => d22::a(&input),
        // (22, B) => d22::b(&input),
        // (23, A) => d23::a(&input),
        // (23, B) => d23::b(&input),
        // (24, A) => d24::a(&input),
        // (24, B) => d24::b(&input),
        // (25, A) => d25::a(&input),
        _ => { println!("Unsupported puzzle: {day} {part:?}"); String::new() }
    }
}
