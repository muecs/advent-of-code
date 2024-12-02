//! Day 2: Red-Nosed Reports

/// number of safe reports
pub fn a(input: &Vec<&str>) -> String {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|r| check(r, false))
        .count()
        .to_string()
}

/// number of safe reports with problem dampener
pub fn b(input: &Vec<&str>) -> String {
    let reports = parse_input(input);
    reports
        .iter()
        .filter(|r| check(r, true))
        .count()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Vec<isize>> {
    input
        .iter()
        .map(|s| s.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn check(levels: &Vec<isize>, dampen: bool) -> bool {
    if levels.len() < 2 {
        return false;
    }
    let mut increasing = false;
    let mut decreasing = false;
    let mut problem = false;
    for (i, win) in levels.windows(2).enumerate() {
        let diff = win[1] - win[0];
        if diff >= 1 && diff <= 3 {
            if decreasing {
                problem = true;
            } else {
                increasing = true;
            }
        } else if diff <= -1 && diff >= -3 {
            if increasing {
                problem = true;
            } else {
                decreasing = true;
            }
        } else {
            problem = true;
        }
        if problem {
            return dampen
                && ({
                    let mut new_levels = levels.clone();
                    new_levels.remove(i);
                    check(&new_levels, false)
                } || {
                    let mut new_levels = levels.clone();
                    new_levels.remove(i + 1);
                    check(&new_levels, false)
                } || (i > 0 && {
                    let mut new_levels = levels.clone();
                    new_levels.remove(i - 1);
                    check(&new_levels, false)
                }));
        }
    }
    true
}

#[test]
pub fn test() {
    let input = vec![
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    assert_eq!(a(&input), "2");
    assert_eq!(b(&input), "4");
}
