//! Day 14: Extended Polymerization

use std::collections::BTreeMap;

/// subtract least common from most common after 10 steps
pub fn a(input: &Vec<&str>) -> String {
    let (template, rules) = parse_input(input);
    let quantities = apply_rules(&template, &rules, 10);

    (quantities.last().unwrap().1 - quantities.first().unwrap().1).to_string()
}

/// subtract least common from most common after 40 steps
pub fn b(input: &Vec<&str>) -> String {
    let (template, rules) = parse_input(input);
    let quantities = apply_rules(&template, &rules, 40);

    (quantities.last().unwrap().1 - quantities.first().unwrap().1).to_string()
}

fn parse_input<'a>(input: &'a Vec<&str>) -> (String, BTreeMap<&'a str, &'a str>) {
    let mut it = input.iter();
    let template = it.next().unwrap().to_string();
    // it.next();
    let rules = it
        .filter_map(|&s| s.split_once(" -> "))
        .collect::<BTreeMap<_, _>>();

    (template, rules)
}

fn apply_rules(
    template: &str,
    rules: &BTreeMap<&str, &str>,
    iterations: usize,
) -> Vec<(char, usize)> {
    /// increment count of given pair by value
    fn inc(pair: &str, value: usize, pairs: &mut BTreeMap<String, usize>) {
        if let Some(entry) = pairs.get_mut(pair) {
            *entry += value;
        } else {
            pairs.insert(pair.to_string(), value);
        }
    }

    // count pairs in template string
    let mut pairs = BTreeMap::new();
    template.chars().collect::<Vec<_>>().windows(2).for_each(|c| {
        let pair = format!("{}{}", c[0], c[1]);
        inc(&pair, 1, &mut pairs);
    });

    for _ in 0..iterations {
        // create new pairs by applying rules
        let mut new_pairs = Vec::new();
        for pair in &pairs {
            let a = pair.0.chars().nth(0).unwrap();
            let b = rules[pair.0.as_str()].chars().nth(0).unwrap();
            let c = pair.0.chars().nth(1).unwrap();
            new_pairs.push((format!("{}{}", a, b), *pair.1));
            new_pairs.push((format!("{}{}", b, c), *pair.1));
        }

        // remove old pairs
        pairs.clear();

        // insert new pairs and their counts
        for pair in &new_pairs {
            inc(&pair.0, pair.1, &mut pairs);
        }
    }

    #[cfg(test)]
    println!("{:?}", pairs);

    // count elements in pairs
    let mut elements: BTreeMap<char, usize> = BTreeMap::new();
    elements.insert(template.chars().nth(0).unwrap(), 1);
    elements.insert(template.chars().last().unwrap(), 1);
    for pair in &pairs {
        for c in pair.0.chars() {
            if let Some(n) = elements.get_mut(&c) {
                *n += *pair.1;
            } else {
                elements.insert(c, *pair.1);
            }
        }
    }

    // transform map to sorted vec, account for duplicates
    let mut vec = elements
        .iter()
        .map(|x| (*x.0, *x.1 / 2))
        .collect::<Vec<_>>();
    vec.sort_unstable_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0)));
    vec
}

#[test]
pub fn test() {
    let input = vec![
        "NNCB",
        "",
        "CH -> B",
        "HH -> N",
        "CB -> H",
        "NH -> C",
        "HB -> C",
        "HC -> B",
        "HN -> C",
        "NN -> C",
        "BH -> H",
        "NC -> B",
        "NB -> B",
        "BN -> B",
        "BB -> N",
        "BC -> B",
        "CC -> N",
        "CN -> C",
    ];

    let (template, rules) = parse_input(&input);
    assert_eq!(template, input[0]);
    assert_eq!(
        apply_rules(&template, &rules, 0),
        vec![('B', 1), ('C', 1), ('N', 2)],
        "NNCB",
    );
    assert_eq!(
        apply_rules(&template, &rules, 1),
        vec![('H', 1), ('B', 2), ('C', 2), ('N', 2)],
        "NCNBCHB",
    );
    assert_eq!(
        apply_rules(&template, &rules, 2),
        vec![('H', 1), ('N', 2), ('C', 4), ('B', 6)],
        "NBCCNBBBCBHCB",
    );
    assert_eq!(
        apply_rules(&template, &rules, 3),
        vec![('H', 4), ('C', 5), ('N', 5), ('B', 11)],
        "NBBBCNCCNBBNBNBBCHBHHBCHB",
    );
    assert_eq!(
        apply_rules(&template, &rules, 10),
        vec![('H', 161), ('C', 298), ('N', 865), ('B', 1749)],
    );

    assert_eq!(a(&input), "1588");
    assert_eq!(b(&input), "2188189693529");
}
