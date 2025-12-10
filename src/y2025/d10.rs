//! Day 10: Factory

use std::collections::{HashSet, VecDeque};

/// fewest button presses to configure indicator lights
pub fn a(input: &Vec<&str>) -> String {
    input.iter()
        .map(|line| Machine::from_str(line).min_presses_for_lights())
        .sum::<usize>()
        .to_string()
}

/// fewest button presses to configure joltage levels
pub fn b(input: &Vec<&str>) -> String {
    input.iter()
        .map(|line| Machine::from_str(line).min_presses_for_joltage())
        .sum::<usize>()
        .to_string()
}

#[derive(Default)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u16>,
}

impl Machine {
    fn from_str(s: &str) -> Self {
        let mut machine = Machine::default();
        let parts = s.split_whitespace().collect::<Vec<_>>();
        for i in 0..parts.len() {
            let part = &parts[i][1..parts[i].len()-1];
            if i == 0 {
                machine.lights = part.chars().map(|c| c == '#').collect();
            } else if i + 1 == parts.len() {
                machine.joltage = part.split(',').map(|s| s.parse().unwrap()).collect();
            } else {
                machine.buttons.push(part.split(',').map(|s| s.parse().unwrap()).collect());
            }
        }
        machine
    }

    fn min_presses_for_lights(&self) -> usize {
        // all lights off at start
        let init = vec![false; self.lights.len()];
        let mut best = usize::MAX;
        let mut queue = VecDeque::new();
        queue.push_back((init.clone(), 0)); // (state, presses)
        let mut visited = HashSet::new();
        visited.insert(init.clone());

        while let Some((state, presses)) = queue.pop_front() {
            if state == self.lights {
                // found solution
                best = best.min(presses);
                continue;
            }
            
            if presses >= best {
                // can't improve
                continue;
            }
            
            for button in self.buttons.iter() {
                let mut next_state = state.clone();
                for &idx in button {
                    next_state[idx] = !next_state[idx];
                }
                
                if visited.insert(next_state.clone()) {
                    queue.push_back((next_state, presses + 1));
                }
            }
        }

        best
    }

    fn min_presses_for_joltage(&self) -> usize {
        // Construct the augmented matrix for the linear system:
        // row - joltage level equation: sum(button_presses * effect) = target_joltage
        // col - button variable, plus one for the constants (target joltages)
        let num_vars = self.buttons.len();
        let num_eqs = self.joltage.len();
        let mut matrix = vec![vec![0.0; num_vars + 1]; num_eqs];

        for (j, button) in self.buttons.iter().enumerate() {
            for &idx in button {
                if idx < num_eqs {
                    matrix[idx][j] = 1.0;
                }
            }
        }

        for (i, &val) in self.joltage.iter().enumerate() {
            matrix[i][num_vars] = val as f64;
        }

        // Solve the system minimizing the sum of button presses (variables)
        crate::utils::solve_min_integer_sum(matrix)
    }
}

#[test]
pub fn test() {
    let input = vec![
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    assert_eq!(a(&input), "7");
    assert_eq!(b(&input), "33");
}
