use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    str::FromStr,
    usize,
};

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Valve {
    name: String,
    rate: usize,
    adjacent: Vec<String>,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(";").collect();
        if parts.len() != 2 {
            return Err("No semicolon to split".to_string());
        }

        // Parse valve name and flow rate
        let first_part_parts: Vec<&str> = parts[0].split_whitespace().collect();
        if first_part_parts.len() < 5 {
            return Err("There are not enough tokens.".to_string());
        }
        let name = first_part_parts[1].to_string();
        let rate_str = first_part_parts[4];
        let rate = rate_str
            .split('=')
            .nth(1)
            .ok_or("Could not get rate")?
            .parse::<usize>()
            .map_err(|_| "Could not parse rate to usize")?;

        // Parse adjacent valves
        let second_part = parts[1];
        let adjacent_str = if second_part.contains("valves") {
            second_part.split("valves ").nth(1)
        } else {
            second_part.split("valve ").nth(1)
        }
        .ok_or("No second part for adjacent")?;

        let adjacent: Vec<String> = adjacent_str
            .split(", ")
            .map(|s| s.trim().to_string())
            .collect();

        Ok(Valve {
            name,
            rate,
            adjacent,
        })
    }
}

#[derive(Clone, Copy, Debug)]
struct State {
    current_valve: usize,
    minutes_remaining: usize,
    pressure_released: usize,
    avoid: usize,
    visited: usize,
}

impl State {
    fn new(position: usize, minutes_remaing: usize) -> Self {
        Self {
            current_valve: position,
            minutes_remaining: minutes_remaing,
            pressure_released: 0,
            avoid: 1 << position,
            visited: 0,
        }
    }

    fn can_visit(&self, valve_idx: usize) -> bool {
        (self.visited | self.avoid) & (1 << valve_idx) == 0
    }

    fn bound(self, flow_rates: &[usize], sorted_flow_rate_indices: &[usize]) -> usize {
        self.pressure_released
            + (0..=self.minutes_remaining)
                .rev()
                .step_by(2)
                .skip(1)
                .zip(
                    sorted_flow_rate_indices
                        .iter()
                        .filter(|&&i| self.can_visit(i))
                        .map(|&i| flow_rates[i]),
                )
                .map(|(minutes, flow)| minutes * flow)
                .sum::<usize>()
    }

    fn branch<'a>(
        self,
        flow_rates: &'a Vec<usize>,
        shortest_path_lengths: &'a Vec<Vec<u8>>,
    ) -> impl IntoIterator<Item = Self> + 'a {
        shortest_path_lengths[self.current_valve]
            .iter()
            .enumerate()
            .filter(move |&(destination, _distance)| self.can_visit(destination))
            .filter_map(move |(destination, distance)| {
                let minutes_remaining =
                    self.minutes_remaining.checked_sub(*distance as usize + 1)?;
                Some(State {
                    visited: self.visited | (1 << destination),
                    avoid: self.avoid,
                    pressure_released: self.pressure_released
                        + minutes_remaining * flow_rates[destination],
                    minutes_remaining,
                    current_valve: destination,
                })
            })
    }
}

fn floyd_warshall(valves: &[Valve]) -> Vec<Vec<u8>> {
    let valve_name_to_idx: HashMap<String, _> = valves
        .iter()
        .enumerate()
        .map(|(i, ref valve)| (valve.name.to_owned(), i))
        .collect();
    let mut dist: Vec<Vec<u8>> = vec![vec![u8::MAX; valves.len()]; valves.len()];
    for (i, valve) in valves.iter().enumerate() {
        for tunnel in valve.adjacent.clone() {
            let j = valve_name_to_idx[&tunnel];
            dist[i][j] = 1;
        }
    }
    (0..valves.len()).for_each(|i| {
        dist[i][i] = 0;
    });
    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                let (result, overflow) = dist[i][k].overflowing_add(dist[k][j]);
                if !overflow {
                    dist[i][j] = dist[i][j].min(result);
                }
            }
        }
    }
    dist
}

