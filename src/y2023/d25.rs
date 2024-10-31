//! Day 25: Snowverload

use std::collections::HashMap;

/// product of size of graph partitions after removing 3 edges
pub fn a(input: &Vec<&str>) -> String {
    let mut adj = parse_input(input);
    // println!("nodes: {}", adj.len());

    // Stoer–Wagner minimum cut algorithm
    let n = adj.len();
    let mut best_cut = (i16::MAX, Vec::<usize>::new());
    let mut subgraphs = (0..n).map(|i| vec![i]).collect::<Vec<_>>();
    for phase in 1..n {
        // TODO speed up with priority queue
        let mut weights = adj[0].to_owned();
        let mut prev = 0;
        let mut curr = 0;
        for _ in 0..n - phase {
            weights[curr] = i16::MIN;
            prev = curr;
            curr = weights
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(idx, _)| idx)
                .unwrap();
            for i in 0..n {
                weights[i] += adj[curr][i];
            }
        }

        let mut curr_subgraph = subgraphs[curr].to_owned();
        best_cut = best_cut.min((weights[curr] - adj[curr][curr], curr_subgraph.to_owned()));

        // merge prev and curr nodes
        subgraphs[prev].append(&mut curr_subgraph);
        for i in 0..n {
            adj[prev][i] += adj[curr][i];
            adj[i][prev] = adj[prev][i];
        }
        adj[0][curr] = i16::MIN;
    }

    let subgraph_size = best_cut.1.len();
    (subgraph_size * (adj.len() - subgraph_size)).to_string()
}

fn parse_input(input: &Vec<&str>) -> Matrix {
    let mut node_mapping = HashMap::<&str, usize>::with_capacity(input.len());
    let mut get_index = |s| {
        if let Some(&idx) = node_mapping.get(s) {
            idx
        } else {
            let idx = node_mapping.len();
            node_mapping.insert(s, idx);
            idx
        }
    };

    // build list of connected components
    let mut connections = Vec::new();
    for &s in input {
        let node_idx = get_index(&s[0..3]);
        for adj in s[5..].split(' ') {
            let adj_idx = get_index(adj);
            connections.push((node_idx, adj_idx));
        }
    }

    // turn list into adjacency matrix
    let mut adj = vec![vec![0; node_mapping.len()]; node_mapping.len()];
    for (a, b) in connections {
        adj[a][b] = 1;
        adj[b][a] = 1;
    }

    adj
}

type Matrix = Vec<Vec<i16>>;

#[test]
pub fn test() {
    let input = vec![
        "jqt: rhn xhk nvd",
        "rsh: frs pzl lsr",
        "xhk: hfx",
        "cmg: qnr nvd lhk bvb",
        "rhn: xhk bvb hfx",
        "bvb: xhk hfx",
        "pzl: lsr hfx nvd",
        "qnr: nvd",
        "ntq: jqt hfx bvb xhk",
        "nvd: lhk",
        "lsr: lhk",
        "rzs: qnr cmg lsr rsh",
        "frs: qnr lhk lsr",
    ];

    assert_eq!(a(&input), "54");
}
