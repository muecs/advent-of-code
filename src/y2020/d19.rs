//! Day 19: Monster Messages

use std::{collections::HashMap, vec};

/// number of messages matching first rule
pub fn a(input: &Vec<&str>) -> String {
    let (rules, messages) = parse_input(input);
    messages.iter().filter(|msg| validate(msg, 0, &rules)).count().to_string()
}

/// number of messages matching recursive rules
/// * first 3 levels of rules are the same, and imply 42{2,} 31+
/// * message lengths in example are multiples of 5, in full input it's 8
pub fn b(input: &Vec<&str>) -> String {
    let (mut rules, messages) = parse_input(input);
    replace_rules(&mut rules);
    let sub_msg_length = rule_length(42, &rules);
    messages.iter().filter(|msg| {
        let (mut offset, mut n42, mut n31) = (0, 0, 0);
        while offset < msg.len() && validate(&msg[offset..offset + sub_msg_length], 42, &rules)  {
            n42 += 1;
            offset += sub_msg_length;
        }
        while offset < msg.len() && validate(&msg[offset..offset + sub_msg_length], 31, &rules)  {
            n31 += 1;
            offset += sub_msg_length;
        }
        n42 >= 2 && n31 >= 1 && n42 > n31 && offset == msg.len()
    }).count().to_string()
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> (Rules, Vec<&'a str>) {
    let mut it = input.iter();
    let mut rules = Rules::new();

    while let Some(&line) = it.next() {
        if line.is_empty() {
            break;
        }

        // 123: 39 86 | 127 32
        let (num, rule) = line
            .split_once(": ")
            .map(|(num, rule)| {
                (
                    num.parse::<u8>().unwrap(),
                    if rule.starts_with('"') && rule.ends_with('"') {
                        Rule::Literal(rule.trim_matches('"').chars().next().unwrap())
                    } else {
                        Rule::Sub(
                            rule.split(" | ")
                                .map(|subrule| {
                                    subrule
                                        .split(' ')
                                        .map(|n| n.parse::<u8>().unwrap())
                                        .collect::<Vec<u8>>()
                                })
                                .collect::<Vec<Vec<u8>>>(),
                        )
                    },
                )
            })
            .unwrap();
        rules.insert(num, rule);
    }

    let messages = it.cloned().collect::<Vec<&str>>();

    (rules, messages)
}

fn validate(msg: &str, rule: u8, rules: &Rules) -> bool {
    fn validate_rule(depth: usize, submsg: &str, rule: u8, rules: &Rules) -> Option<usize> {
        // println!("{}rule {} - {:?}", ".".repeat(depth), rule, submsg);
        if submsg.is_empty() {
            return None;
        }
        match rules.get(&rule) {
            Some(Rule::Literal(c)) => submsg.starts_with(*c).then_some(1),
            Some(Rule::Sub(alternatives)) => {
                // println!("{}> alt: {:?}", ".".repeat(depth), alternatives);
                alternatives.iter().find_map(|subrules| {
                    let mut offset = 0;
                    for r in subrules {
                        // println!("{}> trying subrule {}", ".".repeat(depth), r);
                        if let Some(inc) = validate_rule(depth + 1, &submsg[offset..], *r, rules) {
                            // println!("{}> subrule {} consumed {}", ".".repeat(depth), r, inc);
                            offset += inc;
                        } else {
                            // println!("{}> subrule {} does not match", ".".repeat(depth), r);
                            return None;
                        }
                    }
                    Some(offset)
                })
            },
            None => unreachable!("invalid rule number"),
        }
    }

    let result = validate_rule(0, msg, rule, rules);
    // println!("msg {:?}, len {} - {:?}", msg, msg.len(), result);
    result == Some(msg.len())
}

fn replace_rules(rules: &mut Rules) {
    rules.insert(8, Rule::Sub(vec![vec![42], vec![42, 8]]));
    rules.insert(11, Rule::Sub(vec![vec![42, 31], vec![42, 11, 31]]));
}

fn rule_length(rule: u8, rules: &Rules) -> usize {
    let mut rule_stack = vec![rule];
    let mut length = 0;
    while let Some(r) = rule_stack.pop() {
        match rules.get(&r) {
            Some(Rule::Literal(_)) => length += 1,
            Some(Rule::Sub(alternatives)) => {
                rule_stack.append(&mut alternatives.first().unwrap().clone());
            }
            None => unreachable!("invalid rule number"),
        }
    }
    length
}

enum Rule {
    Literal(char),
    Sub(Vec<Vec<u8>>),
}
type Rules = HashMap<u8, Rule>;

#[test]
pub fn test() {
    let input = vec![
        "0: 1 2",
        "1: \"a\"",
        "2: 1 3 | 3 1",
        "3: \"b\"",
        "",
        "aab",
        "aba",
        "ab",
        "abb",
        "abba",
    ];

    let input2 = vec![
        "0: 4 1 5",
        "1: 2 3 | 3 2",
        "2: 4 4 | 5 5",
        "3: 4 5 | 5 4",
        "4: \"a\"",
        "5: \"b\"",
        "",
        "ababbb",
        "bababa",
        "abbbab",
        "aaabbb",
        "aaaabbb",
    ];

    let input3 = vec![
        "42: 9 14 | 10 1",
        "9: 14 27 | 1 26",
        "10: 23 14 | 28 1",
        "1: \"a\"",
        "11: 42 31",
        "5: 1 14 | 15 1",
        "19: 14 1 | 14 14",
        "12: 24 14 | 19 1",
        "16: 15 1 | 14 14",
        "31: 14 17 | 1 13",
        "6: 14 14 | 1 14",
        "2: 1 24 | 14 4",
        "0: 8 11",
        "13: 14 3 | 1 12",
        "15: 1 | 14",
        "17: 14 2 | 1 7",
        "23: 25 1 | 22 14",
        "28: 16 1",
        "4: 1 1",
        "20: 14 14 | 1 15",
        "3: 5 14 | 16 1",
        "27: 1 6 | 14 18",
        "14: \"b\"",
        "21: 14 1 | 1 14",
        "25: 1 1 | 1 14",
        "22: 14 14",
        "8: 42",
        "26: 14 22 | 1 20",
        "18: 15 15",
        "7: 14 5 | 1 21",
        "24: 14 1",
        "",
        "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
        "bbabbbbaabaabba",
        "babbbbaabbbbbabbbbbbaabaaabaaa",
        "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
        "bbbbbbbaaaabbbbaaabbabaaa",
        "bbbababbbbaaaaaaaabbababaaababaabab",
        "ababaaaaaabaaab",
        "ababaaaaabbbaba",
        "baabbaaaabbaaaababbaababb",
        "abbbbabbbbaaaababbbbbbaaaababb",
        "aaaaabbaabaaaaababaa",
        "aaaabbaaaabbaaa",
        "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
        "babaaabbbaaabaababbaabababaaab",
        "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
    ];

    assert_eq!(a(&input), "2");
    assert_eq!(a(&input2), "2");
    assert_eq!(a(&input3), "3");

    let (mut rules, messages) = parse_input(&input3);
    replace_rules(&mut rules);
    assert_eq!(rules.len(), 31);
    assert_eq!(messages.len(), 15);
    assert_eq!(rule_length(42, &rules), 5);
    assert_eq!(rule_length(31, &rules), 5);
    // assert_eq!(validate(messages[0], &rules), false);
    // assert_eq!(validate(messages[1], &rules), true);
    // assert_eq!(validate(messages[2], &rules), true);
    // assert_eq!(validate(messages[3], &rules), true);
    // assert_eq!(validate(messages[4], &rules), true);

    assert_eq!(b(&input3), "12");
}
