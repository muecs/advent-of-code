//! Day 1: Report Repair

const TARGET_SUM: usize = 2020;

/// product of the two entries that sum to 2020
pub fn a(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    for i in 0..numbers.len()-1 {
        for j in i+1..numbers.len() {
            let n1 = &numbers[i];
            let n2 = &numbers[j];
            if *n1 + *n2 == TARGET_SUM {
                return (*n1 * *n2).to_string();
            }
        }
    }
    String::new()
}

/// product of the three entries that sum to 2020
pub fn b(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    for i in 0..numbers.len()-2 {
        for j in i+1..numbers.len()-1 {
            for k in j+1..numbers.len() {
                let n1 = &numbers[i];
                let n2 = &numbers[j];
                let n3 = &numbers[k];
                if *n1 + *n2 + *n3 == TARGET_SUM {
                    return (*n1 * *n2 * *n3).to_string();
                }
            }
        }
    }
    
    String::new()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    input.iter().map(|s| s.parse().unwrap()).collect()
}

#[test]
pub fn test() {
    let input = vec![
        "1721",
        "979",
        "366",
        "299",
        "675",
        "1456",
    ];

    assert_eq!(a(&input), "514579");
    assert_eq!(b(&input), "241861950");
}
