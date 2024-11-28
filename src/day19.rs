use std::{
    ops::{Add, Mul},
    str::FromStr,
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::u8,
    error::Error,
    sequence::{delimited, separated_pair},
    Finish, IResult, Parser,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Resources {
    ore: u8,
    clay: u8,
    obsidian: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Blueprint {
    id: u8,
    ore_robot: Resources,
    clay_robot: Resources,
    obsidian_robot: Resources,
    geode_robot: Resources,
}

impl FromStr for Blueprint {
    type Err = Error<String>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_blueprint(s).finish() {
            Ok((_remaining, blueprint)) => Ok(blueprint),
            Err(Error { input, code }) => Err(Error {
                input: input.to_string(),
                code,
            }),
        }
    }
}

const ONE_ORE: Resources = Resources {
    ore: 1,
    clay: 0,
    obsidian: 0,
};

const ONE_CLAY: Resources = Resources {
    ore: 0,
    clay: 1,
    obsidian: 0,
};

const ONE_OBSIDIAN: Resources = Resources {
    ore: 0,
    clay: 0,
    obsidian: 1,
};

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, id) = delimited(tag("Blueprint "), u8, tag(": "))(input)?;
    let (input, ore_robot) = delimited(tag("Each ore robot costs "), u8, tag(" ore. "))
        .map(|ore| ONE_ORE * ore)
        .parse(input)?;
    let (input, clay_robot) = delimited(tag("Each clay robot costs "), u8, tag(" ore. "))
        .map(|ore| ONE_ORE * ore)
        .parse(input)?;
    let (input, obsidian_robot) = delimited(
        tag("Each obsidian robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" clay. "),
    )
    .map(|(ore, clay)| ONE_ORE * ore + ONE_CLAY * clay)
    .parse(input)?;
    let (input, geode_robot) = delimited(
        tag("Each geode robot costs "),
        separated_pair(u8, tag(" ore and "), u8),
        tag(" obsidian."),
    )
    .map(|(ore, obsidian)| ONE_ORE * ore + ONE_OBSIDIAN * obsidian)
    .parse(input)?;
    Ok((
        input,
        Blueprint {
            id,
            ore_robot,
            clay_robot,
            obsidian_robot,
            geode_robot,
        },
    ))
}

impl Resources {
    fn checked_sub(self, rhs: Self) -> Option<Self> {
        Some(Self {
            ore: self.ore.checked_sub(rhs.ore)?,
            clay: self.clay.checked_sub(rhs.clay)?,
            obsidian: self.obsidian.checked_sub(rhs.obsidian)?,
        })
    }
}

impl Mul<u8> for Resources {
    type Output = Self;

    fn mul(self, rhs: u8) -> Self::Output {
        Self {
            ore: self.ore * rhs,
            clay: self.clay * rhs,
            obsidian: self.obsidian * rhs,
        }
    }
}

impl Add for Resources {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct State {
    minutes_remaining: u8,
    geodes: u8,
    resources: Resources,
    resources_rate: Resources,
}

impl State {
    fn new(minutes_remaining: u8) -> Self {
        Self {
            minutes_remaining,
            geodes: 0,
            resources: Default::default(),
            resources_rate: ONE_ORE,
        }
    }

    fn try_build_robot(self, cost: Resources, resource_added: Resources) -> Option<Self> {
        (1..self.minutes_remaining).rev().zip(0..).find_map(
            |(minutes_remaining, minutes_passed)| {
                let resources = self.resources + self.resources_rate * minutes_passed;
                resources.checked_sub(cost).map(|resources| Self {
                    minutes_remaining,
                    resources: resources + self.resources_rate,
                    resources_rate: self.resources_rate + resource_added,
                    ..self
                })
            },
        )
    }

    fn branch(self, blueprint: &Blueprint) -> impl Iterator<Item = Self> + '_ {
        let max_ore_cost = blueprint.clay_robot.ore
            .max(blueprint.obsidian_robot.ore)
            .max(blueprint.geode_robot.ore);
        let is_ore_robot_viable = self.resources_rate.ore < max_ore_cost;
        let is_clay_robot_viable = self.resources_rate.clay < blueprint.obsidian_robot.clay;
        let is_obsidian_robot_viable = self.resources_rate.obsidian
            < blueprint.geode_robot.obsidian
            && self.resources_rate.clay > 0;
        let is_geode_robot_viable = self.resources_rate.obsidian > 0;
        [
            is_ore_robot_viable.then(|| self.try_build_robot(blueprint.ore_robot, ONE_ORE)),
            is_clay_robot_viable.then(|| self.try_build_robot(blueprint.clay_robot, ONE_CLAY)),
            is_obsidian_robot_viable
                .then(|| self.try_build_robot(blueprint.obsidian_robot, ONE_OBSIDIAN)),
            is_geode_robot_viable.then(|| {
                self.try_build_robot(blueprint.geode_robot, Default::default())
                    .map(|state| Self {
                        geodes: state.geodes + state.minutes_remaining,
                        ..state
                    })
            }),
        ]
        .into_iter()
        .flatten()
        .flatten()
    }

    fn bound(self, blueprint: &Blueprint) -> u8 {
        let geode_cost = blueprint.geode_robot.obsidian;
        let (_, _, geodes) = (0..self.minutes_remaining).rev().fold(
            (
                self.resources.obsidian,
                self.resources_rate.obsidian,
                self.geodes,
            ),
            |(obsidian, rate, geodes), minutes_remaining| {
                if obsidian >= geode_cost {
                    (
                        obsidian + rate,
                        rate,
                        geodes.saturating_add(minutes_remaining),
                    )
                } else {
                    (obsidian + rate, rate + 1, geodes)
                }
            },
        );
        geodes
    }
}

fn branch_and_bound(blueprint: &Blueprint, state: State, best: &mut u8) {
    *best = state.geodes.max(*best);
    for state in state.branch(blueprint) {
        if state.bound(blueprint) > *best {
            branch_and_bound(blueprint, state, best);
        }
    }
}

#[aoc_generator(day19)]
fn get_blueprints(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .filter_map(|line| Blueprint::from_str(line).ok())
        .collect_vec()
}

#[aoc(day19, part1)]
fn solve_a(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(24), &mut best);
            blueprint.id as usize * best as usize
        })
        .sum()
}

#[aoc(day19, part2)]
fn solve_b(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| {
            let mut best = 0;
            branch_and_bound(blueprint, State::new(32), &mut best);
            best as usize
        })
        .product()
}

#[cfg(test)]
mod test {
    use crate::day19::solve_b;

    use super::{get_blueprints, solve_a};

    const SAMPLE_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn sample_a() {
        let blueprints = get_blueprints(SAMPLE_INPUT);
        let expected = 33;
        let actual = solve_a(&blueprints);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_b() {
        let blueprints = get_blueprints(SAMPLE_INPUT);
        let expected = 56 * 62;
        let actual = solve_b(&blueprints);
        assert_eq!(expected, actual);
    }
}
