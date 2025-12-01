//! Day 1: Secret Entrance

/// Number of rotations landing on zero
pub fn a(input: &Vec<&str>) -> String {
    let mut val = 50;
    let mut n = 0;
    for l in input {
        let steps = l[1..].parse::<u16>().unwrap();
        assert!(steps < 1000);
        match &l[0..1] {
            "L" => val = (val + 1000 - steps) % 100,
            "R" => val = (val + steps) % 100,
            _ => unreachable!(),
        }
        if val == 0 {
            n += 1;
        }
    }
    n.to_string()
}

/// Number of rotations passing zero
pub fn b(input: &Vec<&str>) -> String {
    let mut val = 50;
    let mut n = 0;
    for l in input {
        let steps = l[1..].parse::<u16>().unwrap();
        assert!(steps < 1000);
        assert_ne!(steps, 0);
        match &l[0..1] {
            "L" => {
                n += ((100 - val) % 100 + steps) / 100;
                val = (val + 1000 - steps) % 100;
            }
            "R" => {
                n += (val + steps) / 100;
                val = (val + steps) % 100;
            }
            _ => unreachable!(),
        }
    }
    n.to_string()
}

#[test]
pub fn test() {
    let input = vec![
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    assert_eq!(a(&input), "3");
    assert_eq!(b(&input), "6");
}
