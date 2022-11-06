//! Day 8: Seven Segment Search

use std::collections::BTreeSet;

//  aaaa
// b    c
// b    c
//  dddd
// e    f
// e    f
//  gggg

// 1 - cf      (2) *
// 7 - acf     (3) *
// 4 - bcdf    (4) *
// 2 - acdeg   (5)
// 3 - acdfg   (5)
// 5 - abdfg   (5)
// 0 - abcefg  (6)
// 6 - abdefg  (6)
// 9 - abcdfg  (6)
// 8 - abcdefg (7) *

/// count digits 1, 4, 7, or 8 in the output values
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|&line| parse_line(line)
            .1
            .iter()
            .filter(|&&s| [2usize, 3, 4, 7].contains(&s.len()))
            .count()
        )
        .sum::<usize>()
        .to_string()
}

/// sum of decoded output values
pub fn b(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|&line| {
            let (signals, outputs) = parse_line(line);
            let mut signal_digits = [1, 7, 4, -1, -1, -1, -1, -1, -1, 8];
            //                                \_ 2|3|5 _/ \_ 0|6|9 _/

            let one_chars = signals[0].chars().collect::<BTreeSet<_>>();
            let four_chars = signals[2].chars().collect::<BTreeSet<_>>();
            let mut six_chars = BTreeSet::new();
            
            // group of digits with 6 segments (0, 6, 9)
            // 9 contains all segments of 4; 0 contains all segments of 1
            for i in 6..=8 {
                let chars = signals[i].chars().collect::<BTreeSet<_>>();
                if chars.is_superset(&four_chars) {
                    signal_digits[i] = 9;
                } else if chars.is_superset(&one_chars) {
                    signal_digits[i] = 0;
                } else {
                    signal_digits[i] = 6;
                    six_chars = chars;
                }
            }

            // group of digits with 5 segments (2, 3, 5)
            // 3 contains all segments of 1, 6 contains all segments of 5
            for i in 3..=5 {
                let chars = signals[i].chars().collect::<BTreeSet<_>>();
                if chars.is_superset(&one_chars) {
                    signal_digits[i] = 3;
                } else if chars.is_subset(&six_chars) {
                    signal_digits[i] = 5;
                } else {
                    signal_digits[i] = 2;
                }
            }

            outputs
                .iter()
                .map(|&out| {
                    for i in 0..=9 {
                        if is_same_digit(out, signals[i]) {
                            return signal_digits[i].to_string();
                        }
                    }
                    String::new()
                })
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .sum::<usize>()
        .to_string()
}

/// turns input line into vectors of signal and output digit strings
fn parse_line(input: &str) -> (Vec<&str>, Vec<&str>) {
    let mut parts = input.split(" | ");

    let mut signals = parts
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();
    signals.sort_unstable_by(|&a, &b| a.len().cmp(&b.len()));

    let outputs = parts
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>();

    (signals, outputs)
}

/// check for same segments, regardless of ordering
fn is_same_digit(a: &str, b: &str) -> bool {
    let chars_a = a.chars().collect::<BTreeSet<_>>();
    let chars_b = b.chars().collect::<BTreeSet<_>>();
    chars_a == chars_b
}

#[test]
pub fn test() {
    let input = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    assert_eq!(a(&input), "26");
    assert_eq!(b(&input), "61229");
}
