//! Day 11: Reactor

use std::collections::{HashMap, HashSet};

/// Number of unique paths from `you` to `out`
pub fn a(input: &Vec<&str>) -> String {
    let graph = parse_input(input);
    count_paths(&graph, "you", "out").to_string()
}

/// Number of unique paths from `svr` to `out` via `dac` and `fft`
pub fn b(input: &Vec<&str>) -> String {
    let graph = parse_input(input);

    // count the middle paths first, either `fft` -> `dac` or `dac` -> `fft`
    // assumptions: `svr` is the root node and graph is acyclic
    let (n, u, v) = match count_paths(&graph, "fft", "dac") {
        0 => (count_paths(&graph, "dac", "fft"), "dac", "fft"),
        n => (n, "fft", "dac"),
    };

    // `svr` -> u -> v -> `out`
    (count_paths(&graph, "svr", u) * n * count_paths(&graph, v, "out")).to_string()
}

fn parse_input<'a>(input: &[&'a str]) -> HashMap<&'a str, Vec<&'a str>> {
    input
        .iter()
        .map(|s| (&s[0..3], s[5..].split_whitespace().collect()))
        .collect()
}

/// Count unique paths through DAG from `start` to `end`
fn count_paths(graph: &HashMap<&str, Vec<&str>>, start: &str, end: &str) -> usize {
    let mut counts = HashMap::new();
    let mut visited = HashSet::new();
    let mut stack = vec![(start, false)];

    while let Some((u, processed)) = stack.pop() {
        if processed {
            let count = if u == end {
                1
            } else {
                graph
                    .get(u)
                    .into_iter()
                    .flatten()
                    .map(|v| counts.get(v).copied().unwrap_or(0))
                    .sum()
            };
            counts.insert(u, count);
        } else if visited.insert(u) {
            stack.push((u, true));
            if let Some(neighbors) = graph.get(u) {
                stack.extend(neighbors.iter().map(|&v| (v, false)));
            }
        }
    }

    *counts.get(start).unwrap()
}

#[test]
pub fn test() {
    let input1 = vec![
        "aaa: you hhh",
        "you: bbb ccc",
        "bbb: ddd eee",
        "ccc: ddd eee fff",
        "ddd: ggg",
        "eee: out",
        "fff: out",
        "ggg: out",
        "hhh: ccc fff iii",
        "iii: out",
    ];

    assert_eq!(a(&input1), "5");

    let input2 = vec![
        "svr: aaa bbb",
        "aaa: fft",
        "fft: ccc",
        "bbb: tty",
        "tty: ccc",
        "ccc: ddd eee",
        "ddd: hub",
        "hub: fff",
        "eee: dac",
        "dac: fff",
        "fff: ggg hhh",
        "ggg: out",
        "hhh: out",
    ];

    assert_eq!(b(&input2), "2");
}
