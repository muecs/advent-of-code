//! Day 8: Haunted Wasteland

use crate::utils;
use std::collections::HashMap;

/// steps required to reach goal
pub fn a(input: &Vec<&str>) -> String {
    let graph = parse_graph(input);
    count_steps("AAA", &input[0], &graph, |node| node == "ZZZ").to_string()
}

/// simultaneous steps required to reach goals
pub fn b(input: &Vec<&str>) -> String {
    let graph = parse_graph(input);
    let steps = graph
        .keys()
        .filter_map(|node| {
            node.ends_with('A')
                .then(|| count_steps(node, &input[0], &graph, |n| n.ends_with('Z')))
        })
        .collect::<Vec<_>>();
    // println!("steps: {:?}", steps);
    utils::lcm(&steps).to_string()
}

fn parse_graph<'a>(input: &'a Vec<&'a str>) -> Graph<'a> {
    input
        .iter()
        .skip(2)
        .map(|s| (&s[0..3], (&s[7..10], &s[12..15])))
        .collect()
}

fn count_steps(start: &str, directions: &str, graph: &Graph, condition: fn(&str) -> bool) -> usize {
    let mut pos = start;
    let mut steps = 0;
    let mut dir_it = directions.bytes().cycle();
    while !condition(pos) {
        pos = match dir_it.next() {
            Some(b'L') => graph[pos].0,
            Some(b'R') => graph[pos].1,
            _ => unreachable!(),
        };
        steps += 1;
    }
    steps
}

type Graph<'a> = HashMap<&'a str, (&'a str, &'a str)>;

#[test]
pub fn test() {
    let input = vec![
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    let input2 = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];

    let input3 = vec![
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    assert_eq!(a(&input), "2");
    assert_eq!(a(&input2), "6");
    assert_eq!(b(&input3), "6");
}
