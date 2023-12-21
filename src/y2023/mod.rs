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
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
// mod d22;
// mod d23;
// mod d24;
// mod d25;

use crate::{args::PartValues::{self, *}, SolveFunc};

pub fn solver(day: u8, part: PartValues) -> SolveFunc {
    match (day, part) {
        (01, A) => d01::a,
        (01, B) => d01::b,
        (02, A) => d02::a,
        (02, B) => d02::b,
        (03, A) => d03::a,
        (03, B) => d03::b,
        (04, A) => d04::a,
        (04, B) => d04::b,
        (05, A) => d05::a,
        (05, B) => d05::b,
        (06, A) => d06::a,
        (06, B) => d06::b,
        (07, A) => d07::a,
        (07, B) => d07::b,
        (08, A) => d08::a,
        (08, B) => d08::b,
        (09, A) => d09::a,
        (09, B) => d09::b,
        (10, A) => d10::a,
        (10, B) => d10::b,
        (11, A) => d11::a,
        (11, B) => d11::b,
        (12, A) => d12::a,
        (12, B) => d12::b,
        (13, A) => d13::a,
        (13, B) => d13::b,
        (14, A) => d14::a,
        (14, B) => d14::b,
        (15, A) => d15::a,
        (15, B) => d15::b,
        (16, A) => d16::a,
        (16, B) => d16::b,
        (17, A) => d17::a,
        (17, B) => d17::b,
        (18, A) => d18::a,
        (18, B) => d18::b,
        (19, A) => d19::a,
        (19, B) => d19::b,
        (20, A) => d20::a,
        (20, B) => d20::b,
        (21, A) => d21::a,
        (21, B) => d21::b,
        // (22, A) => d22::a,
        // (22, B) => d22::b,
        // (23, A) => d23::a,
        // (23, B) => d23::b,
        // (24, A) => d24::a,
        // (24, B) => d24::b,
        // (25, A) => d25::a,
        _ => unimplemented!("Unsupported puzzle: {day} {part:?}"),
    }
}
