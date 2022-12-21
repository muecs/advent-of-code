//! Day 20: Grove Positioning System

/// sum of 1000th/2000th/3000th numbers after mixing
pub fn a(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    let mixed = mix(&numbers, 1);
    let offset = mixed.iter().position(|n| *n == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + offset) % mixed.len()])
        .sum::<isize>()
        .to_string()
}

/// sum of 1000th/2000th/3000th decrypted numbers after 10x mixing
pub fn b(input: &Vec<&str>) -> String {
    let numbers = parse_input(input);
    let mixed = mix(&decrypt(&numbers), 10);
    let offset = mixed.iter().position(|n| *n == 0).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|i| mixed[(i + offset) % mixed.len()])
        .sum::<isize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<isize> {
    input.iter().map(|line| line.parse().unwrap()).collect()
}

fn mix(numbers: &Vec<isize>, rounds: usize) -> Vec<isize> {
    let len = numbers.len() as isize;
    let mut num_to_pos = Vec::from_iter(0..numbers.len());
    let mut pos_to_num = num_to_pos.to_owned();

    // println!("n | {numbers:?}");

    for _ in 0..rounds {
        for num in 0..numbers.len() {
            let old_pos = num_to_pos[num];
            let new_pos = (old_pos as isize + numbers[num]).rem_euclid(len - 1) as usize;
            let new_pos = if new_pos != old_pos && new_pos == 0 {
                numbers.len() - 1
            } else {
                new_pos
            };

            let range: Box<dyn Iterator<Item = usize>> = if old_pos <= new_pos {
                Box::new(old_pos..new_pos)
            } else {
                Box::new((new_pos..old_pos).rev())
            };

            for pos in range {
                pos_to_num.swap(pos, pos + 1);
                num_to_pos.swap(pos_to_num[pos], pos_to_num[pos + 1]);
            }

            // println!(
            //     "{num} | {:?}",
            //     pos_to_num
            //         .iter()
            //         .map(|num| numbers[*num])
            //         .collect::<Vec<_>>()
            // );
        }
    }

    pos_to_num.iter().map(|num| numbers[*num]).collect()
}

fn decrypt(numbers: &Vec<isize>) -> Vec<isize> {
    const KEY: isize = 811589153;
    numbers.iter().map(|n| *n * KEY).collect()
}

#[test]
pub fn test() {
    let input = vec!["1", "2", "-3", "3", "-2", "0", "4"];

    let numbers = parse_input(&input);
    assert_eq!(mix(&numbers, 1), vec![1, 2, -3, 4, 0, 3, -2]);
    assert_eq!(
        decrypt(&numbers),
        vec![
            811589153,
            1623178306,
            -2434767459,
            2434767459,
            -1623178306,
            0,
            3246356612
        ]
    );
    assert_eq!(
        mix(&decrypt(&numbers), 1),
        vec![
            0,
            -2434767459,
            3246356612,
            -1623178306,
            2434767459,
            1623178306,
            811589153
        ]
    );
    assert_eq!(
        mix(&decrypt(&numbers), 2),
        vec![
            0,
            2434767459,
            1623178306,
            3246356612,
            -2434767459,
            -1623178306,
            811589153
        ]
    );

    assert_eq!(a(&input), "3");
    assert_eq!(b(&input), "1623178306");
}
