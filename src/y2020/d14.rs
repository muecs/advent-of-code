//! Day 14: Docking Data

use std::collections::BTreeMap;

/// sum of values in memory using mask on values when storing
pub fn a(input: &Vec<&str>) -> String {
    let mut mask_on = 0;
    let mut mask_off = u64::MAX;
    let mut memory = BTreeMap::new();

    for line in input {
        if &line[0..4] == "mask" {
            let mask = &line[7..];
            mask_on = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            mask_off = u64::from_str_radix(&mask.replace("X", "1"), 2).unwrap();
        } else {
            let (address, value) = line[4..].split_once("] = ").unwrap();
            let address = address.parse::<u64>().unwrap();
            let value = value.parse::<u64>().unwrap() & mask_off | mask_on;
            memory.insert(address, value);
        }
    }

    memory.values().sum::<u64>().to_string()
}

/// sum of values in memory using mask on memory addresses when storing
pub fn b(input: &Vec<&str>) -> String {
    let mut mask_on = 0;
    let mut mask_floating = Vec::new();
    let mut memory = BTreeMap::new();

    for line in input {
        if &line[0..4] == "mask" {
            let mask = &line[7..];
            mask_on = u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap();
            mask_floating = mask
                .bytes()
                .enumerate()
                .filter_map(|(i, b)| if b == b'X' { Some(i) } else { None })
                .collect();
        } else {
            let (address, value) = line[4..].split_once("] = ").unwrap();
            let address = address.parse::<u64>().unwrap() | mask_on;
            let value = value.parse::<u64>().unwrap();
            let n = mask_floating.len();
            for i in 0..1 << n {
                let mask_bin = format!("{i:0n$b}");
                let mut address_bin = format!("{address:036b}");
                for m in 0..n {
                    let a = mask_floating[m];
                    address_bin.replace_range(a..a + 1, &mask_bin[m..m + 1]);
                }
                memory.insert(u64::from_str_radix(&address_bin, 2).unwrap(), value);
            }
        }
    }

    memory.values().sum::<u64>().to_string()
}

#[test]
pub fn test() {
    let input = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
        "mem[8] = 11",
        "mem[7] = 101",
        "mem[8] = 0",
    ];

    assert_eq!(a(&input), "165");

    let input = vec![
        "mask = 000000000000000000000000000000X1001X",
        "mem[42] = 100",
        "mask = 00000000000000000000000000000000X0XX",
        "mem[26] = 1",
    ];

    assert_eq!(b(&input), "208");
}
