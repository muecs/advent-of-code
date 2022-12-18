//! Day 16: Proboscidea Volcanium

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const START: &str = "AA";

/// maximum cumulative pressure released in 30 steps
pub fn a(input: &Vec<&str>) -> String {
    let (graph, distances) = parse_input(input);
    let results = find_total_pressure(&graph, &distances, 30, false);
    results
        .iter()
        .map(|s| s.total_pressure)
        .max()
        .unwrap()
        .to_string()
}

/// max cumulative pressure released by 2 actors in 26 steps each
pub fn b(input: &Vec<&str>) -> String {
    let (graph, distances) = parse_input(input);

    #[allow(unused_variables)]
    let intermediate = false;
    #[cfg(test)]
    let intermediate = true;

    let mut results = find_total_pressure(&graph, &distances, 26, intermediate);
    results.sort_unstable_by_key(|r| r.total_pressure);
    let min_pressure = results.last().unwrap().total_pressure;

    // find pairs of intermediate states that don't have the same valves open
    let mut max_pressure = 0;
    for i in (0..results.len()).rev() {
        // avoid pairs with total pressure below single actor optimum
        let start = match results
            .binary_search_by_key(&(min_pressure - results[i].total_pressure), |r| {
                r.total_pressure
            }) {
            Ok(v) => v,
            Err(v) => v,
        };
        for j in start..i {
            if results[i].opened_mask & results[j].opened_mask == 0 {
                let combined_pressure = results[i].total_pressure + results[j].total_pressure;
                if combined_pressure > max_pressure {
                    max_pressure = combined_pressure;
                }
            }
        }
    }

    max_pressure.to_string()
}

fn parse_input<'a>(input: &Vec<&'a str>) -> (Graph<'a>, DistanceCache<'a>) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnels? leads? to valves? (.*)$"
        )
        .unwrap();
    }

    let mut graph: Graph = input
        .iter()
        .map(|line| {
            let cap = RE.captures(line).unwrap();
            (
                cap.get(1).unwrap().as_str(),
                Valve {
                    rate: cap.get(2).unwrap().as_str().parse().unwrap(),
                    adjacent: cap
                        .get(3)
                        .unwrap()
                        .as_str()
                        .split(", ")
                        .map(|s| (s, 1))
                        .collect(),
                },
            )
        })
        .collect();

    // remove valves with zero flow rate
    let useless_ids = graph
        .iter()
        .filter_map(|(id, valve)| (valve.rate == 0 && *id != START).then(|| *id))
        .collect::<Vec<_>>();
    for id in &useless_ids {
        let valve = graph.remove(*id).unwrap();
        for (adj_id, adj_dist) in &valve.adjacent {
            graph.entry(adj_id).and_modify(|adj_valve| {
                adj_valve.adjacent.remove(id);
                for (other_id, other_dist) in &valve.adjacent {
                    if *other_id != *adj_id {
                        adj_valve.adjacent.insert(other_id, other_dist + adj_dist);
                    }
                }
            });
        }
    }

    // cache distances between all remaining valves
    let mut distances = HashMap::<(&'a str, &'a str), usize>::new();
    for (id, valve) in &graph {
        distances.insert((id, id), 0);
        for (adj_id, adj_dist) in &valve.adjacent {
            distances.insert((id, adj_id), *adj_dist);
        }
    }
    for k in graph.keys() {
        for i in graph.keys() {
            for j in graph.keys() {
                let d = distances
                    .get(&(i, k))
                    .unwrap_or(&usize::MAX)
                    .saturating_add(*distances.get(&(k, j)).unwrap_or(&usize::MAX));
                if d < *distances.get(&(i, j)).unwrap_or(&usize::MAX) {
                    distances.insert((i, j), d);
                }
            }
        }
    }

    (graph, distances)
}

fn find_total_pressure<'a>(
    graph: &Graph<'a>,
    distances: &DistanceCache<'a>,
    max_steps: usize,
    include_intermediate: bool,
) -> Vec<Result> {
    let all_valves = graph
        .iter()
        .filter_map(|(id, v)| v.rate.gt(&0).then(|| *id))
        .collect::<HashSet<_>>();
    let mut states = vec![State {
        pos: START,
        unopened: all_valves.to_owned(),
        remaining: max_steps,
        total_pressure: 0,
    }];

    let mut results = Vec::new();

    while let Some(state) = states.pop() {
        /*
        if state.remaining == 0 {
            results.push(state.total_pressure);
            continue;
        }

        let valve = &graph[&state.pos];

        for (id, dist) in &valve.adjacent {
            if *dist > state.remaining {
                continue;
            }
            states.push(State {
                pos: id,
                unopened: state.unopened.clone(),
                remaining: state.remaining - dist,
                total_pressure: state.total_pressure,
            });
        }

        if state.unopened.contains(&state.pos) {
            let mut unopened = state.unopened.to_owned();
            unopened.remove(&state.pos);
            let remaining = state.remaining - 1;
            states.push(State {
                pos: state.pos,
                unopened,
                remaining,
                total_pressure: state.total_pressure + remaining * valve.rate,
            });
        }
        */

        let mut states_added = 0usize;
        for id in &state.unopened {
            let dist = distances[&(state.pos, *id)] + 1;
            if dist >= state.remaining {
                continue;
            }

            let mut unopened = state.unopened.to_owned();
            unopened.remove(id);
            let remaining = state.remaining - dist;
            states.push(State {
                pos: id,
                unopened,
                remaining,
                total_pressure: state.total_pressure + remaining * graph[id].rate,
            });
            states_added += 1;
        }

        if states_added == 0 || include_intermediate {
            let opened_mask = all_valves.iter().enumerate().fold(0usize, |acc, (i, id)| {
                if !state.unopened.contains(id) {
                    acc + (1 << i)
                } else {
                    acc
                }
            });
            results.push(Result {
                opened_mask,
                total_pressure: state.total_pressure,
            });
        }
    }

    results
}

#[derive(Debug)]
struct Valve<'a> {
    rate: usize,
    adjacent: HashMap<&'a str, usize>,
}

type Graph<'a> = HashMap<&'a str, Valve<'a>>;
type DistanceCache<'a> = HashMap<(&'a str, &'a str), usize>;

struct State<'a> {
    pos: &'a str,
    unopened: HashSet<&'a str>,
    remaining: usize,
    total_pressure: usize,
}

#[derive(Debug)]
struct Result {
    opened_mask: usize,
    total_pressure: usize,
}

#[test]
pub fn test() {
    let input = vec![
        "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB",
        "Valve BB has flow rate=13; tunnels lead to valves CC, AA",
        "Valve CC has flow rate=2; tunnels lead to valves DD, BB",
        "Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE",
        "Valve EE has flow rate=3; tunnels lead to valves FF, DD",
        "Valve FF has flow rate=0; tunnels lead to valves EE, GG",
        "Valve GG has flow rate=0; tunnels lead to valves FF, HH",
        "Valve HH has flow rate=22; tunnel leads to valve GG",
        "Valve II has flow rate=0; tunnels lead to valves AA, JJ",
        "Valve JJ has flow rate=21; tunnel leads to valve II",
    ];

    assert_eq!(a(&input), "1651");
    assert_eq!(b(&input), "1707");
}
