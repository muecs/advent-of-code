//! Day 7: Handy Haversacks

use std::collections::HashMap;

/// number of potential container bags for 'shiny gold' bag
pub fn a(input: &Vec<&str>) -> String {
    let rules = parse_input(input);

    let mut contained_by = HashMap::new();
    for (container, contents) in &rules {
        for (content, _) in contents {
            contained_by
                .entry(*content)
                .or_insert(Vec::new())
                .push(*container);
        }
    }

    fn traverse_containers<'a>(
        bag: &str,
        contained_by: &HashMap<&str, Vec<&'a str>>,
        containers: &mut Vec<&'a str>,
    ) {
        if let Some(c) = contained_by.get(bag) {
            for container in c {
                containers.push(*container);
                traverse_containers(*container, contained_by, containers);
            }
        }
    }
    let mut containers = Vec::new();
    traverse_containers("shiny gold", &contained_by, &mut containers);
    containers.sort_unstable();
    containers.dedup();

    containers.len().to_string()
}

/// number of bags contained by 'shiny gold' bag
pub fn b(input: &Vec<&str>) -> String {
    let rules = parse_input(input);

    fn traverse_contents(bag: &str, contents: &HashMap<&str, Vec<(&str, usize)>>) -> usize {
        let mut count = 0;
        if let Some(c) = contents.get(bag) {
            for (content, n) in c {
                count += *n * (1 + traverse_contents(*content, contents));
            }
        }
        count
    }

    let count = traverse_contents("shiny gold", &rules);
    count.to_string()
}

fn parse_input<'a>(input: &'a Vec<&str>) -> HashMap<&'a str, Vec<(&'a str, usize)>> {
    input
        .iter()
        .map(|line| {
            let (container, contents) = line.split_once(" bags contain ").unwrap();
            let contents = if contents == "no other bags." {
                Vec::new()
            } else {
                contents
                    .split(", ")
                    .map(|s| {
                        let (n, s) = s.split_once(" ").unwrap();
                        let (bag, _) = s.split_once(" bag").unwrap();
                        (bag, n.parse::<usize>().unwrap())
                    })
                    .collect()
            };
            (container, contents)
        })
        .collect()
}

#[test]
pub fn test() {
    let input = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.",
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
        "bright white bags contain 1 shiny gold bag.",
        "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
        "faded blue bags contain no other bags.",
        "dotted black bags contain no other bags.",
    ];

    assert_eq!(a(&input), "4");
    assert_eq!(b(&input), "32");
}
