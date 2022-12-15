//! Day 13: Distress Signal

use std::cmp::Ordering;

/// sum of indices of pairs in right order
pub fn a(input: &Vec<&str>) -> String {
    let packets = parse_input(input);
    packets
        .chunks_exact(2)
        .enumerate()
        .filter_map(|(i, pair)| compare(&pair[0], &pair[1]).is_lt().then(|| i + 1))
        .sum::<usize>()
        .to_string()
}

/// product of indices of divider packets in sorted packet list
pub fn b(input: &Vec<&str>) -> String {
    let mut packets = parse_input(input);

    let divider2 = tokenize("[[2]]");
    let divider6 = tokenize("[[6]]");
    packets.push(divider2.to_owned());
    packets.push(divider6.to_owned());
    packets.sort_unstable_by(compare);

    packets
        .iter()
        .enumerate()
        .filter_map(|(i, packet)| {
            (compare(packet, &divider2).is_eq() || compare(packet, &divider6).is_eq())
                .then(|| i + 1)
        })
        .product::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Packet> {
    input
        .iter()
        .filter_map(|line| line.is_empty().eq(&false).then(|| tokenize(line)))
        .collect()
}

fn tokenize(s: &str) -> Packet {
    let mut packet = Packet::new();
    let mut digits = Vec::new();

    for c in s.chars() {
        match c {
            '[' => packet.push(Token::Open),
            ']' | ',' => {
                if !digits.is_empty() {
                    packet.push(Token::Literal(
                        digits.iter().collect::<String>().parse().unwrap(),
                    ));
                    digits = Vec::new();
                }
                if c == ']' {
                    packet.push(Token::Close);
                }
            }
            '0'..='9' => digits.push(c),
            _ => unreachable!("invalid character"),
        }
    }

    packet
}

fn compare(left: &Packet, right: &Packet) -> Ordering {
    // turn into mutable stacks
    let mut left = left.iter().rev().collect::<Vec<_>>();
    let mut right = right.iter().rev().collect::<Vec<_>>();

    while let (Some(l), Some(r)) = (left.pop(), right.pop()) {
        let left_closes = *l == Token::Close;
        let right_closes = *r == Token::Close;

        if left_closes && !right_closes {
            return Ordering::Less;
        } else if right_closes && !left_closes {
            return Ordering::Greater;
        } else if left_closes && right_closes {
            // ignore equality
        } else {
            match (l, r) {
                (Token::Open, Token::Literal(_)) => {
                    right.push(&Token::Close);
                    right.push(r);
                }
                (Token::Literal(_), Token::Open) => {
                    left.push(&Token::Close);
                    left.push(l);
                }
                (Token::Literal(x), Token::Literal(y)) if x != y => {
                    return x.cmp(y);
                }
                _ => {}
            }
        }
    }

    Ordering::Equal
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Token {
    Open,
    Close,
    Literal(u8),
}

type Packet = Vec<Token>;

#[test]
pub fn test() {
    let input = vec![
        "[1,1,3,1,1]",
        "[1,1,5,1,1]",
        "",
        "[[1],[2,3,4]]",
        "[[1],4]",
        "",
        "[9]",
        "[[8,7,6]]",
        "",
        "[[4,4],4,4]",
        "[[4,4],4,4,4]",
        "",
        "[7,7,7,7]",
        "[7,7,7]",
        "",
        "[]",
        "[3]",
        "",
        "[[[]]]",
        "[[]]",
        "",
        "[1,[2,[3,[4,[5,6,7]]]],8,9]",
        "[1,[2,[3,[4,[5,6,0]]]],8,9]",
    ];

    assert_eq!(
        tokenize(&input[3]),
        vec![
            Token::Open,
            Token::Open,
            Token::Literal(1),
            Token::Close,
            Token::Open,
            Token::Literal(2),
            Token::Literal(3),
            Token::Literal(4),
            Token::Close,
            Token::Close,
        ]
    );

    assert_eq!(
        tokenize("[1,10,7,10]"),
        vec![
            Token::Open,
            Token::Literal(1),
            Token::Literal(10),
            Token::Literal(7),
            Token::Literal(10),
            Token::Close,
        ]
    );

    assert_eq!(
        compare(&tokenize(&input[0]), &tokenize(&input[1])),
        Ordering::Less
    );
    assert_eq!(
        compare(&tokenize(&input[3]), &tokenize(&input[4])),
        Ordering::Less
    );
    assert_eq!(
        compare(&tokenize(&input[6]), &tokenize(&input[7])),
        Ordering::Greater
    );
    assert_eq!(
        compare(&tokenize(&input[9]), &tokenize(&input[10])),
        Ordering::Less
    );
    assert_eq!(
        compare(&tokenize(&input[12]), &tokenize(&input[13])),
        Ordering::Greater
    );
    assert_eq!(
        compare(&tokenize(&input[15]), &tokenize(&input[16])),
        Ordering::Less
    );
    assert_eq!(
        compare(&tokenize(&input[18]), &tokenize(&input[19])),
        Ordering::Greater
    );
    assert_eq!(
        compare(&tokenize(&input[21]), &tokenize(&input[22])),
        Ordering::Greater
    );

    assert_eq!(
        compare(
            &tokenize("[[6],[7],[9,[0,[8,6]],[9]]]"),
            &tokenize("[[[[6],[],[2,10],0],[8,[4,10],[4,5,8,0,0]]],[3,1],[[10]]]")
        ),
        Ordering::Less
    );

    assert_eq!(
        compare(&tokenize("[1,[2,[3,4]]]"), &tokenize("[1,[2,[3,4]]]")),
        Ordering::Equal
    );

    assert_eq!(a(&input), "13");
    assert_eq!(b(&input), "140");
}
