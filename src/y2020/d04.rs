//! Day 4: Passport Processing

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// number of valid passport fields, ignoring cid
pub fn a(input: &Vec<&str>) -> String {
    let passports = parse_input(input);
    passports
        .iter()
        .map(|p| validate(p, false) as usize)
        .sum::<usize>()
        .to_string()
}

/// number of valid passport fields and values, ignoring cid
pub fn b(input: &Vec<&str>) -> String {
    let passports = parse_input(input);
    passports
        .iter()
        .map(|p| validate(p, true) as usize)
        .sum::<usize>()
        .to_string()
}

fn parse_input<'a>(input: &'a Vec<&str>) -> Vec<HashMap<&'a str, &'a str>> {
    let mut passports = Vec::new();
    let mut current_passport = HashMap::new();
    for line in input {
        if line.is_empty() {
            if !current_passport.is_empty() {
                passports.push(current_passport);
                current_passport = HashMap::new();
            }
            continue;
        }
        line.split_whitespace()
            .map(|s| s.split_once(':').unwrap())
            .for_each(|(k, v)| {
                current_passport.insert(k, v);
            });
    }
    if !current_passport.is_empty() {
        passports.push(current_passport);
    }

    passports
}

fn validate(passport: &HashMap<&str, &str>, strict: bool) -> bool {
    for field in &FIELDS {
        if !passport.contains_key(field) {
            return false;
        }
    }

    if strict {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if !parse_in_range(passport["byr"], 1920, 2002) {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if !parse_in_range(passport["iyr"], 2010, 2020) {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if !parse_in_range(passport["eyr"], 2020, 2030) {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        let hgt = passport["hgt"];
        if hgt.len() < 4 {
            return false;
        }
        let (height, unit) = hgt.split_at(hgt.len() - 2);
        if !((unit == "cm" && parse_in_range(height, 150, 193))
            || (unit == "in" && parse_in_range(height, 59, 76)))
        {
            return false;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        lazy_static! {
            static ref HCL_REGEX: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
        }
        if !HCL_REGEX.is_match(passport["hcl"]) {
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&passport["ecl"]) {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        lazy_static! {
            static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
        }
        if !PID_REGEX.is_match(passport["pid"]) {
            return false;
        }
    }

    true
}

fn parse_in_range(value: &str, min: u16, max: u16) -> bool {
    if let Ok(v) = value.parse::<u16>() {
        if v >= min && v <= max {
            return true;
        }
    }
    false
}

const FIELDS: [&str; 7] = [
    "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /* , "cid" */
];

#[test]
pub fn test() {
    let input = vec![
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd",
        "byr:1937 iyr:2017 cid:147 hgt:183cm",
        "",
        "iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884",
        "hcl:#cfa07d byr:1929",
        "",
        "hcl:#ae17e1 iyr:2013",
        "eyr:2024",
        "ecl:brn pid:760753108 byr:1931",
        "hgt:179cm",
        "",
        "hcl:#cfa07d eyr:2025 pid:166559648",
        "iyr:2011 ecl:brn hgt:59in",
    ];

    assert_eq!(a(&input), "2");

    let input_invalid = vec![
        "eyr:1972 cid:100",
        "hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926",
        "",
        "iyr:2019",
        "hcl:#602927 eyr:1967 hgt:170cm",
        "ecl:grn pid:012533040 byr:1946",
        "",
        "hcl:dab227 iyr:2012",
        "ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277",
        "",
        "hgt:59cm ecl:zzz",
        "eyr:2038 hcl:74454a iyr:2023",
        "pid:3556412378 byr:2007",
    ];

    let input_valid = vec![
        "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980",
        "hcl:#623a2f",
        "",
        "eyr:2029 ecl:blu cid:129 byr:1989",
        "iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm",
        "",
        "hcl:#888785",
        "hgt:164cm byr:2001 iyr:2015 cid:88",
        "pid:545766238 ecl:hzl",
        "eyr:2022",
        "",
        "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719",
    ];

    assert_eq!(b(&input_invalid), "0");
    assert_eq!(b(&input_valid), "4");
}
