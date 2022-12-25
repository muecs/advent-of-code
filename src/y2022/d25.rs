//! Day 25: Full of Hot Air

/// Sum of SNAFU numbers
pub fn a(input: &Vec<&str>) -> String {
    let sum = input.iter().map(|line| snafu_to_decimal(line)).sum();
    decimal_to_snafu(sum)
}

fn snafu_to_decimal(snafu: &str) -> isize {
    let mut n = 0isize;
    for b in snafu.bytes() {
        n = n * 5
            + match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => unreachable!("invalid SNAFU digit"),
            }
    }
    n
}

fn decimal_to_snafu(mut decimal: isize) -> String {
    const DIGITS: [u8; 5] = [b'0', b'1', b'2', b'=', b'-'];
    let mut snafu = Vec::new();
    while decimal > 0 {
        let rem = decimal % 5;
        snafu.push(DIGITS[rem as usize]);
        decimal = (decimal / 5) + (rem >= 3) as isize;
    }
    String::from_utf8(snafu.iter().rev().cloned().collect()).unwrap()
}

#[test]
pub fn test() {
    let input = vec![
        "1=-0-2", "12111", "2=0=", "21", "2=01", "111", "20012", "112", "1=-1=", "1-12", "12",
        "1=", "122",
    ];

    assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
    assert_eq!(snafu_to_decimal("12111"), 906);
    assert_eq!(snafu_to_decimal("2=0="), 198);
    assert_eq!(snafu_to_decimal("21"), 11);
    assert_eq!(snafu_to_decimal("2=01"), 201);
    assert_eq!(snafu_to_decimal("111"), 31);
    assert_eq!(snafu_to_decimal("20012"), 1257);
    assert_eq!(snafu_to_decimal("112"), 32);
    assert_eq!(snafu_to_decimal("1=-1="), 353);
    assert_eq!(snafu_to_decimal("1-12"), 107);
    assert_eq!(snafu_to_decimal("12"), 7);
    assert_eq!(snafu_to_decimal("1="), 3);
    assert_eq!(snafu_to_decimal("122"), 37);

    assert_eq!(decimal_to_snafu(1), "1");
    assert_eq!(decimal_to_snafu(2), "2");
    assert_eq!(decimal_to_snafu(3), "1=");
    assert_eq!(decimal_to_snafu(4), "1-");
    assert_eq!(decimal_to_snafu(5), "10");
    assert_eq!(decimal_to_snafu(6), "11");
    assert_eq!(decimal_to_snafu(7), "12");
    assert_eq!(decimal_to_snafu(8), "2=");
    assert_eq!(decimal_to_snafu(9), "2-");
    assert_eq!(decimal_to_snafu(10), "20");
    assert_eq!(decimal_to_snafu(15), "1=0");
    assert_eq!(decimal_to_snafu(20), "1-0");
    assert_eq!(decimal_to_snafu(2022), "1=11-2");
    assert_eq!(decimal_to_snafu(12345), "1-0---0");
    assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0");

    assert_eq!(a(&input), "2=-1=0");
}
