//! Day 21: Keypad Conundrum

use std::collections::HashMap;

/// sum of complexities of shortest button sequences across chain of 3 robots
pub fn a(input: &Vec<&str>) -> String {
    let mut cache = Cache::new();
    input
        .iter()
        .map(|code| {
            let len = presses_on_dirpad(NUMPAD, &parse_code(code), 3, &mut cache);
            complexity(code, len)
        })
        .sum::<usize>()
        .to_string()
}

/// sum of complexities of shortest button sequences across chain of 26 robots
pub fn b(input: &Vec<&str>) -> String {
    let mut cache = Cache::new();
    input
        .iter()
        .map(|code| {
            let len = presses_on_dirpad(NUMPAD, &parse_code(code), 26, &mut cache);
            complexity(code, len)
        })
        .sum::<usize>()
        .to_string()
}

/// converts ASCII character to numeric keypad index
fn numbtn(b: u8) -> usize {
    match b {
        b'7'..=b'9' => (b - b'7') as usize,
        b'4'..=b'6' => (b - b'4' + 3) as usize,
        b'1'..=b'3' => (b - b'1' + 6) as usize,
        b'0' => 10,
        b'A' => 11,
        _ => unreachable!(),
    }
}

fn parse_code(code: &str) -> Vec<usize> {
    code.bytes().map(numbtn).collect()
}

/// translates sequence via graph to directional keypad presses
fn presses_on_dirpad(keypad: &str, source: &Sequence, depth: usize, cache: &mut Cache) -> usize {
    #[cfg(test)]
    print_sequence(keypad, &source);

    if depth == 0 {
        return source.len();
    }

    let mut len = 0;
    let mut curr = if keypad.len() == 6 { 2 } else { 11 };
    for src in source {
        if let Some(n) = cache.get(&(depth, curr, *src)) {
            len += *n;
        } else {
            let mut seq = find_path(keypad, &curr, src);
            seq.push(2); // A
            let n = presses_on_dirpad(DIRPAD, &seq, depth - 1, cache);
            len += n;
            cache.insert((depth, curr, *src), n);
        }
        curr = *src;
    }
    len
}

/// cheapest path through keypad graph
fn find_path(keypad: &str, start: &usize, end: &usize) -> Sequence {
    // Must go in at most two straight lines, avoiding empty button.
    // Start with leftmost directional buttons to minimize travel.
    //
    //   789
    //   456   ^A
    //   123  <v>
    //    0A
    //
    // Both keypads have 3 columns:
    let dx = (end % 3) as i8 - (start % 3) as i8;
    let dy = (end / 3) as i8 - (start / 3) as i8;
    // println!("{start} -> {end} ({dx}, {dy})");

    let x_moves = [if dx < 0 { 3 } else { 5 }].repeat(dx.unsigned_abs().into());
    let y_moves = [if dy < 0 { 1 } else { 4 }].repeat(dy.unsigned_abs().into());

    let x_blocked = keypad.as_bytes()[start.saturating_add_signed(dx.into())] == b' ';
    let y_blocked = keypad.as_bytes()[start.saturating_add_signed((3 * dy).into())] == b' ';

    // prefer going left, then down/up, then right
    if (dx < 0 && !x_blocked) || y_blocked {
        [x_moves, y_moves].concat()
    } else {
        [y_moves, x_moves].concat()
    }
}

/// product of length of button sequence and numeric part of code
fn complexity(code: &str, len: usize) -> usize {
    len * code[0..3].parse::<usize>().unwrap()
}

#[cfg(test)]
fn format_sequence(keypad: &str, seq: &Sequence) -> String {
    seq.iter().map(|&i| &keypad[i..=i]).collect()
}

#[cfg(test)]
fn print_sequence(keypad: &str, seq: &Sequence) {
    println!("{}", format_sequence(keypad, seq));
}

const NUMPAD: &str = "789456123 0A";
const DIRPAD: &str = " ^A<v>";

type Sequence = Vec<usize>;
type Cache = HashMap<(usize, usize, usize), usize>;

#[test]
pub fn test() {
    let input = vec!["029A", "980A", "179A", "456A", "379A"];

    let code = parse_code(input[0]);
    assert_eq!(format_sequence(NUMPAD, &code), input[0]);

    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'A'), &numbtn(b'0'))),
        "<"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'0'), &numbtn(b'2'))),
        "^"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'A'), &numbtn(b'1'))),
        "^<<"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'2'), &numbtn(b'9'))),
        "^^>"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'9'), &numbtn(b'A'))),
        "vvv"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'A'), &numbtn(b'5'))),
        "<^^"
    );
    assert_eq!(
        format_sequence(DIRPAD, &find_path(NUMPAD, &numbtn(b'7'), &numbtn(b'A'))),
        ">>vvv"
    );

    assert_eq!(presses_on_dirpad(NUMPAD, &code, 1, &mut Cache::new()), 12);
    assert_eq!(presses_on_dirpad(NUMPAD, &code, 2, &mut Cache::new()), 28);
    assert_eq!(presses_on_dirpad(NUMPAD, &code, 3, &mut Cache::new()), 68);

    assert_eq!(a(&input), "126384");
    // assert_eq!(b(&input), "");
}
