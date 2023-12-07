//! Day 7: Camel Cards

/// sum of winnings of each hand
pub fn a(input: &Vec<&str>) -> String {
    let mut hands = parse_input(input, false);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
        .to_string()
}

/// sum of winnings of each hand with joker
pub fn b(input: &Vec<&str>) -> String {
    let mut hands = parse_input(input, true);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) * hand.bid)
        .sum::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>, allow_jokers: bool) -> Hands {
    input
        .iter()
        .map(|s| {
            let mut multiples = [0u8; 14]; // 13 cards, with double role for J
            let mut wildcards = 0;
            Hand {
                cards: s[0..5]
                    .bytes()
                    .map(|b| {
                        let card = match b {
                            // starting with 0 for joker, then 1 for 2 etc
                            b'A' => 13,
                            b'K' => 12,
                            b'Q' => 11,
                            b'J' => allow_jokers.then_some(0).unwrap_or(10),
                            b'T' => 9,
                            b'2'..=b'9' => b + 1 - b'2',
                            _ => unreachable!("invalid card"),
                        };
                        if allow_jokers && card == 0 {
                            wildcards += 1;
                        } else {
                            multiples[card as usize] += 1;
                        }
                        card
                    })
                    .collect::<Vec<u8>>()
                    .try_into()
                    .unwrap(),
                htype: {
                    multiples.sort_unstable_by_key(|b| u8::MAX - b);
                    multiples[0] += wildcards; // top up most frequent card
                    match multiples {
                        [5, ..] => 6,    // five of a kind
                        [4, ..] => 5,    // four of a kind
                        [3, 2, ..] => 4, // full house
                        [3, 1, ..] => 3, // three of a kind
                        [2, 2, ..] => 2, // two pair
                        [2, 1, ..] => 1, // one pair
                        _ => 0,          // high card
                    }
                },
                bid: s[6..].parse().unwrap(),
            }
        })
        .collect()
}

type Cards = [u8; 5];
#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Hand {
    // sort by hand type first, then cards
    htype: u8,
    cards: Cards,
    bid: usize,
}
type Hands = Vec<Hand>;

#[test]
pub fn test() {
    let input = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483",
    ];

    {
        let hands = parse_input(&input, false);
        assert_eq!(hands.len(), 5);
        assert_eq!(hands[0].cards, [2, 1, 9, 2, 12]);
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].cards, [9, 4, 4, 10, 4]);
        assert_eq!(hands[1].bid, 684);
        assert_eq!(
            hands.iter().map(|hand| hand.htype).collect::<Vec<_>>(),
            vec![1, 3, 2, 2, 3]
        );
    }

    assert_eq!(a(&input), "6440");

    {
        let hands = parse_input(&input, true);
        assert_eq!(hands.len(), 5);
        assert_eq!(hands[0].cards, [2, 1, 9, 2, 12]);
        assert_eq!(hands[0].bid, 765);
        assert_eq!(hands[1].cards, [9, 4, 4, 0, 4]);
        assert_eq!(hands[1].bid, 684);
        assert_eq!(
            hands.iter().map(|hand| hand.htype).collect::<Vec<_>>(),
            vec![1, 5, 2, 5, 5]
        );
    }
    assert_eq!(b(&input), "5905");
}
