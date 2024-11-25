use std::{collections::HashMap, str::FromStr, usize};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Valve {
    name: String,
    rate: usize,
    adjacent: Vec<String>,
}

#[derive(Clone, Debug)]
struct State {
    current_valve: String,
    minutes_remaining: usize,
    current_rate: usize,
    released: usize,
    opened: Vec<String>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            current_valve: "AA".to_string(),
            minutes_remaining: 30,
            current_rate: 0,
            released: 0,
            opened: Vec::new(),
        }
    }
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

fn initial_adjacency_matrix(map: &HashMap<String, Valve>) -> HashMap<(String, String), usize> {
    let mut adjacency_matrix = HashMap::new();
    map.keys().for_each(|key| {
        map[key].adjacent.iter().for_each(|adjacent_valve| {
            adjacency_matrix.insert((key.clone(), adjacent_valve.clone()), 1);
        })
    });
    adjacency_matrix
}

fn fill_matrix(
    map: &HashMap<String, Valve>,
    adjacency_matrix: &mut HashMap<(String, String), usize>,
) {
    for k in map.keys() {
        for i in map.keys() {
            for j in map.keys() {
                if i != j {
                    let dist_i_j = adjacency_matrix
                        .get(&(i.to_string(), k.to_string()))
                        .unwrap_or(&usize::MAX)
                        .clone();
                    let dist_j_k = adjacency_matrix
                        .get(&(k.to_string(), j.to_string()))
                        .unwrap_or(&usize::MAX)
                        .clone();
                    adjacency_matrix
                        .entry((i.to_string(), j.to_string()))
                        .and_modify(|e| *e = (*e).min(dist_i_j.saturating_add(dist_j_k)))
                        .or_insert(usize::MAX);
                }
            }
        }
    }
}

fn brute_search(
    state: &State,
    map: &HashMap<String, Valve>,
    adjacency_matrix: &HashMap<(String, String), usize>,
) -> usize {
    let mut stack = Vec::new();
    let mut max_released = 0;

    stack.push(state.clone());
    while let Some(mut current_state) = stack.pop() {
        if current_state.minutes_remaining == 0 {
            println!("State expired: {}, max: {}", current_state.opened.join(", "), current_state.released);
            max_released = max_released.max(current_state.released);
            continue;
        }

        if !current_state.opened.contains(&current_state.current_valve)
            && map
                .get(&current_state.current_valve)
                .expect("Could not find current_valve")
                .rate
                != 0
        {
            current_state.minutes_remaining -= 1;
            current_state.released += current_state.current_rate;

            let mut opened_state = current_state.clone();
            opened_state
                .opened
                .push(current_state.current_valve.clone());
            opened_state.current_rate += map
                .get(&current_state.current_valve)
                .expect("Could not find current_valve")
                .rate;
            stack.push(opened_state);
        }

        adjacency_matrix
            .keys()
            .filter(|(valve, to)| {
                *valve == current_state.current_valve
                    && map.get(to).expect("Could not find destination valve").rate > 0
                    && !current_state.opened.contains(to)
                    && current_state.minutes_remaining.saturating_sub(
                        *adjacency_matrix
                            .get(&(valve.to_string(), to.to_string()))
                            .expect("No entry"),
                    ) > 1
            })
            .for_each(|entry| {
                let mut next_state = current_state.clone();
                next_state.minutes_remaining -=
                    adjacency_matrix.get(entry).expect("Could not find entry");
                next_state.released += next_state.current_rate
                    * adjacency_matrix.get(entry).expect("Could not find entry");
                next_state.current_valve = entry.1.clone();
                stack.push(next_state);
            });
    }
    max_released
}

#[aoc_generator(day16)]
fn parse_valves(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .filter_map(|line| Valve::from_str(line).ok())
        .map(move |valve| (valve.name.clone(), valve))
        .collect()
}

#[aoc(day16, part1)]
fn part1_greedy(input: &HashMap<String, Valve>) -> usize {
    let state = State::default();
    let mut adjacency_matrix = initial_adjacency_matrix(&input);
    fill_matrix(&input, &mut adjacency_matrix);
    brute_search(&state, &input, &adjacency_matrix)
}

#[cfg(test)]
mod test {
    use crate::day16::part1_greedy;

    use super::{fill_matrix, initial_adjacency_matrix, parse_valves, Valve};

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
    fn sample_brute() {
      let map = parse_valves(SAMPLE_INPUT);
      let expected = 1651;
      let actual = part1_greedy(&map);
      assert_eq!(expected, actual);
    }
}
