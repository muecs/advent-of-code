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
        _ => unimplemented!("Unsupported puzzle: {day} {part:?}"),
    }
}
