//! Day 4: Giant Squid

use std::collections::BTreeSet;

type Board = Vec<u8>;

/// sum of unmarked numbers on first winning board multiplied by called number
pub fn a(input: &Vec<&str>) -> String {
    let (numbers, mut boards) = parse_input(&input);

    for number in numbers {
        for board in &mut boards {
            mark_board(board, number);
            if check_board(board) {
                let sum = board
                    .iter()
                    .filter(|&&x| x != u8::MAX)
                    .map(|&x| u32::from(x))
                    .sum::<u32>();
                return (u32::from(number) * sum).to_string();
            }
        }
    }

    String::new()
}

/// sum of unmarked numbers on last winning board multiplied by called number
pub fn b(input: &Vec<&str>) -> String {
    let (numbers, mut boards) = parse_input(&input);
    let count = boards.len();
    let mut winners = BTreeSet::new();
    for number in numbers {
        println!("Calling {number}");
        for (i, board) in boards.iter_mut().enumerate() {
            if winners.contains(&i) {
                continue;
            }
            mark_board(board, number);
            if check_board(board) && winners.insert(i) {
                let sum = board
                    .iter()
                    .filter(|&&x| x != u8::MAX)
                    .map(|&x| u32::from(x))
                    .sum::<u32>();
                let score = u32::from(number) * sum;
                println!("> Board {i} scored {score} ({}/{})", winners.len(), count);
                if winners.len() == count {
                    // Note: this will not give the right solution as the two
                    //       last boards are called at the same time
                    //
                    //       Calling 87
                    //       > Board 59 scored 14877 (99/100)  <- accepted solution
                    //       > Board 63 scored 8265 (100/100)  <- printed solution
                    return score.to_string();
                }
            }
        }
    }

    String::new()
}

fn parse_input(input: &Vec<&str>) -> (Vec<u8>, Vec<Board>) {
    let numbers = input[0]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u8>>();
    
    let boards = input
        .iter()
        .skip(1)
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_whitespace().map(|s| s.parse().unwrap()).collect::<Vec<u8>>())
        .collect::<Vec<u8>>()
        .chunks_exact(25)
        .map(&Vec::from)
        .collect();
    
    (numbers, boards)
}

/// sets the given number to MAX
fn mark_board(board: &mut Board, number: u8) {
    if let Some(item) = board.iter_mut().find(|x| **x == number) {
        *item = u8::MAX;
    }
}

/// determines whether the board is a winner
fn check_board(board: &Board) -> bool {
    assert_eq!(board.len(), 25);

    for i in 0..4 {
        // check row
        if board.iter().skip(i * 5).take(5).filter(|&&x| x == u8::MAX).count() == 5 {
            return true;
        }

        // check column
        if board.iter().skip(i).step_by(5).filter(|&&x| x == u8::MAX).count() == 5 {
            return true;
        }
    }

    false
}

#[test]
pub fn test() {
    let input = vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
        "",
        "22 13 17 11  0",
        " 8  2 23  4 24",
        "21  9 14 16  7",
        " 6 10  3 18  5",
        " 1 12 20 15 19",
        "",
        " 3 15  0  2 22",
        " 9 18 13 17  5",
        "19  8  7 25 23",
        "20 11 10 24  4",
        "14 21 16 12  6",
        "",
        "14 21 17 24  4",
        "10 16 15  9 19",
        "18  8 23 26 20",
        "22 11 13  6  5",
        " 2  0 12  3  7",
    ];

    let (numbers, boards) = parse_input(&input);
    assert_eq!(numbers.len(), 27);
    assert_eq!(boards.len(), 3);

    assert!(!check_board(&boards[0]));
    assert!(check_board(&vec![
        1u8, 2, 3, 4, 5,
        6, 7, 8, 9, 10,
        255, 255, 255, 255, 255,
        11, 12, 13, 14, 15,
        16, 17, 18, 19, 20,
    ]));
    assert!(check_board(&vec![
        1u8, 2, 255, 4, 5,
        6, 7, 255, 9, 10,
        11, 12, 255, 14, 15,
        16, 17, 255, 19, 20,
        21, 22, 255, 24, 25,
    ]));

    assert_eq!(a(&input), "4512");
    assert_eq!(b(&input), "1924");
}
