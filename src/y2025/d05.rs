//! Day 5: Cafeteria

/// number of IDs included in at least one range
pub fn a(input: &Vec<&str>) -> String {
    let (ranges, ids) = parse_input(input);
    ids.iter()
        .filter(|id| ranges.iter().any(|(start, end)| *id >= start && *id <= end))
        .count()
        .to_string()
}

/// size of merged ranges
pub fn b(input: &Vec<&str>) -> String {
    let (mut ranges, _) = parse_input(input);
    ranges.sort_unstable();
    let mut count = 0;
    let mut max = 0;
    for (start, end) in ranges {
        if end <= max {
            continue;
        }
        if start > max {
            count += end - start + 1;
            max = end;
        } else {
            count += end - max;
            max = end;
        }
    }
    count.to_string()
}

fn parse_input(input: &Vec<&str>) -> (Vec<(usize, usize)>, Vec<usize>) {
    let mut it = input.iter();
    let ranges = it
        .by_ref()
        .map_while(|s| {
            (!s.is_empty()).then(|| {
                s.split_once('-')
                    .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
                    .unwrap()
            })
        })
        .collect();

    let ids = it.map(|s| s.parse::<usize>().unwrap()).collect();
    (ranges, ids)
}

#[test]
pub fn test() {
    let input = vec![
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    assert_eq!(a(&input), "3");
    assert_eq!(b(&input), "14");
}
