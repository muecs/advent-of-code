//! Day 6: Lanternfish

/// population size after 80 days
pub fn a(input: &Vec<&str>) -> String {
    let mut population = parse_input(input);
    simulate(&mut population, 80);
    census(&population).to_string()
}

/// population size after 256 days
pub fn b(input: &Vec<&str>) -> String {
    let mut population = parse_input(input);
    simulate(&mut population, 256);
    census(&population).to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<usize> {
    let mut population = vec![0usize; 9];
    input
        .first()
        .unwrap()
        .split(',')
        .map(|x| x.parse::<usize>().unwrap())
        .for_each(|x| population[x] += 1);
    population
}

// count down from 6 to 0, on 0 spawn new instance (timer 8) and reset to 6
fn simulate(population: &mut Vec<usize>, days: usize) {
    for _ in 0..days {
        let spawn = population[0];
        for i in 1..9 {
            population[i - 1] = population[i];
        }
        population[6] += spawn;
        population[8] = spawn;
    }
}

fn census(population: &Vec<usize>) -> usize {
    population.iter().sum()
}

#[test]
pub fn test() {
    let input = vec!["3,4,3,1,2"];

    let mut population = parse_input(&input);
    assert_eq!(population, vec![0usize, 1, 1, 2, 1, 0, 0, 0, 0]);
    assert_eq!(census(&population), 5);

    simulate(&mut population, 1);
    assert_eq!(population, vec![1usize, 1, 2, 1, 0, 0, 0, 0, 0]);
    assert_eq!(census(&population), 5);

    simulate(&mut population, 1);
    assert_eq!(population, vec![1usize, 2, 1, 0, 0, 0, 1, 0, 1]);
    assert_eq!(census(&population), 6);

    simulate(&mut population, 1);
    assert_eq!(population, vec![2usize, 1, 0, 0, 0, 1, 1, 1, 1]);
    assert_eq!(census(&population), 7);
    
    simulate(&mut population, 15);
    assert_eq!(census(&population), 26);

    assert_eq!(a(&input), "5934");
    assert_eq!(b(&input), "26984457539");
}
