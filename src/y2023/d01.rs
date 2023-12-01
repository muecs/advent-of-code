//! Day 1: Trebuchet?!

/// part a
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|s| {
            // print!("{}", s);
            let mut s2 = s
                .chars()
                .filter_map(|c| (c >= '0' && c <= '9').then_some(c))
                .collect::<String>();
            s2 = s2[0..1].to_string() + &s2[s2.len() - 1..].to_string();
            // println!(" -> {}", s2);
            s2.parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

/// part b
pub fn b(input: &Vec<&str>) -> String {
    let processed = input
        .iter()
        .map(|s| {
            // allow for overlapping digit words
            s.replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "4")
                .replace("five", "5e")
                .replace("six", "6")
                .replace("seven", "7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .collect::<Vec<_>>();
    a(&processed.iter().map(|s| &**s).collect())
}

#[test]
pub fn test() {
    let input1 = vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
    let input2 = vec![
        "two1nine",
        "eightwothree",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
    ];

    assert_eq!(a(&input1), "142");
    assert_eq!(b(&input2), "281");
}
