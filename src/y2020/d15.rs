//! Day 15: Rambunctious Recitation

// use std::collections::HashMap;

type Number = u32;
// type NumberMap = HashMap<Number, Number>;
// Note: BTreeMap and HashMap are too slow
type NumberMap = Vec<Number>;

/// determine the 2020th number spoken
pub fn a(input: &Vec<&str>) -> String {
    const TURNS: Number = 2020;
    let (mut numbers, mut number, mut turn) = parse_input(input, TURNS);
    while turn < TURNS {
        take_turn(&mut numbers, &mut number, &mut turn);
    }
    number.to_string()
}

/// determine the 30000000th number spoken
pub fn b(input: &Vec<&str>) -> String {
    const TURNS: Number = 30000000;
    let (mut numbers, mut number, mut turn) = parse_input(input, TURNS);
    while turn < TURNS {
        take_turn(&mut numbers, &mut number, &mut turn);
    }
    number.to_string()
}

fn parse_input(input: &Vec<&str>, capacity: Number) -> (NumberMap, usize, Number) {
    let mut numbers: Vec<usize> = input
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let last = numbers.pop().unwrap();

    let mut number_map = NumberMap::new();
    number_map.resize(capacity as usize, 0);

    let mut turn = 1;
    for &n in numbers.iter() {
        number_map[n] = turn;
        turn += 1;
    }

    (number_map, last, turn)
}

fn take_turn(numbers: &mut NumberMap, number: &mut usize, turn: &mut Number) {
    // println!("turn: {:?}, number: {:?}, numbers: {:?}", turn, number, numbers);
    let last_turn = numbers[*number];
    let next = if last_turn > 0 {
        (*turn - last_turn) as usize
    } else {
        0
    };
    numbers[*number] = *turn;
    *number = next;
    *turn += 1;
}

#[test]
pub fn test() {
    assert_eq!(a(&vec!["0,3,6"]), "436");
    assert_eq!(a(&vec!["1,3,2"]), "1");
    assert_eq!(a(&vec!["2,1,3"]), "10");
    assert_eq!(a(&vec!["1,2,3"]), "27");
    assert_eq!(a(&vec!["2,3,1"]), "78");
    assert_eq!(a(&vec!["3,2,1"]), "438");
    assert_eq!(a(&vec!["3,1,2"]), "1836");

    assert_eq!(b(&vec!["0,3,6"]), "175594");
    // assert_eq!(b(&vec!["1,3,2"]), "2578");
    // assert_eq!(b(&vec!["2,1,3"]), "3544142");
    // assert_eq!(b(&vec!["1,2,3"]), "261214");
    // assert_eq!(b(&vec!["2,3,1"]), "6895259");
    // assert_eq!(b(&vec!["3,2,1"]), "18");
    // assert_eq!(b(&vec!["3,1,2"]), "362");
}
