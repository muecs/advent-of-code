//! Day 16: Ticket Translation

use std::{collections::BTreeSet, str::FromStr};

/// ticket scanning error rate
pub fn a(input: &Vec<&str>) -> String {
    let notes = parse_input(input);
    notes
        .nearby_tickets
        .iter()
        .map(|t| t.validate(&notes.rules))
        .sum::<u16>()
        .to_string()
}

/// product of `departure*` fields on own ticket
pub fn b(input: &Vec<&str>) -> String {
    let notes = parse_input(input);
    let valid_tickets = notes
        .nearby_tickets
        .iter()
        .filter(|t| t.is_valid(&notes.rules))
        .collect::<Vec<_>>();
    let n = valid_tickets.first().unwrap().numbers.len();

    let mut rule_matches = notes
        .rules
        .iter()
        .map(|rule| {
            (
                &rule.field,
                (0..n)
                    .filter(|pos| {
                        valid_tickets
                            .iter()
                            .all(|ticket| rule.is_valid(&ticket.numbers[*pos]))
                    })
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<Vec<_>>();

    rule_matches.sort_by_cached_key(|(_, m)| m.len());

    let mut prod = 1;
    let mut found = BTreeSet::<usize>::new();
    for rm in rule_matches {
        let pos = rm.1.difference(&found).next().unwrap();
        if rm.0.starts_with("departure") {
            prod *= notes.own_ticket.numbers[*pos] as usize;
        }
        println!("{}: #{}", rm.0, pos);
        found.insert(*pos);
    }
    
    prod.to_string()
}

fn parse_input(input: &Vec<&str>) -> Notes {
    let mut notes = Notes::default();
    let mut it = input.iter();

    while let Some(&line) = it.next() {
        if line.is_empty() {
            break;
        }
        notes.rules.push(line.parse().unwrap());
    }

    assert_eq!(it.next().unwrap(), &"your ticket:");

    notes.own_ticket = it.next().unwrap().parse().unwrap();

    assert!(it.next().unwrap().is_empty());

    assert_eq!(it.next().unwrap(), &"nearby tickets:");

    while let Some(&line) = it.next() {
        notes.nearby_tickets.push(line.parse().unwrap());
    }

    notes
}

type Range = (u16, u16);

struct Rule {
    field: String,
    ranges: [Range; 2],
}

impl Rule {
    fn is_valid(&self, n: &u16) -> bool {
        self.ranges.iter().any(|r| n >= &r.0 && n <= &r.1)
    }
}

impl FromStr for Rule {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (field, s) = s.split_once(": ").unwrap();
        let range_str = s.split_once(" or ").unwrap();
        Ok(Self {
            field: field.to_string(),
            ranges: [
                range_str
                    .0
                    .split_once('-')
                    .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
                    .unwrap(),
                range_str
                    .1
                    .split_once('-')
                    .map(|r| (r.0.parse().unwrap(), r.1.parse().unwrap()))
                    .unwrap(),
            ],
        })
    }
}

#[derive(Default)]
struct Ticket {
    numbers: Vec<u16>,
}

impl Ticket {
    // sum of numbers that don't match any rule
    fn validate(&self, rules: &Vec<Rule>) -> u16 {
        self.numbers
            .iter()
            .filter(|n| !rules.iter().any(|r| r.is_valid(n)))
            .sum()
    }

    // whether a ticket has matches for all rules
    fn is_valid(&self, rules: &Vec<Rule>) -> bool {
        self.numbers.iter().all(|n| rules.iter().any(|r| r.is_valid(n)))
    }
}

impl FromStr for Ticket {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Ticket {
            numbers: s.split(',').map(|n| n.parse().unwrap()).collect(),
        })
    }
}

#[derive(Default)]
struct Notes {
    rules: Vec<Rule>,
    own_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[test]
pub fn test() {
    let input = vec![
        "class: 1-3 or 5-7",
        "row: 6-11 or 33-44",
        "seat: 13-40 or 45-50",
        "",
        "your ticket:",
        "7,1,14",
        "",
        "nearby tickets:",
        "7,3,47",
        "40,4,50",
        "55,2,20",
        "38,6,12",
    ];

    let notes = parse_input(&input);
    assert_eq!(notes.rules.len(), 3);
    assert_eq!(notes.own_ticket.numbers.len(), 3);
    assert_eq!(notes.nearby_tickets.len(), 4);
    assert!(!notes.rules[0].is_valid(&0));
    assert!(notes.rules[0].is_valid(&1));
    assert!(notes.rules[0].is_valid(&3));
    assert!(!notes.rules[0].is_valid(&4));
    assert!(notes.rules[0].is_valid(&5));
    assert!(notes.rules[0].is_valid(&7));
    assert!(!notes.rules[0].is_valid(&8));
    assert!(notes.nearby_tickets[0].is_valid(&notes.rules));
    assert!(!notes.nearby_tickets[1].is_valid(&notes.rules));
    assert!(!notes.nearby_tickets[2].is_valid(&notes.rules));
    assert!(!notes.nearby_tickets[3].is_valid(&notes.rules));
    assert_eq!(notes.nearby_tickets[0].validate(&notes.rules), 0);
    assert_eq!(notes.nearby_tickets[1].validate(&notes.rules), 4);
    assert_eq!(notes.nearby_tickets[2].validate(&notes.rules), 55);
    assert_eq!(notes.nearby_tickets[3].validate(&notes.rules), 12);

    assert_eq!(a(&input), "71");

    let input2 = vec![
        "class: 0-1 or 4-19",
        "row: 0-5 or 8-19",
        "seat: 0-13 or 16-19",
        "",
        "your ticket:",
        "11,12,13",
        "",
        "nearby tickets:",
        "3,9,18",
        "15,1,5",
        "5,14,9",
    ];

    assert_eq!(b(&input2), "1");
}
