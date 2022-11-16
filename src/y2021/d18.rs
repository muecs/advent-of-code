//! Day 18: Snailfish

use std::{str::FromStr, ops::Add, fmt::Display};

#[derive(Clone, Debug, PartialEq)]
enum SnailfishNumber {
    Pair(Box<SnailfishNumber>, Box<SnailfishNumber>),
    Value(usize),
}

impl SnailfishNumber {
    /// Recursively calculates the number's magnitude.
    pub fn magnitude(&self) -> usize {
        match self {
            Self::Pair(a, b) => 3 * a.magnitude() + 2 * b.magnitude(),
            Self::Value(v) => *v,
        }
    }

    /// Applies 'explode' operation and return whether it happened.
    /// 
    /// If any pair is nested inside four pairs, the leftmost such pair explodes.
    fn try_explode(&mut self) -> bool {
        /// Recurses to leftmost leaf and adds value.
        fn add_to_leftmost(
            number: &mut SnailfishNumber,
            lval: &usize,
        ) {
            match number {
                SnailfishNumber::Pair(lnum, _) => add_to_leftmost(lnum, lval),
                SnailfishNumber::Value(v) => *v += *lval,
            }
        }

        /// Recurses to rightmost leaf and adds value.
        fn add_to_rightmost(
            number: &mut SnailfishNumber,
            rval: &usize,
        ) {
            match number {
                SnailfishNumber::Pair(_, rnum) => add_to_rightmost(rnum, rval),
                SnailfishNumber::Value(v) => *v += *rval,
            }
        }

        /// Internal explode - tracks depth and bubbles up values to add.
        fn explode(
            number: &mut SnailfishNumber,
            depth: usize,
            lval: &mut usize,
            rval: &mut usize
        ) -> bool {
            if let SnailfishNumber::Pair(lnum, rnum) = number {
                if depth == 4 {
                    if let SnailfishNumber::Value(v) = **lnum {
                        *lval = v;
                    }
                    if let SnailfishNumber::Value(v) = **rnum {
                        *rval = v;
                    }
                    *number = SnailfishNumber::Value(0);
                    true
                } else if explode(lnum, depth + 1, lval, rval) {
                    if *rval != 0 {
                        add_to_leftmost(rnum, rval);
                        *rval = 0;
                    }
                    true
                } else if explode(rnum, depth + 1, lval, rval) {
                    if *lval != 0 {
                        add_to_rightmost(lnum, lval);
                        *lval = 0;
                    }
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }

        let mut lval = 0;
        let mut rval = 0;
        explode(self, 0, &mut lval, &mut rval)
    }

    /// Apply 'split' operation and return whether it happened.
    /// 
    /// If any regular number is 10 or greater, the leftmost such regular number splits.
    fn try_split(&mut self) -> bool {
        match self {
            Self::Pair(lnum, rnum) => lnum.try_split() || rnum.try_split(),
            Self::Value(v) if *v >= 10 => {
                //let lval = v.div_floor(2);
                //let rval = v.div_ceil(2);
                let half = *v / 2;
                *self = Self::from((half, *v - half));
                true
            },
            _ => false,
        }
    }
}

impl From<(usize, usize)> for SnailfishNumber {
    fn from(pair: (usize, usize)) -> Self {
        Self::Pair(
            Box::new(Self::Value(pair.0)),
            Box::new(Self::Value(pair.1)),
        )
    }
}

impl From<(SnailfishNumber, usize)> for SnailfishNumber {
    fn from(pair: (SnailfishNumber, usize)) -> Self {
        Self::Pair(
            Box::new(pair.0),
            Box::new(Self::Value(pair.1)),
        )
    }
}

impl From<(usize, SnailfishNumber)> for SnailfishNumber {
    fn from(pair: (usize, SnailfishNumber)) -> Self {
        Self::Pair(
            Box::new(Self::Value(pair.0)),
            Box::new(pair.1),
        )
    }
}

impl From<(SnailfishNumber, SnailfishNumber)> for SnailfishNumber {
    fn from(pair: (SnailfishNumber, SnailfishNumber)) -> Self {
        Self::Pair(
            Box::new(pair.0),
            Box::new(pair.1),
        )
    }
}

impl FromStr for SnailfishNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s[0..=0].eq("[") {
            let mut open = 0;
            let mut mid = 0;
            for (i, c) in s.char_indices() {
                match c {
                    '[' => open += 1,
                    ']' => open -= 1,
                    ',' if open <= 1 => {
                        mid = i;
                        break;
                    },
                    _ => (),
                }
            }
            Ok(Self::Pair(
                Box::new(Self::from_str(&s[1..mid])?),
                Box::new(Self::from_str(&s[mid+1..s.len()-1])?),
            ))
        } else {
            Ok(Self::Value(s.parse().unwrap()))
        }
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pair(a, b) => write!(f, "[{},{}]", a, b),
            Self::Value(v) => write!(f, "{}", v),
        }
    }
}

