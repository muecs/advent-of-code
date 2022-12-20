//! Day 19: Not Enough Minerals

use std::str::FromStr;

/// sum of products of blueprint ID and max number of geodes in 24 mins
pub fn a(input: &Vec<&str>) -> String {
    let blueprints = parse_input(input);
    blueprints
        .iter()
        .enumerate()
        .map(|(i, b)| (i + 1) * simulate(b, 24))
        .sum::<usize>()
        .to_string()
}

/// product of max number of geodes of first 3 blueprints in 32 mins
pub fn b(input: &Vec<&str>) -> String {
    let blueprints = parse_input(input);
    blueprints
        .iter()
        .take(3)
        .map(|b| simulate(b, 32))
        .product::<usize>()
        .to_string()
}

fn parse_input(input: &Vec<&str>) -> Vec<Blueprint> {
    input
        .iter()
        .map(|line| {
            line.split_once(": ")
                .unwrap()
                .1
                .strip_suffix('.')
                .unwrap()
                .split(". ")
                .map(|robot| {
                    let mut it = robot[5..].splitn(4, ' ');
                    let robot_type: ResourceType = it.next().unwrap().parse().unwrap();
                    it.next();
                    it.next();
                    let robot_costs = it
                        .next()
                        .unwrap()
                        .split(" and ")
                        .map(|cost| {
                            cost.split_once(' ')
                                .map(|(n, res)| {
                                    (
                                        res.parse::<ResourceType>().unwrap(),
                                        n.parse::<u8>().unwrap(),
                                    )
                                })
                                .unwrap()
                        })
                        .fold(Resources::default(), |mut acc, (res, n)| {
                            acc[res as usize] = n;
                            acc
                        });
                    (robot_type, robot_costs)
                })
                .fold(
                    Blueprint::default(),
                    |mut acc, (robot_type, robot_costs)| {
                        acc[robot_type as usize] = robot_costs;
                        acc
                    },
                )
        })
        .collect::<Vec<_>>()
}

fn simulate(blueprint: &Blueprint, steps: u8) -> usize {
    fn traverse(
        blueprint: &Blueprint,
        limits: &Resources,
        mut robots: Resources,
        mut materials: Resources,
        construct: Option<ResourceType>,
        mut skip: u8,
        remaining: u8,
        benchmark: &mut u8,
    ) -> u8 {
        // pay for scheduled construction
        if let Some(robot_type) = construct {
            for (res, cost) in blueprint[robot_type as usize].iter().enumerate() {
                materials[res] -= cost;
            }
        }

        // harvest materials
        for (robot_type, count) in robots.iter().enumerate() {
            materials[robot_type] = materials[robot_type].saturating_add(*count);
        }

        // finish construction
        if let Some(robot_type) = construct {
            robots[robot_type as usize] += 1;
        }

        // println!("[t-{remaining}] Robots: {robots:?}, Materials: {materials:?}");

        // keep track of lower bound for harvestable geodes
        let geodes = materials[ResourceType::Geode as usize];
        if geodes > *benchmark {
            *benchmark = geodes;
        }

        if remaining == 0 {
            return geodes;
        } else {
            let geode_robots = robots[ResourceType::Geode as usize] as u16;
            let potential =
                geodes as u16 + remaining as u16 * (geode_robots + ((remaining as f32 - 1.0) / 2.0).ceil() as u16);
            if potential < *benchmark as u16 {
                return 0; // can no longer reach the current geode maximum
            }
        }

        // generate robot construction options
        let robot_options = blueprint
            .iter()
            .enumerate()
            .filter_map(|(robot_type, robot_costs)| {
                ((robot_type == ResourceType::Geode as usize
                    || robots[robot_type] < limits[robot_type])
                    && skip & (1u8 << robot_type) == 0
                    && robot_costs
                        .iter()
                        .enumerate()
                        .all(|(res, cost)| materials[res] >= *cost))
                .then(|| robot_type)
            })
            .collect::<Vec<_>>();

        // add construction options to skip mask
        skip = robot_options
            .iter()
            .map(|robot_type| 1u8 << *robot_type)
            .fold(skip, |acc, mask| acc | mask);

        // traverse construction options
        robot_options
            .iter()
            .map(|robot_type| Some(ResourceType::from(robot_type)))
            .chain([None].into_iter())
            .map(|construct| {
                traverse(
                    blueprint,
                    limits,
                    robots.to_owned(),
                    materials.to_owned(),
                    construct,
                    if construct.is_none() {
                        // don't attempt to build later what we could have built now
                        skip
                    } else {
                        0
                    },
                    remaining - 1,
                    benchmark,
                )
            })
            .max()
            .unwrap()
    }

    let mut limits = Resources::default();
    for costs in blueprint {
        for (res, cost) in costs.iter().enumerate() {
            if *cost > limits[res] {
                limits[res] = *cost;
            }
        }
    }

    let mut benchmark = 0;
    traverse(
        blueprint,
        &limits,
        [1, 0, 0, 0],
        Resources::default(),
        None,
        0,
        steps - 1,
        &mut benchmark,
    ) as usize
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum ResourceType {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl From<&usize> for ResourceType {
    fn from(val: &usize) -> Self {
        match *val {
            0 => ResourceType::Ore,
            1 => ResourceType::Clay,
            2 => ResourceType::Obsidian,
            3 => ResourceType::Geode,
            _ => unreachable!("invalid resource type index"),
        }
    }
}

impl FromStr for ResourceType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ore" => Ok(ResourceType::Ore),
            "clay" => Ok(ResourceType::Clay),
            "obsidian" => Ok(ResourceType::Obsidian),
            "geode" => Ok(ResourceType::Geode),
            _ => Err("invalid resource type"),
        }
    }
}

type Resources = [u8; 4];
type Blueprint = [Resources; 4];

#[test]
#[ignore = "slow"] // 2.5s
pub fn test() {
    let input = vec![
        "Blueprint 1: \
          Each ore robot costs 4 ore. \
          Each clay robot costs 2 ore. \
          Each obsidian robot costs 3 ore and 14 clay. \
          Each geode robot costs 2 ore and 7 obsidian.",
        "Blueprint 2: \
          Each ore robot costs 2 ore. \
          Each clay robot costs 3 ore. \
          Each obsidian robot costs 3 ore and 8 clay. \
          Each geode robot costs 3 ore and 12 obsidian.",
    ];

    let blueprints = parse_input(&input);
    assert_eq!(blueprints.len(), 2);
    assert_eq!(simulate(&blueprints[0], 19), 1);

    assert_eq!(a(&input), "33");
    assert_eq!(b(&input), "3472");
}
