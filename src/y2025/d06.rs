//! Day 6: Trash Compactor

/// sum of columnar equation results
pub fn a(input: &Vec<&str>) -> String {
    solve(input, |col| {
        col.iter()
            .map(|v| str::from_utf8(v).unwrap().trim().parse::<usize>().unwrap())
            .collect()
    })
    .to_string()
}

/// sum of RTL columnar equation results
pub fn b(input: &Vec<&str>) -> String {
    solve(input, |col| {
        (0..col[0].len())
            .map(|i| {
                col.iter()
                    .map(|v| v[i] as char)
                    .collect::<String>()
                    .trim()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect()
    })
    .to_string()
}

pub fn solve(input: &Vec<&str>, parse_nums: impl Fn(&Vec<&[u8]>) -> Vec<usize>) -> usize {
    let lines = input.iter().map(|s| s.as_bytes()).collect::<Vec<_>>();
    // collect offsets and operators
    let equations = lines
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i, &b)| (!b.is_ascii_whitespace()).then_some((i, b)))
        .collect::<Vec<_>>();
    // parse numbers from columns and solve the equation
    (0..equations.len())
        .map(|i| {
            let end = equations
                .get(i + 1)
                .map_or(lines[0].len(), |(idx, _)| *idx - 1);
            let col = (0..lines.len() - 1)
                .map(|j| &lines[j][equations[i].0..end])
                .collect();
            let nums = parse_nums(&col);
            // println!(
            //     "col: {:?}, nums: {nums:?}, op: {}",
            //     col.iter()
            //         .map(|v| str::from_utf8(v).unwrap())
            //         .collect::<Vec<_>>(),
            //     equations[i].1 as char
            // );
            match equations[i].1 {
                b'+' => nums.iter().sum::<usize>(),
                b'*' => nums.iter().product::<usize>(),
                _ => unreachable!(),
            }
        })
        .sum::<usize>()
}

#[test]
pub fn test() {
    let input = vec![
        "123 328  51 64 ",
        " 45 64  387 23 ",
        "  6 98  215 314",
        "*   +   *   +  ",
    ];

    assert_eq!(a(&input), "4277556");
    assert_eq!(b(&input), "3263827");
}