impl Add for SnailfishNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = Self::Pair(Box::new(self), Box::new(rhs));

        while result.try_explode() || result.try_split() {}

        result
    }
}

/// magnitude of sum of snailfish numbers
pub fn a(input: &Vec<&str>) -> String {
    input
        .iter()
        .map(|&s| s.parse::<SnailfishNumber>().unwrap())
        .reduce(|acc, num| acc + num)
        .unwrap()
        .magnitude()
        .to_string()
}

/// largest magnitude of any sum of two different snailfish numbers
pub fn b(input: &Vec<&str>) -> String {
    let numbers = input
        .iter()
        .map(|&s| s.parse::<SnailfishNumber>().unwrap())
        .collect::<Vec<_>>();
    let mut max_magnitude = 0;
    for n1 in &numbers {
        for n2 in &numbers {
            if n1 != n2 {
                let m = (n1.to_owned() + n2.to_owned()).magnitude();
                if m > max_magnitude {
                    max_magnitude = m;
                }
            }
        }
    }
    max_magnitude.to_string()
}

#[test]
pub fn test() {
    let input = vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
        "[[[5,[2,8]],4],[5,[[9,9],0]]]",
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
        "[[[[5,4],[7,7]],8],[[8,3],8]]",
        "[[9,3],[[9,9],[6,[4,9]]]]",
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
    ];

    assert_eq!(SnailfishNumber::from_str("[1,2]"), Ok((1, 2).into()));
    assert_eq!(
        SnailfishNumber::from_str("[[1,2],3]"),
        Ok(SnailfishNumber::from((SnailfishNumber::from((1,2)), 3))),
    );
    assert_eq!(
        SnailfishNumber::from_str("[9,[8,7]]"),
        Ok(SnailfishNumber::from((9, SnailfishNumber::from((8,7))))),
    );
    assert_eq!(
        SnailfishNumber::from_str("[[1,9],[8,5]]"), 
        Ok(SnailfishNumber::from((
            SnailfishNumber::from((1,9)),
            SnailfishNumber::from((8,5)),
        ))),
    );

    let x = SnailfishNumber::from_str("[[1,2],[[3,4],5]]").unwrap();
    assert_eq!(x.magnitude(), 143);
    let x = SnailfishNumber::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").unwrap();
    assert_eq!(x.magnitude(), 1384);
    let x = SnailfishNumber::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]").unwrap();
    assert_eq!(x.magnitude(), 445);
    let x = SnailfishNumber::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]").unwrap();
    assert_eq!(x.magnitude(), 791);
    let x = SnailfishNumber::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]").unwrap();
    assert_eq!(x.magnitude(), 1137);
    let x = SnailfishNumber::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").unwrap();
    assert_eq!(x.magnitude(), 3488);

    let x = SnailfishNumber::from((1,2));
    let y = SnailfishNumber::from((SnailfishNumber::from((3,4)), 5));
    assert_eq!(x.to_string(), "[1,2]");
    assert_eq!(y.to_string(), "[[3,4],5]");
    assert_eq!((x + y).to_string(), "[[1,2],[[3,4],5]]");

    let x = SnailfishNumber::from_str("[3,[2,[1,[7,3]]]]").unwrap();
    let y = SnailfishNumber::from_str("[6,[5,[4,[3,2]]]]").unwrap();
    assert_eq!((x + y).to_string(), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    
    let x = SnailfishNumber::from_str("[[[[4,3],4],4],[7,[[8,4],9]]]").unwrap();
    let y = SnailfishNumber::from_str("[1,1]").unwrap();
    assert_eq!((x + y).to_string(), "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    assert_eq!(a(&input), "4140");
    assert_eq!(b(&input), "3993");
}