fn branch_and_bound(
    flow_rates: &Vec<usize>,
    sorted_flow_rate_indices: &[usize],
    shortest_path_lengths: &Vec<Vec<u8>>,
    state: State,
    best_for_visited: &mut [usize],
    best: &mut usize,
    filter_bound: impl Fn(usize, usize) -> bool + Copy,
) {
    if let Some(cur_best) = best_for_visited.get_mut(state.visited) {
        *cur_best = state.pressure_released.max(*cur_best);
    }
    *best = state.pressure_released.max(*best);
    let bound_branch_pairs = state
        .branch(flow_rates, shortest_path_lengths)
        .into_iter()
        .map(|state| (state.bound(flow_rates, sorted_flow_rate_indices), state))
        .filter(|&(bound, _)| filter_bound(bound, *best))
        .sorted_unstable_by_key(|(bound, _)| Reverse(*bound))
        .collect_vec();
    for (bound, branch) in bound_branch_pairs {
        if filter_bound(bound, *best) {
            branch_and_bound(
                flow_rates,
                sorted_flow_rate_indices,
                shortest_path_lengths,
                branch,
                best_for_visited,
                best,
                filter_bound,
            );
        }
    }
}

#[aoc_generator(day16)]
fn parse_valves(input: &str) -> (Vec<usize>, Vec<Vec<u8>>, Vec<usize>, usize) {
    let valves: Vec<Valve> = input
        .lines()
        .filter_map(|line| Valve::from_str(line).ok())
        .collect();

    let adjacency_matrix = floyd_warshall(&valves);
    let notable_valves = valves
        .iter()
        .enumerate()
        .filter(|&(_, valve)| valve.name == "AA" || valve.rate > 0)
        .map(|(i, _)| i)
        .collect_vec();
    let flow_rates = notable_valves.iter().map(|&i| valves[i].rate).collect_vec();
    let shortest_lengths: Vec<Vec<_>> = notable_valves
        .iter()
        .map(|&i| {
            notable_valves
                .iter()
                .map(|&j| adjacency_matrix[i][j])
                .collect()
        })
        .collect();
    let starting_node = notable_valves
        .iter()
        .position(|&i| valves[i].name == "AA")
        .expect("No valve labeled AA");
    let sorted_flow_rate_indices = flow_rates
        .iter()
        .enumerate()
        .sorted_unstable_by_key(|&(_, &flow)| Reverse(flow))
        .map(|(i, _)| i)
        .collect_vec();
    (
        flow_rates,
        shortest_lengths,
        sorted_flow_rate_indices,
        starting_node,
    )
}

#[aoc(day16, part1)]
fn part1_branch_and_bound(input: &(Vec<usize>, Vec<Vec<u8>>, Vec<usize>, usize)) -> usize {
    let state = State::new(input.3, 30);
    let mut best = 0;
    branch_and_bound(
        &input.0,
        &input.2,
        &input.1,
        state,
        &mut [],
        &mut best,
        |bound, best| bound > best,
    );
    best
}

#[aoc(day16, part2)]
fn part2_branch_and_bound(input: &(Vec<usize>, Vec<Vec<u8>>, Vec<usize>, usize)) -> usize {
    let mut best_per_visited = vec![0; u16::MAX as usize];
    branch_and_bound(
        &input.0,
        &input.2,
        &input.1,
        State::new(input.3, 26),
        &mut best_per_visited,
        &mut 0,
        |bound, best| bound > best * 1 / 4,
    );
    let best_per_visited_filtered_sorted = best_per_visited
        .into_iter()
        .enumerate()
        .filter(|&(_, best)| best > 0)
        .map(|(i, best)| (i as u16, best))
        .sorted_unstable_by_key(|&(_, best)| Reverse(best))
        .collect_vec();
    let mut best = 0;
    for (i, &(my_visited, my_best)) in best_per_visited_filtered_sorted.iter().enumerate() {
        for &(elephant_visited, elephant_best) in &best_per_visited_filtered_sorted[i + 1..] {
            let score = my_best + elephant_best;
            if score <= best {
                break;
            }
            if my_visited & elephant_visited == 0 {
                best = score;
                break;
            }
        }
    }
    best
}

#[cfg(test)]
mod test {
    use crate::day16::{part1_branch_and_bound, part2_branch_and_bound};

    use super::{parse_valves, Valve};

    const SAMPLE_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn aa_from_str() {
        let expected = Valve {
            name: "AA".to_string(),
            rate: 0,
            adjacent: vec!["DD".to_string(), "II".to_string(), "BB".to_string()],
        };
        let actual = SAMPLE_INPUT
            .lines()
            .next()
            .expect("No lines")
            .parse()
            .expect("Could not parse line.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_part1() {
        let map = parse_valves(SAMPLE_INPUT);
        let expected = 1651;
        let actual = part1_branch_and_bound(&map);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_part2() {
        let map = parse_valves(SAMPLE_INPUT);
        let expected = 1707;
        let actual = part2_branch_and_bound(&map);
        assert_eq!(expected, actual);
    }
}
