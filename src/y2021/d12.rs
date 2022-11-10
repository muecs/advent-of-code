//! Day 12: Passage Pathing

use std::collections::BTreeMap;

type Graph<V> = BTreeMap<V, Vec<V>>;

/// number of paths through graph visiting lowercase nodes at most once
pub fn a(input: &Vec<&str>) -> String {
    let graph = parse_input(input);
    count_paths(&graph, false).to_string()
}

/// number of paths allowing a single duplicate lowercase node
pub fn b(input: &Vec<&str>) -> String {
    let graph = parse_input(input);
    count_paths(&graph, true).to_string()
}

fn parse_input<'a>(input: &'a Vec<&str>) -> Graph<&'a str> {
    let mut graph = Graph::new();

    for &line in input {
        let mut it = line.split('-');
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        if let Some(edges) = graph.get_mut(a) {
            edges.push(b);
        } else {
            graph.insert(a, vec![b]);
        }

        if let Some(edges) = graph.get_mut(b) {
            edges.push(a);
        } else {
            graph.insert(b, vec![a]);
        }
    }

    graph
}

fn count_paths(graph: &Graph<&str>, allow_single_duplicate: bool) -> usize {
    fn valid_node(
        node: &str,
        path: &Vec<&str>,
        allow_single_duplicate: bool,
    ) -> bool {
        if path.is_empty() || !node.chars().next().unwrap().is_lowercase() || !path.contains(&node) {
            true
        } else if allow_single_duplicate && node != "start" {
            // check if there already are any duplicates
            let mut path = path
                .iter()
                .filter(|&&s| s.chars().next().unwrap().is_lowercase())
                .collect::<Vec<_>>();
            path.sort_unstable();
            let len = path.len();
            path.dedup();

            path.len() == len
        } else {
            false
        }
    }

    fn traverse<'a>(
        graph: &Graph<&'a str>,
        nodes: &Vec<&'a str>,
        path: &mut Vec<&'a str>,
        paths: &mut usize,
        allow_single_duplicate: bool,
    ) {
        for &node in nodes {
            if node == "end" {
                #[cfg(test)]
                println!("{},end", path.join(","));

                *paths += 1;
                continue;
            }

            if !valid_node(&node, path, allow_single_duplicate) {
                continue;
            }

            path.push(node);
            traverse(graph, &graph[node], path, paths, allow_single_duplicate);
            path.pop();
        }
    }

    #[cfg(test)]
    println!("{:?}", graph);

    let mut paths = 0;
    let mut path = Vec::new();
    traverse(graph, &vec!["start"], &mut path, &mut paths, allow_single_duplicate);

    paths
}

#[test]
pub fn test() {

    //     start
    //     /   \
    // c--A-----b--d
    //     \   /
    //      end
    let input = vec![
        "start-A",
        "start-b",
        "A-c",
        "A-b",
        "b-d",
        "A-end",
        "b-end",
    ];

    let input2 = vec![
        "dc-end",
        "HN-start",
        "start-kj",
        "dc-start",
        "dc-HN",
        "LN-dc",
        "HN-end",
        "kj-sa",
        "kj-HN",
        "kj-dc",
    ];

    let input3 = vec![
        "fs-end",
        "he-DX",
        "fs-he",
        "start-DX",
        "pj-DX",
        "end-zg",
        "zg-sl",
        "zg-pj",
        "pj-he",
        "RW-he",
        "fs-DX",
        "pj-RW",
        "zg-RW",
        "start-pj",
        "he-WI",
        "zg-he",
        "pj-fs",
        "start-RW",
    ];

    assert_eq!(a(&input), "10");
    assert_eq!(a(&input2), "19");
    assert_eq!(a(&input3), "226");

    assert_eq!(b(&input), "36");
    assert_eq!(b(&input2), "103");
    assert_eq!(b(&input3), "3509");
}
