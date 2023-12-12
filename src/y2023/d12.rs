//! Day 12: Hot Springs

use std::collections::HashMap;

/// sum of operational/broken spring arrangement permutations
pub fn a(input: &Vec<&str>) -> String {
    let records = parse_input(input);
    records
        .iter()
        .map(|(conditions, sizes)| find_combinations(conditions, sizes))
        .sum::<usize>()
        .to_string()
}

/// sum of permutations with repetitions
pub fn b(input: &Vec<&str>) -> String {
    let records = parse_input(input);
    records
        .iter()
        .map(|record| {
            let (conditions, sizes) = unfold(record);
            find_combinations(&conditions, &sizes)
        })
        // .inspect(|n| println!("{}", n))
        .sum::<usize>()
        .to_string()
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> Records<'a> {
    input
        .iter()
        .map(|line| {
            line.split_once(' ')
                .map(|(conditions, sizes)| {
                    (
                        conditions,
                        sizes.split(',').map(|n| n.parse::<u8>().unwrap()).collect(),
                    )
                })
                .unwrap()
        })
        .collect()
}

fn unfold<'a>(record: &'a Record<'a>) -> (String, Vec<u8>) {
    (
        record.0.to_owned() + &(String::from("?") + &record.0).repeat(4),
        record.1.repeat(5),
    )
}

fn find_combinations(conditions: &str, sizes: &[u8]) -> usize {
    fn combinations(conditions: &str, sizes: &[u8], matching: u8, cache: &mut Cache) -> usize {
        if let Some(val) = cache.get(&(conditions.len(), sizes.len(), matching)) {
            return *val; // cache hit
        }

        let result = match conditions.bytes().next() {
            Some(b'#') => combinations(&conditions[1..], sizes, matching + 1, cache),
            Some(b'.') => {
                if !sizes.is_empty() && sizes[0] == matching {
                    combinations(&conditions[1..], &sizes[1..], 0, cache)
                } else if matching == 0 {
                    combinations(&conditions[1..], sizes, 0, cache)
                } else {
                    0
                }
            }
            Some(b'?') => {
                // consider both damaged ('#') and operational ('.') options
                let damaged = combinations(&conditions[1..], sizes, matching + 1, cache);
                let operational = if !sizes.is_empty() && sizes[0] == matching {
                    combinations(&conditions[1..], &sizes[1..], 0, cache)
                } else if matching == 0 {
                    combinations(&conditions[1..], sizes, 0, cache)
                } else {
                    0
                };
                damaged + operational
            }
            Some(_) => unreachable!("invalid character"),
            None => {
                if sizes.is_empty() && matching == 0 {
                    1
                } else if sizes.len() == 1 && sizes[0] == matching {
                    1
                } else {
                    0
                }
            }
        };

        cache.insert((conditions.len(), sizes.len(), matching), result);

        result
    }

    let mut cache = Cache::new();
    combinations(conditions, sizes, 0, &mut cache)
}

type Cache = HashMap<(usize, usize, u8), usize>;
type Record<'a> = (&'a str, Vec<u8>);
type Records<'a> = Vec<Record<'a>>;

#[test]
pub fn test() {
    let input = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];

    {
        let records = parse_input(&input);
        assert_eq!(records.len(), 6);
        assert_eq!(find_combinations(&records[0].0, &records[0].1), 1);
        assert_eq!(find_combinations(&records[1].0, &records[1].1), 4);
        assert_eq!(find_combinations(&records[2].0, &records[2].1), 1);
        assert_eq!(find_combinations(&records[3].0, &records[3].1), 1);
        assert_eq!(find_combinations(&records[4].0, &records[4].1), 4);
        assert_eq!(find_combinations(&records[5].0, &records[5].1), 10);
    }

    assert_eq!(a(&input), "21");

    {
        let records = parse_input(&input)
            .iter()
            .map(|record| unfold(record))
            .collect::<Vec<_>>();
        assert_eq!(records.len(), 6);
        assert_eq!(find_combinations(&records[0].0, &records[0].1), 1);
        assert_eq!(find_combinations(&records[1].0, &records[1].1), 16384);
        assert_eq!(find_combinations(&records[2].0, &records[2].1), 1);
        assert_eq!(find_combinations(&records[3].0, &records[3].1), 16);
        assert_eq!(find_combinations(&records[4].0, &records[4].1), 2500);
        assert_eq!(find_combinations(&records[5].0, &records[5].1), 506250);
    }

    assert_eq!(b(&input), "525152");
}
