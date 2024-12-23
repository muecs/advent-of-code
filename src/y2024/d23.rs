//! Day 23: LAN Party

use std::collections::{HashMap, HashSet};

/// interconnected sets of 3 nodes involving one starting with `t`
pub fn a(input: &Vec<&str>) -> String {
    let graph = parse_input(input);
    let mut count = 0;
    for (node, adj) in &graph {
        for a in adj {
            if a < node {
                continue;
            }
            for b in &graph[*a] {
                if b > a
                    && (node.starts_with('t') || a.starts_with('t') || b.starts_with('t'))
                    && graph[*b].contains(node)
                {
                    // found group of 3
                    count += 1;
                }
            }
        }
    }
    count.to_string()
}

/// sorted nodes in largest fully connected subgraph
pub fn b(input: &Vec<&str>) -> String {
    let graph = parse_input(input);
    let mut subgraph = Nodes::new();
    max_cliques(
        &graph,
        &Nodes::new(),
        graph.keys().copied().collect(),
        Nodes::new(),
        &mut subgraph,
    );
    let mut subgraph = Vec::from_iter(subgraph.into_iter());
    subgraph.sort_unstable();
    subgraph.join(",")
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> Graph<'a> {
    // build undirected graph
    let mut graph = Graph::new();
    for s in input {
        let (a, b) = s.split_once('-').unwrap();
        graph
            .entry(a)
            .and_modify(|adj| {
                adj.insert(b);
            })
            .or_insert(HashSet::from([b]));
        graph
            .entry(b)
            .and_modify(|adj| {
                adj.insert(a);
            })
            .or_insert(HashSet::from([a]));
    }
    graph
}

/// Bron-Kerbosch algorithm
/// https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn max_cliques<'a>(
    graph: &Graph<'a>,
    r: &Nodes<'a>,
    mut p: Nodes<'a>,
    mut x: Nodes<'a>,
    max: &mut Nodes<'a>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max.len() {
            *max = r.to_owned();
        }
        return;
    }
    for v in p.to_owned() {
        let adj = &graph[v];
        max_cliques(
            graph,
            &r.union(&HashSet::from([v])).copied().collect(),
            p.intersection(&adj).copied().collect(),
            x.intersection(&adj).copied().collect(),
            max,
        );
        p.remove(v);
        x.insert(v);
    }
}

type Nodes<'a> = HashSet<&'a str>;
type Graph<'a> = HashMap<&'a str, Nodes<'a>>;

#[test]
pub fn test() {
    let input = vec![
        "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka", "wh-tc",
        "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka", "td-qp", "aq-cg",
        "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de", "kh-ta", "co-tc", "wh-qp",
        "tb-vc", "td-yn",
    ];

    assert_eq!(a(&input), "7");
    assert_eq!(b(&input), "co,de,ka,ta");
}
