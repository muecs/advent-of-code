//! Day 13: Shuttle Search

/// product of earliest bus ID and waiting time
pub fn a(input: &Vec<&str>) -> String {
    let (start, buses) = parse_input(input);
    let (earliest_bus, wait) = buses
        .iter()
        .filter_map(|opt| opt.map(|id| (id, id - start % id)))
        .reduce(|acc, item| if item.1 < acc.1 { item } else { acc })
        .unwrap();
    (earliest_bus * wait).to_string()
}

/// part b
pub fn b(input: &Vec<&str>) -> String {
    let (_, buses) = parse_input(input);
    find_matching_timestamp(&buses).to_string()
}

fn parse_input(input: &Vec<&str>) -> (usize, Vec<Option<usize>>) {
    (
        input[0].parse().unwrap(),
        input[1].split(',').map(|s| s.parse().ok()).collect(),
    )
}

fn find_matching_timestamp(buses: &Vec<Option<usize>>) -> usize {
    let mut time = 0;
    let mut step = 1;

    for (i, id) in buses
        .iter()
        .enumerate()
        .filter_map(|(i, val)| val.map(|id| (i, id)))
    {
        while (time + i) % id != 0 {
            time += step;
        }

        step *= id; 
    }

    time
}

#[test]
pub fn test() {
    let input = vec!["939", "7,13,x,x,59,x,31,19"];

    assert_eq!(a(&input), "295");

    assert_eq!(
        find_matching_timestamp(&vec![Some(17), None, Some(13), Some(19)]),
        3417
    );
    assert_eq!(
        find_matching_timestamp(&vec![Some(67), Some(7), Some(59), Some(61)]),
        754018
    );
    assert_eq!(
        find_matching_timestamp(&vec![Some(67), None, Some(7), Some(59), Some(61)]),
        779210
    );
    assert_eq!(
        find_matching_timestamp(&vec![Some(67), Some(7), None, Some(59), Some(61)]),
        1261476
    );
    assert_eq!(
        find_matching_timestamp(&vec![Some(1789), Some(37), Some(47), Some(1889)]),
        1202161486
    );

    assert_eq!(b(&input), "1068781");
}
