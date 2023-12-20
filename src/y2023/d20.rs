//! Day 20: Pulse Propagation

use crate::utils;
use std::collections::{HashMap, VecDeque};

/// product of low and high pulse count
pub fn a(input: &Vec<&str>) -> String {
    let (modules, cables) = parse_input(input);
    let mut flip_flop_states = vec![false; modules.len()];
    let mut cable_states = vec![false; cables.len()];

    let (low_count, high_count) = (0..1000).fold((0, 0), |acc, _| {
        let counts = propagate(
            &modules,
            &cables,
            &mut flip_flop_states,
            &mut cable_states,
            |_| {},
        );
        (acc.0 + counts.0, acc.1 + counts.1)
    });

    // println!("low: {low_count}, high: {high_count}");

    (low_count * high_count).to_string()
}

/// fewest number of button presses for low pulse on final cable
pub fn b(input: &Vec<&str>) -> String {
    let (modules, cables) = parse_input(input);
    let final_cable_idx = cables.iter().position(|&(_, i)| i == usize::MAX).unwrap();
    let mut flip_flop_states = vec![false; modules.len()];
    let mut cable_states = vec![false; cables.len()];
    cable_states[final_cable_idx] = true; // to detect low pulse (theoretically...)

    // Pulses going into the penultimate module must all be high to get a low
    // in 'rx'. There are a few distinct subgraphs starting right after
    // 'broadcaster', which have their own periodicity.
    let final_module_idx = cables[final_cable_idx].0;
    let subgraph_count = modules[final_module_idx].cables_in.len();
    let mut periods = Vec::<usize>::with_capacity(subgraph_count);

    let mut iterations = 0;
    while cable_states[final_cable_idx] && iterations < 10000 {
        iterations += 1;
        propagate(
            &modules,
            &cables,
            &mut flip_flop_states,
            &mut cable_states,
            |idx| {
                if idx == final_module_idx {
                    periods.push(iterations);
                }
            },
        );
        if periods.len() == subgraph_count {
            break;
        }
    }

    if !periods.is_empty() {
        // println!("Periods: {periods:?}");
        iterations = utils::lcm(&periods);
    }

    iterations.to_string()
}

fn parse_input(input: &Vec<&str>) -> (Modules, Cables) {
    let mut modules = Modules::with_capacity(input.len());
    let mut cables = Cables::new();

    // first pass: map module names to indices
    let module_map = input
        .iter()
        .map(|s| {
            let pos = s.chars().position(|c| c == ' ').unwrap();
            let idx = modules.len();
            let mut cables_in = Vec::new();
            let (mtype, name) = if s.starts_with('%') {
                (ModuleType::FlipFlop, &s[1..pos])
            } else if s.starts_with('&') {
                (ModuleType::Conjunction, &s[1..pos])
            } else {
                let n = &s[0..pos];
                if n == "broadcaster" {
                    // first cable
                    cables_in.push(0);
                    cables.push((usize::MAX, idx));
                }
                (ModuleType::Broadcast, n)
            };
            modules.push(Module {
                mtype,
                cables_in,
                cables_out: Vec::new(),
            });
            (name, (idx, &s[pos + 4..]))
        })
        .collect::<HashMap<_, _>>();

    // second pass: determine incoming and outgoing cable indices
    for &(idx, adj) in module_map.values() {
        if adj.is_empty() {
            continue;
        }
        for name in adj.split(", ") {
            let other_idx = module_map.get(name).map(|&(i, _)| i).unwrap_or(usize::MAX);
            modules[idx].cables_out.push(cables.len());
            if other_idx < modules.len() {
                modules[other_idx].cables_in.push(cables.len());
            }
            cables.push((idx, other_idx));
        }
    }

    (modules, cables)
}

fn propagate(
    modules: &Modules,
    cables: &Cables,
    flip_flop_states: &mut Vec<bool>,
    cable_states: &mut Vec<bool>,
    mut handle_conj_high_pulse: impl FnMut(usize),
) -> (usize, usize) {
    // start with cable to broadcaster
    let mut pulses = VecDeque::from([0]);
    let mut low_count = 0;
    let mut high_count = 0;
    while let Some(cable_idx) = pulses.pop_front() {
        let pulse = cable_states[cable_idx];
        if pulse {
            high_count += 1;
        } else {
            low_count += 1;
        }

        let receiver_idx = cables[cable_idx].1;
        if receiver_idx >= modules.len() {
            // not a real module
            continue;
        }

        let receiver = &modules[receiver_idx];
        let out = match receiver.mtype {
            ModuleType::FlipFlop => {
                if pulse {
                    // ignore high pulse
                    continue;
                }
                let state = &mut flip_flop_states[receiver_idx];
                *state = !*state; // toggle
                *state
            }
            ModuleType::Conjunction => {
                if pulse {
                    handle_conj_high_pulse(receiver_idx);
                }

                // low only if all incoming high
                !receiver.cables_in.iter().all(|i| cable_states[*i])
            }
            ModuleType::Broadcast => {
                pulse // just relay
            }
        };

        // send pulses
        for &cable_idx in &receiver.cables_out {
            cable_states[cable_idx] = out;
            pulses.push_back(cable_idx);
        }
    }

    (low_count, high_count)
}

#[derive(PartialEq)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}
struct Module {
    mtype: ModuleType,
    cables_in: Vec<usize>,
    cables_out: Vec<usize>,
}
type Cables = Vec<(usize, usize)>;
type Modules = Vec<Module>;

#[test]
pub fn test() {
    let input = vec![
        "broadcaster -> a, b, c",
        "%a -> b",
        "%b -> c",
        "%c -> inv",
        "&inv -> a",
    ];

    let input2 = vec![
        "broadcaster -> a",
        "%a -> inv, con",
        "&inv -> b",
        "%b -> con",
        "&con -> output",
    ];

    assert_eq!(a(&input), "32000000");
    assert_eq!(a(&input2), "11687500");
    assert_eq!(b(&input2), "1");
}
