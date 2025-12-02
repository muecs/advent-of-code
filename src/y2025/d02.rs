//! Day 2: Gift Shop

/// Sum of numbers with repeated halves within ranges
pub fn a(input: &Vec<&str>) -> String {
    input[0].split(',').map(|s| s.split_once('-').unwrap()).map(|(a, b)| {
        if a.len() % 2 != 0 && b.len() == a.len() {
            // don't have even length IDs in range
            return 0usize;
        }
        let min = a.parse::<usize>().unwrap();
        let max = b.parse::<usize>().unwrap();
        let mut sum = 0usize;
        let mut n = a[0..a.len() / 2].parse::<usize>().unwrap_or_default();
        loop {
            let nn = format!("{n}{n}").parse::<usize>().unwrap();
            if nn > max {
                break;
            }
            if nn >= min {
                sum += nn;
            }
            n += 1;
        }
        sum
    }).sum::<usize>().to_string()
}

/// Sum of numbers with repeated sequences within ranges
pub fn b(input: &Vec<&str>) -> String {
    input[0].split(',').map(|s| s.split_once('-').unwrap()).map(|(a, b)| {
        let min = a.parse::<usize>().unwrap();
        let max = b.parse::<usize>().unwrap();
        let mut sum = 0usize;
        for i in 1..=b[0..b.len() - b.len() / 2].parse::<usize>().unwrap() {
            let s = i.to_string();
            if (1..=s.len() / 2).any(|l| s.len() % l == 0 && s.as_bytes().chunks_exact(l).skip(1).all(|c| c == &s.as_bytes()[0..l])) {
                // skip internal repetitions; already covered
                continue;
            }
            for l in a.len()..=b.len() {
                if l % s.len() != 0 {
                    continue;
                }
                let reps = l / s.len();
                if reps < 2 || reps * s.len() < a.len() {
                    continue;
                }
                let n = s.repeat(reps).parse::<usize>().unwrap();
                if n >= min && n <= max {
                    // println!("{min}-{max}: {i} -> {n}");
                    sum += n;
                }
            }
        }
        sum
    }).sum::<usize>().to_string()
}

#[test]
pub fn test() {
    let input = vec!["11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"];

    assert_eq!(a(&input), "1227775554");
    assert_eq!(b(&input), "4174379265");
}
