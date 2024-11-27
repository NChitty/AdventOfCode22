enum Direction {
    Right,
    Left,
}

use core::panic;
use std::{collections::HashMap, usize};

use itertools::Itertools;

fn rock_generator() -> impl Iterator<Item = Vec<u8>> {
    let rocks = vec![
        vec![0x1e],             // -
        vec![0x08, 0x1c, 0x08], // +
        vec![0x04, 0x04, 0x1c], // _|
        vec![0x10; 4],          // |
        vec![0x18, 0x18],       // []
    ];
    rocks.into_iter().cycle()
}

fn simulate(directions: &[Direction], num_rocks: usize) -> Vec<u8> {
    let mut chamber: Vec<u8> = vec![];
    let mut air_directions = directions.iter().cycle();
    for mut rock in rock_generator().take(num_rocks) {
        let mut rock_top = chamber.len() + 3 + rock.len();
        loop {
            rock_top -= 1;
            let direction = air_directions.next().unwrap();
            let is_pushed = rock
                .iter()
                .enumerate()
                .map(|(height, row)| (row, chamber.get(rock_top - height)))
                .all(|(row, chamber_row)| {
                    // check room on left or right
                    if match direction {
                        Direction::Right => 1 & row != 0,
                        Direction::Left => 0x40 & row != 0,
                    } {
                        return false;
                    }
                    let Some(chamber_row) = chamber_row else {
                        return true;
                    };
                    // check chamber collision
                    match direction {
                        Direction::Right => (row >> 1) & chamber_row == 0,
                        Direction::Left => (row << 1) & chamber_row == 0,
                    }
                });
            if is_pushed {
                rock.iter_mut().for_each(|row| match direction {
                    Direction::Right => *row >>= 1,
                    Direction::Left => *row <<= 1,
                });
            }
            // hit the floor
            if rock_top < rock.len() {
                break;
            }
            let is_falling = rock.iter().enumerate().all(|(row_idx, row)| {
                if let Some(chamber_row) = chamber.get(rock_top - row_idx - 1) {
                    row & chamber_row == 0
                } else {
                    true
                }
            });
            if !is_falling {
                break;
            }
        }
        chamber.resize(chamber.len().max(rock_top + 1), 0);
        for (row_idx, row) in rock.into_iter().enumerate() {
            chamber[rock_top - row_idx] |= row;
        }
    }
    chamber
}

fn simulate_pattern_recognition(directions: &[Direction], target: usize) -> usize {
    let mut rocks = 0;
    let mut jets = 0;
    let mut top = 0;
    let mut added_by_repeats = 0;
    let mut seen: HashMap<(usize, usize), (usize, usize, usize)> = HashMap::new();
    let mut chamber: Vec<u8> = vec![];
    let mut air_directions = directions.iter().cycle();
    let mut rock_iter = rock_generator();
    while rocks != target {
        let mut rock = rock_iter.next().expect("Must be a rock");
        let mut rock_top = chamber.len() + 3 + rock.len();
        loop {
            rock_top -= 1;
            let direction = air_directions.next().unwrap();
            jets += 1;
            let is_pushed = rock
                .iter()
                .enumerate()
                .map(|(height, row)| (row, chamber.get(rock_top - height)))
                .all(|(row, chamber_row)| {
                    // check room on left or right
                    if match direction {
                        Direction::Right => 1 & row != 0,
                        Direction::Left => 0x40 & row != 0,
                    } {
                        return false;
                    }
                    let Some(chamber_row) = chamber_row else {
                        return true;
                    };
                    // check chamber collision
                    match direction {
                        Direction::Right => (row >> 1) & chamber_row == 0,
                        Direction::Left => (row << 1) & chamber_row == 0,
                    }
                });
            if is_pushed {
                rock.iter_mut().for_each(|row| match direction {
                    Direction::Right => *row >>= 1,
                    Direction::Left => *row <<= 1,
                });
            }
            // hit the floor
            if rock_top < rock.len() {
                break;
            }
            let is_falling = rock.iter().enumerate().all(|(row_idx, row)| {
                if let Some(chamber_row) = chamber.get(rock_top - row_idx - 1) {
                    row & chamber_row == 0
                } else {
                    true
                }
            });
            if !is_falling {
                break;
            }
        }
        chamber.resize(chamber.len().max(rock_top + 1), 0);
        for (row_idx, row) in rock.into_iter().enumerate() {
            chamber[rock_top - row_idx] |= row;
        }
        top = top.max(chamber.len());
        // look for cycle
        if added_by_repeats == 0 {
            let key = (rocks % 5, jets % directions.len());
            if let Some((2, num_rocks_old, old_top)) = seen.get(&key) {
                let delta_top = top - old_top;
                let delta_rocks = rocks - num_rocks_old;
                let repeats = (target - rocks) / delta_rocks;
                added_by_repeats += repeats * delta_top;
                rocks += repeats * delta_rocks;
            }
            seen.entry(key)
                .and_modify(|(amnt, old_num_rocks, old_top)| {
                    *amnt += 1;
                    *old_num_rocks = rocks;
                    *old_top = top;
                }).or_insert((1, rocks, top));
        }
        rocks += 1;
    }
    top + added_by_repeats
}

#[allow(dead_code)]
fn print_chamber(chamber: &[u8]) {
    chamber.iter().rev().for_each(|row| {
        println!("|{:07b}|", row);
    });
    println!("+-------+");
}

#[aoc_generator(day17)]
fn parse_directions(input: &str) -> Vec<Direction> {
    input
        .chars()
        .map(|char| match char {
            '>' => Direction::Right,
            '<' => Direction::Left,
            _ => panic!("Not an allowed direction."),
        })
        .collect_vec()
}

#[aoc(day17, part1)]
fn simulate_2022(directions: &[Direction]) -> usize {
    let chamber = simulate(directions, 2022);
    chamber.len()
}

#[aoc(day17, part2)]
fn simulate_trillion(directions: &[Direction]) -> usize {
    simulate_pattern_recognition(directions, 1_000_000_000_000)
}

#[cfg(test)]
mod test {

    use crate::day17::simulate_trillion;

    use super::{parse_directions, simulate_2022};
    const SAMPLE_DIRECTIONS: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn sample_part1() {
        let directions = parse_directions(SAMPLE_DIRECTIONS);
        let expected = 3068;
        let actual = simulate_2022(&directions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_part2() {
        let directions = parse_directions(SAMPLE_DIRECTIONS);
        let expected = 1_514_285_714_288;
        let actual = simulate_trillion(&directions);
        assert_eq!(expected, actual);
    }
}
