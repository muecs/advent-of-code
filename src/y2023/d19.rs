//! Day 19: Aplenty

use std::collections::HashMap;

/// sum of accepted part ratings
pub fn a(input: &Vec<&str>) -> String {
    let (workflows, parts) = parse_input(input);
    parts
        .iter()
        .filter_map(|part| classify_part(part, &workflows).then(|| part.iter().sum::<u16>() as u32))
        .sum::<u32>()
        .to_string()
}

/// number of accepted part ratings combinations
pub fn b(input: &Vec<&str>) -> String {
    const RANGE: Range = (1, 4000);
    let (workflows, _) = parse_input(input);
    combinations("in", &[RANGE, RANGE, RANGE, RANGE], &workflows).to_string()
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> (Workflows<'a>, Vec<Part>) {
    let mut it = input.iter();
    (
        it.by_ref()
            .map_while(|s| {
                (!s.is_empty()).then(|| {
                    s.split_once('{')
                        .map(|(name, s)| {
                            (
                                name,
                                s.split(',')
                                    .map(|rule| {
                                        if rule.ends_with('}') {
                                            Rule::Fallback(&rule[0..rule.len() - 1])
                                        } else {
                                            let (condition, destination) =
                                                rule.split_once(':').unwrap();
                                            Rule::Condition {
                                                category: match &condition[0..1] {
                                                    "x" => 0,
                                                    "m" => 1,
                                                    "a" => 2,
                                                    "s" => 3,
                                                    _ => unreachable!(),
                                                },
                                                less: match &condition[1..2] {
                                                    "<" => true,
                                                    ">" => false,
                                                    _ => unreachable!(),
                                                },
                                                value: condition[2..].parse().unwrap(),
                                                destination,
                                            }
                                        }
                                    })
                                    .collect(),
                            )
                        })
                        .unwrap()
                })
            })
            .collect(),
        it.map(|s| {
            Part::try_from(
                s[1..s.len() - 1]
                    .split(',')
                    .map(|var| var[2..].parse().unwrap())
                    .collect::<Vec<_>>(),
            )
            .unwrap()
        })
        .collect(),
    )
}

/// whether to accept or reject the part
fn classify_part(part: &Part, workflows: &Workflows) -> bool {
    const CMP: [for<'a, 'b> fn(&'a u16, &'b u16) -> bool; 2] = [u16::gt, u16::lt];
    let mut curr = "in";
    while let Some(rules) = workflows.get(curr) {
        let next = rules
            .iter()
            .find_map(|rule| match rule {
                Rule::Condition {
                    category,
                    less,
                    value,
                    destination,
                } => CMP[*less as usize](&part[*category as usize], value).then_some(*destination),
                Rule::Fallback(destination) => Some(*destination),
            })
            .unwrap();
        match next {
            "A" => return true,
            "R" => return false,
            _ => curr = next,
        }
    }
    unreachable!()
}

fn combinations(name: &str, ranges: &[Range; 4], workflows: &Workflows) -> u64 {
    if name == "A" {
        return ranges
            .iter()
            .fold(1, |acc, (min, max)| acc * (max + 1 - min) as u64);
    } else if name == "R" {
        return 0;
    }

    let rules = &workflows[name];
    let mut ranges = ranges.clone();
    rules
        .iter()
        .map(|rule| match rule {
            Rule::Condition {
                category,
                less,
                value,
                destination,
            } => {
                let mut new_ranges = ranges.clone();
                if *less {
                    new_ranges[*category as usize].1 = value - 1; // set max
                    ranges[*category as usize].0 = *value; // set min
                } else {
                    new_ranges[*category as usize].0 = value + 1; // set min
                    ranges[*category as usize].1 = *value; // set max
                }
                combinations(destination, &new_ranges, workflows)
            }
            Rule::Fallback(destination) => combinations(destination, &ranges, workflows),
        })
        .sum()
}

enum Rule<'a> {
    Condition {
        category: u8,
        less: bool,
        value: u16,
        destination: &'a str,
    },
    Fallback(&'a str),
}
type Workflows<'a> = HashMap<&'a str, Vec<Rule<'a>>>;
type Part = [u16; 4];
type Range = (u16, u16);

#[test]
pub fn test() {
    let input = vec![
        "px{a<2006:qkq,m>2090:A,rfg}",
        "pv{a>1716:R,A}",
        "lnx{m>1548:A,A}",
        "rfg{s<537:gd,x>2440:R,A}",
        "qs{s>3448:A,lnx}",
        "qkq{x<1416:A,crn}",
        "crn{x>2662:A,R}",
        "in{s<1351:px,qqz}",
        "qqz{s>2770:qs,m<1801:hdj,R}",
        "gd{a>3333:R,R}",
        "hdj{m>838:A,pv}",
        "",
        "{x=787,m=2655,a=1222,s=2876}",
        "{x=1679,m=44,a=2067,s=496}",
        "{x=2036,m=264,a=79,s=2244}",
        "{x=2461,m=1339,a=466,s=291}",
        "{x=2127,m=1623,a=2188,s=1013}",
    ];

    assert_eq!(a(&input), "19114");
    assert_eq!(b(&input), "167409079868000");
}
