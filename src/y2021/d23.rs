//! Day 23: Amphipod

use std::collections::{BinaryHeap, HashMap};

/// least total energy to move to target positions
pub fn a(input: &Vec<&str>) -> String {
    let state = parse_input(input);
    let energy = least_total_energy(&state);
    energy.to_string()
}

/// least energy required for extended input
pub fn b(input: &Vec<&str>) -> String {
    let extended_input = vec![
        input[0],
        input[1],
        input[2],
        "  #D#C#B#A#",
        "  #D#B#A#C#",
        input[3],
        input[4],
    ];
    let state = parse_input(&extended_input);
    let energy = least_total_energy(&state);
    energy.to_string()
}

fn parse_input(input: &Vec<&str>) -> State {
    // #############
    // #01.2.3.4.56#
    // ###7#8#9#A###
    //   #B#C#D#E#
    //   #########

    let mut state = State::with_capacity(7 + (input.len() - 3) * 4);

    state.push(input[1][1..2].into());
    for i in 0..5 {
        state.push(input[1][2 + i * 2..3 + i * 2].into());
    }
    state.push(input[1][11..12].into());

    for j in 0..input.len() - 3 {
        for i in 0..4 {
            state.push(input[2 + j][3 + i * 2..4 + i * 2].into());
        }
    }

    state
}

/// checks whether given state is the goal
fn is_target_state(state: &State) -> bool {
    for i in 0..state.len() {
        if i < 7 {
            if state[i] != AmphipodType::None {
                return false;
            }
        } else {
            if state[i] as usize != (i - 7) % 4 {
                return false;
            }
        }
    }
    true
}

/// heuristic for minimum remaining cost to target state
fn estimate_cost(state: &State) -> usize {
    let mut cost = 0;
    for i in 0..state.len() {
        if state[i] == AmphipodType::None {
            continue;  // ignore empty space
        }

        if i >= 7 && (i - 7) % 4 == state[i] as usize {
            continue;  // already in correct room
        }
        
        let adjusted_pos = match i {
            0 => 1,
            1..=5 => i * 2,
            6 => 11,
            _ => ((i - 7) % 4) * 2 + 3,
        };

        // distance to correct room
        cost += adjusted_pos.abs_diff(state[i] as usize * 2 + 3) * ENERGY[state[i] as usize];
    }

    cost
}

fn build_adjacency(size: usize) -> Adjacency {
    assert!(size >= 11);

    // adjacency lists for hallway and room entrances
    let mut adjacency = vec![
        vec![(1, 1)],  // 0
        vec![(0, 1), (2, 2), (7, 2)],  // 1
        vec![(1, 2), (3, 2), (7, 2), (8, 2)],  // 2
        vec![(2, 2), (4, 2), (8, 2), (9, 2)],  // 3
        vec![(3, 2), (5, 2), (9, 2), (10, 2)],  // 4
        vec![(4, 2), (6, 1), (10, 2)],  // 5
        vec![(5, 1)],  // 6
        vec![(1, 2), (2, 2)],  // 7
        vec![(2, 2), (3, 2)],  // 8
        vec![(3, 2), (4, 2)],  // 9
        vec![(4, 2), (5, 2)],  // 10
    ];

    // add depth to rooms
    for i in 11..size {
        adjacency[i - 4].push((i, 1));
        adjacency.push(vec![(i - 4, 1)]);
    }

    adjacency
}

fn generate_transitions(state: &State, adjacency: &Adjacency) -> Vec<Transition> {
    let room_depth = (adjacency.len() - 7) / 4;
    let mut transitions = Vec::<Transition>::new();

    for i in 0..state.len() {
        if state[i] == AmphipodType::None {
            continue;  // ignore empty space
        }

        // index of correct room for this amphipod
        let room = state[i] as usize + 7;

        // collect potential spaces to move to
        let mut candidates = Vec::<usize>::new();
        if i < 7 {
            // in hallway, can only move into correct room
            let mut r = room + 4 * room_depth;
            loop {
                r -= 4;
                if state[r] != state[i] {
                    // lowest room position not occupied by same amphipod type
                    candidates.push(r);
                    break;
                }
                if i == room {
                    break;
                }
            }
        } else if (i - 7) % 4 == state[i] as usize {
            // in correct room, check whether blocking another type below
            let mut blocking = false;
            let mut r = i + 4;
            while r < state.len() && !blocking {
                blocking = state[r] != state[i];
                r += 4;
            }
            if blocking {
                // make space for incorrect type to move out
                candidates.append(&mut (0..7).collect());
            }
        } else {
            // move out of incorrect room
            candidates.append(&mut (0..7).collect());
        }

        if candidates.is_empty() {
            continue;
        }

        let mut unvisited = BinaryHeap::from([GraphNode { vertex: i, cost: 0 }]);
        let mut costs = HashMap::from([(i, 0)]);

        while let Some(node) = unvisited.pop() {
            // let cost = *costs.entry(pos).or_insert(usize::MAX);

            if candidates.contains(&node.vertex) {
                transitions.push(Transition {
                    start: i,
                    end: node.vertex,
                    steps: node.cost,
                })
            }

            for (new_pos, steps) in &adjacency[node.vertex] {
                if state[*new_pos] != AmphipodType::None {
                    continue;
                }
                let new_cost = node.cost + *steps;
                let prev_cost = costs.entry(*new_pos).or_insert(usize::MAX);
                if new_cost < *prev_cost {
                    // found a shorter path to adjacent space
                    *prev_cost = new_cost;
                    unvisited.push(GraphNode {
                        vertex: *new_pos,
                        cost: new_cost,
                    });
                }
            }
        }
    }

    transitions
}

