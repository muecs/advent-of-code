#!/bin/bash
set -e

YEAR=$1
if [ -z "$YEAR" ]; then
    echo "Please specify year as first argument."
    exit 1
fi

echo "Generating templates for $YEAR..."

mkdir -p "y$YEAR" && cd "y$YEAR"

rm -f mod.rs
for d in {1..25}; do
    printf "// mod d%02d;\n" $d >> mod.rs
done

echo '
use crate::{args::PartValues::{self, *}, SolveFunc};

pub fn solver(day: u8, part: PartValues) -> SolveFunc {
    match (day, part) {' >> mod.rs

for d in {1..25}; do
    printf "        // (%02d, A) => d%02d::a,\n" $d $d >> mod.rs
    if [ $d -lt 25 ]; then
        printf "        // (%02d, B) => d%02d::b,\n" $d $d >> mod.rs
    fi

    echo "//! Day $d

/// part a
pub fn a(_input: &Vec<&str>) -> String {
    String::new()
}

/// part b
pub fn b(_input: &Vec<&str>) -> String {
    String::new()
}

// fn parse_input(input: &Vec<&str>) {}

#[test]
pub fn test() {
    let input = vec![];

    assert_eq!(a(&input), \"\");
    assert_eq!(b(&input), \"\");
}" > $(printf "d%02d.rs" $d)

done

echo "\
        _ => unimplemented!("Unsupported puzzle: {day} {part:?}"),
    }
}" >> mod.rs

git add mod.rs #d??.rs

echo 'Done. Please add to main.rs manually.'
