//! Day 11: Monkey in the Middle

use std::collections::VecDeque;

/// product of number of inspected items by 2 most active monkeys after 20 rounds
pub fn a(input: &Vec<&str>) -> String {
    let mut monkeys = parse_input(input);
    for _ in 0..20 {
        play_round(&mut monkeys, true);
    }
    monkeys.sort_unstable_by(|a, b| b.count.cmp(&a.count));
    (monkeys[0].count * monkeys[1].count).to_string()
}

/// 10000 rounds with custom relief mechanism
pub fn b(input: &Vec<&str>) -> String {
    let mut monkeys = parse_input(input);
    for _ in 0..10000 {
        play_round(&mut monkeys, false);
    }
    monkeys.sort_unstable_by(|a, b| b.count.cmp(&a.count));
    (monkeys[0].count * monkeys[1].count).to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Monkey> {
    input
        .chunks(7)
        .map(|lines| Monkey {
            items: lines[1][18..]
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect(),
            operation: {
                let s = &lines[2][23..];
                if s == "* old" {
                    Box::new(|val| val * val)
                } else {
                    let operand: usize = s[2..].parse().unwrap();
                    match &s[0..1] {
                        "*" => Box::new(move |val| val * operand),
                        "+" => Box::new(move |val| val + operand),
                        _ => unreachable!("unsupported operation"),
                    }
                }
            },
            test: lines[3][21..].parse().unwrap(),
            if_true: lines[4][29..].parse().unwrap(),
            if_false: lines[5][30..].parse().unwrap(),
            count: 0,
        })
        .collect()
}

fn play_round(monkeys: &mut Vec<Monkey>, relief: bool) {
    let test_multiple: usize = monkeys.iter().map(|m| m.test).product();  // lcm?
    for i in 0..monkeys.len() {
        monkeys[i].count += monkeys[i].items.len();
        while let Some(mut item) = monkeys[i].items.pop_front() {
            item = (monkeys[i].operation)(&item);
            if relief {
                item /= 3;
            } else {
                item %= test_multiple;
            }
            let destination = if item % monkeys[i].test == 0 {
                monkeys[i].if_true
            } else {
                monkeys[i].if_false
            };
            monkeys[destination].items.push_back(item);
        }
    }
}

struct Monkey {
    /// worry level for each item in inspection order
    items: VecDeque<usize>,
    /// how worry level changes on inspection
    operation: Box<dyn Fn(&usize) -> usize>,
    /// where to throw item next
    test: usize,
    /// destination if test was true
    if_true: usize,
    /// destination if test was false
    if_false: usize,
    /// number of items inspected
    count: usize,
}

#[test]
pub fn test() {
    let input = vec![
        "Monkey 0:",
        "  Starting items: 79, 98",
        "  Operation: new = old * 19",
        "  Test: divisible by 23",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 3",
        "",
        "Monkey 1:",
        "  Starting items: 54, 65, 75, 74",
        "  Operation: new = old + 6",
        "  Test: divisible by 19",
        "    If true: throw to monkey 2",
        "    If false: throw to monkey 0",
        "",
        "Monkey 2:",
        "  Starting items: 79, 60, 97",
        "  Operation: new = old * old",
        "  Test: divisible by 13",
        "    If true: throw to monkey 1",
        "    If false: throw to monkey 3",
        "",
        "Monkey 3:",
        "  Starting items: 74",
        "  Operation: new = old + 3",
        "  Test: divisible by 17",
        "    If true: throw to monkey 0",
        "    If false: throw to monkey 1",
    ];

    assert_eq!(a(&input), "10605");
    assert_eq!(b(&input), "2713310158");
}
