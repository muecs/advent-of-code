//! Day 22: Monkey Market

use std::collections::HashMap;

/// sum of 2000th secret numbers
pub fn a(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    numbers
        .iter()
        .map(|n| secrefy(*n, 2000))
        .sum::<usize>()
        .to_string()
}

/// best sequence of last digit price changes after which to sell
pub fn b(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    let mut sequences = Sequences::new();
    for n in numbers {
        let seq = build_sequences(n, 2000);
        for (changes, price) in seq {
            sequences
                .entry(changes)
                .and_modify(|e| *e += price)
                .or_insert(price);
        }
    }
    sequences.values().max().unwrap().to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

fn mix_and_prune(secret: usize, new: usize) -> usize {
    (new ^ secret) % 16777216
}

fn secrefy(mut val: usize, reps: usize) -> usize {
    for _ in 0..reps {
        val = mix_and_prune(val, val * 64);
        val = mix_and_prune(val, val / 32);
        val = mix_and_prune(val, val * 2048);
    }
    val
}

fn build_sequences(mut val: usize, reps: usize) -> Sequences {
    let mut prices = Vec::with_capacity(reps);
    prices.push((val % 10) as i8);
    for _ in 0..reps {
        val = mix_and_prune(val, val * 64);
        val = mix_and_prune(val, val / 32);
        val = mix_and_prune(val, val * 2048);
        prices.push((val % 10) as i8);
    }
    let mut sequences = Sequences::new();
    for p in prices.windows(5) {
        sequences
            .entry([p[1] - p[0], p[2] - p[1], p[3] - p[2], p[4] - p[3]])
            .or_insert(p[4] as usize);
    }
    sequences
}

type Sequences = HashMap<[i8; 4], usize>;

#[test]
pub fn test() {
    let input1 = vec!["1", "10", "100", "2024"];
    let input2 = vec!["1", "2", "3", "2024"];

    let n = 123;
    assert_eq!(secrefy(n, 1), 15887950);
    assert_eq!(secrefy(n, 2), 16495136);
    assert_eq!(secrefy(n, 3), 527345);
    assert_eq!(secrefy(n, 4), 704524);
    assert_eq!(secrefy(n, 5), 1553684);
    assert_eq!(secrefy(n, 6), 12683156);
    assert_eq!(secrefy(n, 7), 11100544);
    assert_eq!(secrefy(n, 8), 12249484);
    assert_eq!(secrefy(n, 9), 7753432);
    assert_eq!(secrefy(n, 10), 5908254);

    assert_eq!(secrefy(1, 2000), 8685429);

    let seq = build_sequences(n, 10);
    assert_eq!(seq.get(&[-1, -1, 0, 2]), Some(&6));

    assert_eq!(a(&input1), "37327623");
    assert_eq!(b(&input2), "23");
}
