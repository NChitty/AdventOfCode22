use std::cmp::{max, min};

#[derive(Clone)]
enum CaveFill {
    AIR,
    ROCK,
    SAND,
}

fn parse_coord(str: &str) -> (usize, usize) {
    let mut parts = str.split(',');
    (
        parts
            .next()
            .expect("Malformed coordinate")
            .parse()
            .expect("Not an integer."),
        parts
            .next()
            .expect("Malformed coordinate")
            .parse()
            .expect("Not an integer."),
    )
}

fn is_obstructed(candidate_coord: (usize, usize), cave: &Vec<Vec<CaveFill>>) -> bool {
    match cave[candidate_coord.1][candidate_coord.0] {
        CaveFill::AIR => false,
        CaveFill::ROCK | CaveFill::SAND => true,
    }
}

#[aoc_generator(day14, part1)]
fn build_cave(input: &str) -> (Vec<Vec<CaveFill>>, usize) {
    let mut cave: Vec<Vec<CaveFill>> = vec![vec![CaveFill::AIR; 1000]; 200];
    let mut y_bound = 0;
    input.lines().for_each(|line| {
        let mut previous: Option<(usize, usize)> = None;
        let mut coords = line.split(" -> ");
        while let Some(coord_str) = coords.next() {
            let coord = parse_coord(coord_str);
            match previous {
                Some(prev_coord) => {
                    if coord.1 > y_bound {
                        y_bound = coord.1;
                    }
                    if prev_coord.0 == coord.0 {
                        for i in min(prev_coord.1, coord.1)..=max(prev_coord.1, coord.1) {
                            cave[i][prev_coord.0] = CaveFill::ROCK;
                        }
                    } else {
                        for i in min(prev_coord.0, coord.0)..=max(prev_coord.0, coord.0) {
                            cave[prev_coord.1][i] = CaveFill::ROCK;
                        }
                    }
                    previous = Some(coord);
                }
                None => {
                    previous = Some(coord);
                    if coord.1 > y_bound {
                        y_bound = coord.1;
                    }
                    continue;
                }
            }
        }
    });
    cave.truncate(y_bound + 1);
    (cave, y_bound)
}

#[aoc_generator(day14, part2)]
fn build_cave_with_floor(input: &str) -> Vec<Vec<CaveFill>> {
    let mut cave: Vec<Vec<CaveFill>> = vec![vec![CaveFill::AIR; 1000]; 200];
    let mut y_bound = 0;
    input.lines().for_each(|line| {
        let mut previous: Option<(usize, usize)> = None;
        let mut coords = line.split(" -> ");
        while let Some(coord_str) = coords.next() {
            let coord = parse_coord(coord_str);
            match previous {
                Some(prev_coord) => {
                    if coord.1 > y_bound {
                        y_bound = coord.1;
                    }
                    if prev_coord.0 == coord.0 {
                        for i in min(prev_coord.1, coord.1)..=max(prev_coord.1, coord.1) {
                            cave[i][prev_coord.0] = CaveFill::ROCK;
                        }
                    } else {
                        for i in min(prev_coord.0, coord.0)..=max(prev_coord.0, coord.0) {
                            cave[prev_coord.1][i] = CaveFill::ROCK;
                        }
                    }
                    previous = Some(coord);
                }
                None => {
                    previous = Some(coord);
                    if coord.1 > y_bound {
                        y_bound = coord.1;
                    }
                    continue;
                }
            }
        }
    });
    cave.truncate(y_bound + 1);
    cave.push(vec![CaveFill::AIR; 1000]);
    cave.push(vec![CaveFill::ROCK; 1000]);
    cave
}

#[aoc(day14, part1)]
fn simulate_falling_sand(input: &(Vec<Vec<CaveFill>>, usize)) -> usize {
    let mut cave = input.0.clone();
    let mut sand_particles = 0;
    let mut fell_off = false;
    loop {
        let mut sand_location = (500, 0);
        let mut is_done_falling = false;
        while !is_done_falling {
            if sand_location.1 + 1 > input.1 {
                fell_off = true;
                break;
            }
            // move down
            if !is_obstructed((sand_location.0, sand_location.1 + 1), &cave) {
                sand_location.1 += 1;
                continue;
            }
            // move left down
            if !is_obstructed((sand_location.0 - 1, sand_location.1 + 1), &cave) {
                sand_location.0 -= 1;
                sand_location.1 += 1;
                continue;
            }
            // move right down
            if !is_obstructed((sand_location.0 + 1, sand_location.1 + 1), &cave) {
                sand_location.0 += 1;
                sand_location.1 += 1;
                continue;
            }
            // rest
            sand_particles += 1;
            is_done_falling = true;
            cave[sand_location.1][sand_location.0] = CaveFill::SAND;
        }
        if fell_off {
            break;
        }
    }
    sand_particles
}

#[aoc(day14, part2)]
fn simulate_falling_sand_to_source(input: &Vec<Vec<CaveFill>>) -> usize {
    let mut cave = input.clone();
    let mut sand_particles = 0;
    let mut is_filled = false;
    loop {
        let mut sand_location = (500, 0);
        let mut is_done_falling = false;
        while !is_done_falling {
            // move down
            if !is_obstructed((sand_location.0, sand_location.1 + 1), &cave) {
                sand_location.1 += 1;
                continue;
            }
            // move left down
            if !is_obstructed((sand_location.0 - 1, sand_location.1 + 1), &cave) {
                sand_location.0 -= 1;
                sand_location.1 += 1;
                continue;
            }
            // move right down
            if !is_obstructed((sand_location.0 + 1, sand_location.1 + 1), &cave) {
                sand_location.0 += 1;
                sand_location.1 += 1;
                continue;
            }
            // rest
            sand_particles += 1;
            if sand_location == (500, 0) {
                is_filled = true;
                break;
            }
            is_done_falling = true;
            cave[sand_location.1][sand_location.0] = CaveFill::SAND;
        }
        if is_filled {
            break;
        }
    }
    sand_particles
}

#[cfg(test)]
mod test {
    use crate::day14::{build_cave_with_floor, simulate_falling_sand_to_source};

    use super::{build_cave, simulate_falling_sand};

    const SAMPLE: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn sample_simulation_part1() {
        let input = build_cave(SAMPLE);
        let expected = 24;
        let actual = simulate_falling_sand(&input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_simulation_part2() {
        let input = build_cave_with_floor(SAMPLE);
        let expected = 93;
        let actual = simulate_falling_sand_to_source(&input);
        assert_eq!(expected, actual);
    }
}
