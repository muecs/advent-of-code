//! Day 10: Syntax Scoring

/// total syntax error score for corrupted lines
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|&line| get_syntax_score(line).err().unwrap_or_default())
        .sum::<usize>()
        .to_string()
}

/// middle value of sorted completion scores
pub fn b(input: &Vec<&str>) -> String {
    let mut scores = input
        .iter()
        .filter_map(|line| get_syntax_score(line).ok())
        .collect::<Vec<_>>();
    let mid = scores.len() / 2;
    scores
        .select_nth_unstable(mid)
        .1
        .to_string()
}

/// calculates completion or error score
fn get_syntax_score(line: &str) -> Result<usize, usize> {
    let mut stack = Vec::new();

    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => stack.push(c),
            _ => if let Some(pair) = stack.pop() {
                match c {
                    ')' if pair != '(' => return Err(3),
                    ']' if pair != '[' => return Err(57),
                    '}' if pair != '{' => return Err(1197),
                    '>' if pair != '<' => return Err(25137),
                    _ => {},
                }
            }
        }
    }

    Ok(
        stack.iter().rev().fold(0usize, |total, &c| total * 5 + match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })
    )
}

#[test]
pub fn test() {
    let input = vec![
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    assert_eq!(get_syntax_score(input[0]), Ok(288957));
    assert_eq!(get_syntax_score(input[1]), Ok(5566));
    assert_eq!(get_syntax_score(input[2]), Err(1197));
    assert_eq!(get_syntax_score(input[4]), Err(3));
    assert_eq!(get_syntax_score(input[5]), Err(57));
    assert_eq!(get_syntax_score(input[8]), Err(25137));

    assert_eq!(a(&input), "26397");
    assert_eq!(b(&input), "288957");
}
