//! Day 4: Scratchcards

/// sum of winning score of each scratch card
pub fn a(input: &Vec<&str>) -> String {
    let cards = parse_input(input);
    cards
        .iter()
        .map(|card| score(count_matches(card)))
        .sum::<usize>()
        .to_string()
}

/// number of scratch cards including winning multiplier
pub fn b(input: &Vec<&str>) -> String {
    let cards = parse_input(input);
    let mut instances = vec![1; cards.len()];
    for i in 0..cards.len() {
        let winners = count_matches(&cards[i]);
        for j in 1..=winners {
            // sic: don't need to check for overflow
            instances[i + j] += instances[i];
        }
    }
    instances.iter().sum::<usize>().to_string()
}

fn parse_input(input: &Vec<&str>) -> Cards {
    input
        .iter()
        .map(|s| {
            s.split_once(": ")
                .unwrap()
                .1
                .split_once(" | ")
                .map(|(win, have)| {
                    let mut card: Card = (
                        win.split_whitespace().map(|n| n.parse().unwrap()).collect(),
                        have.split_whitespace()
                            .map(|n| n.parse().unwrap())
                            .collect(),
                    );
                    card.0.sort_unstable();
                    card
                })
                .unwrap()
        })
        .collect()
}

fn count_matches((win, have): &Card) -> usize {
    have.iter().filter(|n| win.binary_search(n).is_ok()).count()
}

fn score(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        2_usize.pow(n as u32 - 1)
    }
}

type Card = (Vec<u8>, Vec<u8>);
type Cards = Vec<Card>;

#[test]
pub fn test() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    assert_eq!(score(0), 0);
    assert_eq!(score(1), 1);
    assert_eq!(score(4), 8);

    assert_eq!(a(&input), "13");
    assert_eq!(b(&input), "30");
}