fn least_total_energy(initial_state: &State) -> usize {
    let adjacency = build_adjacency(initial_state.len());
    let mut unvisited = BinaryHeap::from([GraphNode {
        vertex: initial_state.to_owned(),
        cost: estimate_cost(&initial_state),
    }]);
    let mut costs = HashMap::from([(initial_state.to_owned(), 0usize)]);
    let mut search_count = 0usize;

    while let Some(node) = unvisited.pop() {
        search_count += 1;

        let cost = *costs.entry(node.vertex.to_owned()).or_insert(usize::MAX);

        if is_target_state(&node.vertex) {
            // println!("{:?} - {} *", node.vertex, node.cost);
            println!("Searched {search_count} state transitions.");
            return cost;
        }

        let transitions = generate_transitions(&node.vertex, &adjacency);
        for transition in &transitions {
            let new_state = transition.apply_to(&node.vertex);
            let new_cost =
                cost + transition.steps * ENERGY[node.vertex[transition.start] as usize];
            let prev_cost = costs.entry(new_state.to_owned()).or_insert(usize::MAX);
            if new_cost < *prev_cost {
                // found a cheaper path to adjacent state
                *prev_cost = new_cost;
                // println!("{:?} - {}", new_state, new_cost);
                unvisited.push(GraphNode {
                    vertex: new_state.to_owned(),
                    cost: new_cost + estimate_cost(&new_state),
                });
            }
        }
    }

    usize::MAX
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd)]
enum AmphipodType {
    A,
    B,
    C,
    D,
    None,
}

impl Default for AmphipodType {
    fn default() -> Self {
        Self::None
    }
}

impl From<&str> for AmphipodType {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            _ => Self::None,
        }
    }
}

/// step cost multipliers per amphipod
const ENERGY: [usize; 4] = [1, 10, 100, 1000]; // A, B, C, D

/// Representation of game state
type State = Vec<AmphipodType>;

/// adjacent spaces and their transitioning cost
type Adjacency = Vec<Vec<(usize, usize)>>;

#[derive(Eq, PartialEq)]
struct GraphNode<T> {
    vertex: T,
    cost: usize,
}

impl<T: std::cmp::Eq + std::cmp::PartialOrd> Ord for GraphNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<T: std::cmp::Eq + std::cmp::PartialOrd> PartialOrd for GraphNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Transition {
    start: usize,
    end: usize,
    steps: usize,
}

impl Transition {
    fn apply_to(&self, state: &State) -> State {
        let mut new_state = state.to_owned();
        new_state.swap(self.start, self.end);
        new_state
    }
}

#[test]
pub fn test() {
    let input = vec![
        "#############",
        "#...........#",
        "###B#C#B#D###",
        "  #A#D#C#A#",
        "  #########",
    ];

    let debug_input = vec![
        "#############",
        "#...........#",
        "###D#B#C#A###",
        "  #A#B#C#D#",
        "  #########",
    ];

    let debug_input2 = vec![
        "#############",
        "#.....D.D.A.#",
        "###.#B#C#.###",
        "  #A#B#C#.#",
        "  #########",
    ];

    let target_input = vec![
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#",
        "  #########",
    ];

    let target_input_ext = vec![
        "#############",
        "#...........#",
        "###A#B#C#D###",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #A#B#C#D#",
        "  #########",
    ];

    let state = parse_input(&input);
    assert_eq!(state.len(), 15);
    assert_eq!(
        state,
        [
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::None,
            AmphipodType::B,
            AmphipodType::C,
            AmphipodType::B,
            AmphipodType::D,
            AmphipodType::A,
            AmphipodType::D,
            AmphipodType::C,
            AmphipodType::A,
        ]
    );

    assert!(!is_target_state(&state));
    assert!(is_target_state(&parse_input(&target_input)));
    assert!(is_target_state(&parse_input(&target_input_ext)));

    let adjacency = build_adjacency(state.len());
    assert_eq!(adjacency.len(), state.len());
    assert_eq!(generate_transitions(&state, &adjacency).len(), 28);

    assert_eq!(a(&debug_input), "8010");
    assert_eq!(a(&debug_input2), "7008");

    assert_eq!(a(&input), "12521");

    assert_eq!(b(&input), "44169");
}
