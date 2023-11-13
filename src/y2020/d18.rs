//! Day 18: Operation Order

/// sum of evaluated expressions with equal precedence
pub fn a(input: &Vec<&str>) -> String {
    let expressions = parse_input(input, false);
    expressions.iter().map(|e| solve(e)).sum::<u64>().to_string()
}

/// sum of evaluated expressions with addition having higher precedence
pub fn b(input: &Vec<&str>) -> String {
    let expressions = parse_input(input, true);
    expressions.iter().map(|e| solve(e)).sum::<u64>().to_string()
}

fn parse_input(input: &Vec<&str>, advanced: bool) -> Vec<Expression> {
    input
        .iter()
        .map(|s| {
            let mut stack = vec![Expression::new()];
            for b in s.bytes() {
                match b {
                    b'0'..=b'9' => stack.last_mut().unwrap().push(Element::Number((b - b'0') as u64)),
                    b'+' => stack.last_mut().unwrap().push(Element::Operator(Operator::Add)),
                    b'*' => stack.last_mut().unwrap().push(Element::Operator(Operator::Multiply)),
                    b'(' => stack.push(Expression::new()),
                    b')' => {
                        let expression = stack.pop().unwrap();
                        stack.last_mut().unwrap().push(Element::Parentheses(expression));
                    }
                    _ => {},
                }
                if advanced {
                    if let Some(expression) = stack.last_mut() {
                        if let Some(Element::Operator(op)) = expression.iter().nth_back(1) {
                            if *op == Operator::Add {
                                // precedence for '+', so wrap 'a + b' in parentheses
                                let exp = expression.split_off(expression.len() - 3);
                                expression.push(Element::Parentheses(exp));
                            }
                        }
                    }
                }
            }
            stack.pop().unwrap()
        })
        .collect()
}

fn solve(expression: &Expression) -> u64 {
    let mut result = 0;
    let mut operator = Operator::None;
    for e in expression {
        match e {
            Element::Number(n) => match operator {
                Operator::None => result = *n,
                Operator::Add => result += n,
                Operator::Multiply => result *= n,
            },
            Element::Operator(op) => operator = *op,
            Element::Parentheses(exp) => {
                let n = solve(exp);
                match operator {
                    Operator::None => result = n,
                    Operator::Add => result += n,
                    Operator::Multiply => result *= n,
                };
            },
        }
    }

    result
}

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    None,
    Add,
    Multiply,
}

enum Element {
    Number(u64),
    Operator(Operator),
    Parentheses(Expression),
}

type Expression = Vec<Element>;

#[test]
pub fn test() {
    let input = vec![
        "1 + 2 * 3 + 4 * 5 + 6", // 71 | 231
        "1 + (2 * 3) + (4 * (5 + 6))", // 51 | 51
        "2 * 3 + (4 * 5)", // 26 | 46
        "5 + (8 * 3 + 9 + 3 * 4 * 3)", // 437 | 1445
        "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", // 12240 | 669060
        "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", // 13632 | 23340
    ];

    assert_eq!(a(&input), "26457");
    assert_eq!(b(&input), "694173");
}
