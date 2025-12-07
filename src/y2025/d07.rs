//! Day 7: Laboratories

/// number of beam splits
pub fn a(input: &Vec<&str>) -> String {
    simulate(input, false).to_string()
}

/// permutations of beam paths
pub fn b(input: &Vec<&str>) -> String {
    simulate(input, true).to_string()
}

fn simulate(input: &Vec<&str>, paths: bool) -> usize {
    let mut beams = vec![0usize; input[0].len()];
    let mut min = input[0].len() / 2;
    let mut max = min;
    let mut splits = 0usize;
    for line in input {
        // println!("Line: {}", &line[min..max+1]);
        for i in min..=max {
            let field = line.as_bytes()[i];
            // println!("{i} {} {}", field as char, beams[i]);
            match field {
                b'S' => beams[i] = 1,
                b'^' if beams[i] > 0 => {
                    splits += 1;
                    let curr_beams = beams[i];
                    beams[i] = 0;
                    if let Some(b) = beams.get_mut(i - 1) {
                        *b += curr_beams;
                        min = min.min(i - 1);
                    }
                    if let Some(b) = beams.get_mut(i + 1) {
                        *b += curr_beams;
                        max = max.max(i + 1);
                    }
                    beams[i] = 0;
                }
                _ => {}
            }
        }
    }
    if paths {
        beams.iter().sum()
    } else {
        splits
    }
}

#[test]
pub fn test() {
    let input = vec![
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    assert_eq!(a(&input), "21");
    assert_eq!(b(&input), "40");
}
