//! Day 24: Crossed Wires

use std::collections::HashMap;

/// decimal number calculated by logic circuit
pub fn a(input: &Vec<&str>) -> String {
    let (mut states, graph) = parse_input(input);

    graph
        .keys()
        .filter_map(|wire| {
            wire.starts_with("z").then(|| {
                (evaluate(wire, &graph, &mut states) as usize)
                    << wire[1..].parse::<usize>().unwrap()
            })
        })
        .sum::<usize>()
        .to_string()
}

/// swap output wires of 4 pairs of gates to make circuit a ripple-carry adder
pub fn b(input: &Vec<&str>) -> String {
    // half adder: inputs XOR'd is result bit, inputs AND'd is carry bit
    // full adder: second half adder - prev result with input XOR'd is result, carry bits get OR'd
    // -> results always follows XOR (except last), which (except first) follows XOR and OR, with xor following two inputs and OR following two ANDs
    let (_, graph) = parse_input(input);
    let outputs = graph
        .keys()
        .filter(|wire| wire.starts_with("z"))
        .copied()
        .collect::<Vec<_>>();
    let max_out = format!("z{:02}", outputs.len() - 1);
    let mut swaps = Vec::new();
    for output in &outputs {
        if ["z00", "z01", &max_out].contains(output) {
            continue;
        }
        let gate = &graph[output];
        if gate.op != Op::Xor {
            println!("! {output} follows {:?}", gate.op);
            swaps.push(*output);
            continue;
        }
        for (gate_in_wire, gate_in_gate) in [
            graph.get_key_value(gate.in1).unwrap(),
            graph.get_key_value(gate.in2).unwrap(),
        ] {
            if gate_in_gate.op == Op::And {
                println!(
                    "! {output} follows {} follows {:?}",
                    gate_in_wire, gate_in_gate.op
                );
                swaps.push(*gate_in_wire);
            } else if gate_in_gate.op == Op::Xor {
                for gate_in_gate_in_wire in [gate_in_gate.in1, gate_in_gate.in2] {
                    if ![b'x', b'y'].contains(&gate_in_gate_in_wire.as_bytes()[0]) {
                        println!(
                            "! {output} follows {} follows {} (not input)",
                            gate_in_wire, gate_in_gate_in_wire
                        );
                        swaps.push(*gate_in_wire);
                        break;
                    }
                }
            } else if gate_in_gate.op == Op::Or {
                for (gate_in_gate_in_wire, gate_in_gate_in_gate) in [
                    graph.get_key_value(gate_in_gate.in1).unwrap(),
                    graph.get_key_value(gate_in_gate.in2).unwrap(),
                ] {
                    if gate_in_gate_in_gate.op != Op::And {
                        println!(
                            "! {output} follows {} follows {} follows {:?}",
                            gate_in_wire, gate_in_gate_in_wire, gate_in_gate_in_gate.op
                        );
                        swaps.push(*gate_in_gate_in_wire);
                    }
                }
            }
        }
    }

    swaps.sort_unstable();
    swaps.join(",")
}

fn parse_input<'a>(input: &'a Vec<&'a str>) -> (States<'a>, Graph<'a>) {
    let mut it = input.iter();
    let states = it
        .by_ref()
        .map_while(|s| {
            (!s.is_empty()).then(|| {
                s.split_once(": ")
                    .map(|(wire, val)| (wire, val != "0"))
                    .unwrap()
            })
        })
        .collect();
    let graph = it
        .map(|s| {
            let mut parts = s.split_whitespace();
            let gate = Gate {
                in1: parts.next().unwrap(),
                op: match parts.next() {
                    Some("AND") => Op::And,
                    Some("OR") => Op::Or,
                    Some("XOR") => Op::Xor,
                    _ => unreachable!(),
                },
                in2: parts.next().unwrap(),
            };
            parts.next(); // ->
            let out = parts.next().unwrap();
            // println!("  {0} -> {0}_{2:?}_{1} [label=\"in1\"];\n  {1} -> {0}_{2:?}_{1} [label=\"in2\"];\n  {0}_{2:?}_{1} -> {3};\n  {0}_{2:?}_{1} [label=\"{2:?}\"];", gate.in1, gate.in2, gate.op, out);
            (out, gate)
        })
        .collect();
    (states, graph)
}

fn evaluate(wire: &str, graph: &Graph, states: &mut States) -> bool {
    if let Some(state) = states.get(wire) {
        *state
    } else {
        let gate = &graph[wire];
        let val1 = evaluate(gate.in1, graph, states);
        let val2 = evaluate(gate.in2, graph, states);
        match gate.op {
            Op::And => val1 & val2,
            Op::Or => val1 | val2,
            Op::Xor => val1 ^ val2,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug)]
struct Gate<'a> {
    in1: &'a str,
    in2: &'a str,
    op: Op,
}

type States<'a> = HashMap<&'a str, bool>;
type Graph<'a> = HashMap<&'a str, Gate<'a>>;

#[test]
pub fn test() {
    let input1 = vec![
        "x00: 1",
        "x01: 1",
        "x02: 1",
        "y00: 0",
        "y01: 1",
        "y02: 0",
        "",
        "x00 AND y00 -> z00",
        "x01 XOR y01 -> z01",
        "x02 OR y02 -> z02",
    ];
    let input2 = vec![
        "x00: 1",
        "x01: 0",
        "x02: 1",
        "x03: 1",
        "x04: 0",
        "y00: 1",
        "y01: 1",
        "y02: 1",
        "y03: 1",
        "y04: 1",
        "",
        "ntg XOR fgs -> mjb",
        "y02 OR x01 -> tnw",
        "kwq OR kpj -> z05",
        "x00 OR x03 -> fst",
        "tgd XOR rvg -> z01",
        "vdt OR tnw -> bfw",
        "bfw AND frj -> z10",
        "ffh OR nrd -> bqk",
        "y00 AND y03 -> djm",
        "y03 OR y00 -> psh",
        "bqk OR frj -> z08",
        "tnw OR fst -> frj",
        "gnj AND tgd -> z11",
        "bfw XOR mjb -> z00",
        "x03 OR x00 -> vdt",
        "gnj AND wpb -> z02",
        "x04 AND y00 -> kjc",
        "djm OR pbm -> qhw",
        "nrd AND vdt -> hwm",
        "kjc AND fst -> rvg",
        "y04 OR y02 -> fgs",
        "y01 AND x02 -> pbm",
        "ntg OR kjc -> kwq",
        "psh XOR fgs -> tgd",
        "qhw XOR tgd -> z09",
        "pbm OR djm -> kpj",
        "x03 XOR y03 -> ffh",
        "x00 XOR y04 -> ntg",
        "bfw OR bqk -> z06",
        "nrd XOR fgs -> wpb",
        "frj XOR qhw -> z04",
        "bqk OR frj -> z07",
        "y03 OR x01 -> nrd",
        "hwm AND bqk -> z03",
        "tgd XOR rvg -> z12",
        "tnw OR pbm -> gnj",
    ];

    assert_eq!(a(&input1), "4");
    assert_eq!(a(&input2), "2024");
    // assert_eq!(b(&input), "");
}
