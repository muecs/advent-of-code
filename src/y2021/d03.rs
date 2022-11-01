//! Day 3: Binary Diagnostic

/// calculate power consumption by multiplying gamma and epsilon
pub fn a(input: &Vec<&str>) -> String {
    let gamma_str = get_gamma(input);
    let gamma = u32::from_str_radix(&gamma_str, 2).unwrap();

    let epsilon_str = flip_bit_str(&gamma_str);
    let epsilon = u32::from_str_radix(&epsilon_str, 2).unwrap();

    (gamma * epsilon).to_string()
}

/// calculate life support rating
pub fn b(input: &Vec<&str>) -> String {
    let oxygen_generator_rating_str = get_rating(input, false);
    let oxygen_generator_rating = 
        u32::from_str_radix(&oxygen_generator_rating_str, 2).unwrap();

    let co2_scrubber_rating_str = get_rating(input, true);
    let co2_scrubber_rating = 
        u32::from_str_radix(&co2_scrubber_rating_str, 2).unwrap();
    
    (oxygen_generator_rating * co2_scrubber_rating).to_string()
}

/// determines most common bit in the given position
fn most_common(input: &Vec<&str>, index: usize) -> char {
    let avg = input
        .iter()
        .map(|s| if s.chars().nth(index).unwrap() == '0' { -1i8 } else { 1i8 })
        .sum::<i8>();
    if avg >= 0 { '1' } else { '0' }
}

fn get_gamma(input: &Vec<&str>) -> String {
    // input
    //     .iter()
    //     .map(|s| s.chars().map(|c| if c == '0' { -1i8 } else { 1i8 }).collect::<Vec<i8>>())
    //     .reduce(|accum, item| item.iter().enumerate().map(|(i, val)| accum[i] + val).collect())
    //     .unwrap()
    //     .iter()
    //     .map(|&n| if n > 0 { '1' } else { '0' })
    //     .collect()
    input[0]
        .chars()
        .enumerate()
        .map(|(i, _)| most_common(input, i))
        .collect()
}

fn get_rating(input: &Vec<&str>, least_common: bool) -> String {
    let n = input[0].len();
    let mut selection = input.to_owned();
    let mut i = 0;
    while i < n && selection.len() > 1 {
        let mut bit = most_common(&selection, i);
        if least_common {
            bit = flip_bit(bit);
        }
        selection = selection
            .iter()
            .filter(|s| s.chars().nth(i).unwrap() == bit)
            .cloned()
            .collect();
        i += 1;
    }

    selection[0].to_owned()
}

fn flip_bit(c: char) -> char {
    if c == '0' { '1' } else { '0' }
}

fn flip_bit_str(s: &str) -> String {
    s.chars().map(&flip_bit).collect()
}

#[test]
pub fn test() {
    let input = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010",
    ];

    assert_eq!(flip_bit('0'), '1');
    assert_eq!(flip_bit('1'), '0');
    assert_eq!(flip_bit_str(input[0]), "11011");

    assert_eq!(most_common(&input, 0), '1');
    assert_eq!(most_common(&input, 1), '0');
    assert_eq!(get_gamma(&input), "10110");
    assert_eq!(get_rating(&input, false), "10111");
    assert_eq!(get_rating(&input, true), "01010");

    assert_eq!(a(&input), "198");
    assert_eq!(b(&input), "230");
}
