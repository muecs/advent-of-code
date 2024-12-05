//! Day 5: Print Queue

use std::collections::{HashMap, HashSet};

/// sum of middle page numbers in valid sequences
pub fn a(input: &Vec<&str>) -> String {
    let (rules, sequences) = parse_input(input);
    sequences
        .iter()
        .filter_map(|seq| {
            check_sequence(seq, &rules).then(|| seq[seq.len() / 2].parse::<usize>().unwrap())
        })
        .sum::<usize>()
        .to_string()
}

/// sum of middle page numbers in corrected invalid sequences
pub fn b(input: &Vec<&str>) -> String {
    let (rules, sequences) = parse_input(input);
    sequences
        .iter()
        .filter_map(|seq| (!check_sequence(seq, &rules)).then(|| find_correct_mid(seq, &rules)))
        .sum::<usize>()
        .to_string()
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> (Rules, Sequences) {
    let mut it = input.iter();
    let rules = it.by_ref().map_while(|s| s.split_once('|')).collect();
    let seq = it.map(|s| s.split(',').collect()).collect();
    (rules, seq)
}

/// checks whether a page sequence follows the rules
fn check_sequence(seq: &Sequence, rules: &Rules) -> bool {
    let mut before = HashSet::new();
    let mut after = HashSet::<_>::from_iter(seq);
    for page in seq {
        after.remove(page);
        if rules
            .iter()
            .any(|(r1, r2)| (page == r1 && before.contains(r2)) || page == r2 && after.contains(r1))
        {
            return false;
        }
        before.insert(page);
    }
    true
}

/// finds the mid page in a correctly ordered page sequence
fn find_correct_mid(seq: &Sequence, rules: &Rules) -> usize {
    let mut count = 0;

    // build a page graph, ignore irrelevant rules
    let mut graph =
        HashMap::<_, _>::from_iter(seq.iter().map(|page| (*page, GraphNode::default())));
    for (r1, r2) in rules {
        if !graph.contains_key(r1) || !graph.contains_key(r2) {
            continue;
        }
        graph.get_mut(r1).unwrap().after.push(r2);
        graph.get_mut(r2).unwrap().before.insert(r1);
    }

    // list of pages with none before
    let mut stack = graph
        .iter()
        .filter_map(|(page, node)| node.before.is_empty().then_some(*page))
        .collect::<Vec<_>>();

    // do a top sort, but just count until we get to the mid page
    while let Some(page) = stack.pop() {
        count += 1;
        if count > seq.len() / 2 {
            // found the mid
            return page.parse().unwrap();
        }
        while let Some(a) = graph.get_mut(page).unwrap().after.pop() {
            let after_node = graph.get_mut(&a).unwrap();
            after_node.before.remove(page);
            if after_node.before.is_empty() {
                stack.push(&a);
            }
        }
    }

    unreachable!()
}

type Rules<'a> = Vec<(&'a str, &'a str)>;
type Sequence<'a> = Vec<&'a str>;
type Sequences<'a> = Vec<Sequence<'a>>;

#[derive(Default)]
struct GraphNode<'a> {
    before: HashSet<&'a str>,
    after: Vec<&'a str>,
}

#[test]
pub fn test() {
    let input = vec![
        "47|53",
        "97|13",
        "97|61",
        "97|47",
        "75|29",
        "61|13",
        "75|53",
        "29|13",
        "97|29",
        "53|29",
        "61|53",
        "97|53",
        "61|29",
        "47|13",
        "75|47",
        "97|75",
        "47|61",
        "75|61",
        "47|29",
        "75|13",
        "53|13",
        "",
        "75,47,61,53,29",
        "97,61,53,29,13",
        "75,29,13",
        "75,97,47,61,53",
        "61,13,29",
        "97,13,75,29,47",
    ];

    assert_eq!(a(&input), "143");
    assert_eq!(b(&input), "123");
}
