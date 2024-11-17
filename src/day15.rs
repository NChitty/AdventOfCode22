use std::{isize, ops::Range, str::FromStr};

use itertools::Itertools;
use regex::Regex;

const LOCATION_REGEX: &str = "Sensor at x=(?<sensor_x>[-\\d]*?), y=(?<sensor_y>[-\\d]*?): closest beacon is at x=(?<beacon_x>[-\\d]*?), y=(?<beacon_y>[-\\d]*?)$";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn manhattan_distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    sensor: Point,
    beacon: Point,
}

impl FromStr for Pair {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matcher = Regex::new(LOCATION_REGEX).map_err(|_| "Could not build regex.")?;
        let captures = matcher.captures(s).ok_or("No matches found.")?;
        let sensor: Point = Point {
            x: captures
                .name("sensor_x")
                .ok_or("No match for sensor x position")?
                .as_str()
                .parse()
                .map_err(|_| "Could not parse sensor x as integer")?,
            y: captures
                .name("sensor_y")
                .ok_or("No match for sensor y position")?
                .as_str()
                .parse()
                .map_err(|_| "Could not parse sensor y as integer")?,
        };
        let beacon: Point = Point {
            x: captures
                .name("beacon_x")
                .ok_or("No match for beacon x position")?
                .as_str()
                .parse()
                .map_err(|_| "Could not parse beacon x as integer")?,
            y: captures
                .name("beacon_y")
                .ok_or("No match for beacon y position")?
                .as_str()
                .parse()
                .map_err(|_| "Could not parse beacon y as integer")?,
        };
        Ok(Self { sensor, beacon })
    }
}

impl Pair {
    fn covered_distance(&self) -> usize {
        self.sensor.manhattan_distance(&self.beacon)
    }

    fn is_covered(&self, location: &Point) -> bool {
        self.sensor.manhattan_distance(location) <= self.covered_distance()
    }

    fn get_covered_ranges_in_row(&self, row: isize) -> Option<Range<isize>> {
        let x_offset = self.covered_distance() as isize - (self.sensor.y - row).abs();
        Some((self.sensor.x - x_offset)..(self.sensor.x + x_offset + 1)).filter(|r| !r.is_empty())
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| line.parse().expect("Could not parse line"))
        .collect()
}

fn solve(pairs: &[Pair], row: isize) -> usize {
    let covered_in_row: usize = pairs
        .iter()
        .flat_map(|pair| pair.get_covered_ranges_in_row(row))
        .sorted_unstable_by_key(|range| range.start)
        .coalesce(|a, b| {
            if a.end >= b.start {
                Ok(a.start..b.end.max(a.end))
            } else {
                Err((a, b))
            }
        })
        .map(|xs| xs.len())
        .sum();
    let blocked_xs = pairs
        .into_iter()
        .flat_map(|pair| [pair.sensor, pair.beacon])
        .filter(|p| p.y == row)
        .unique()
        .count();
    covered_in_row - blocked_xs
}

#[aoc(day15, part1)]
fn solve_part1(input: &[Pair]) -> usize {
    solve(input, 2_000_000)
}

#[cfg(test)]
mod test {
    use super::{parse_input, solve};

    const SAMPLE_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn sample_part1() {
        let input = parse_input(SAMPLE_INPUT);
        let expected = 26;
        let actual = solve(&input, 10);
        assert_eq!(expected, actual);
    }
}
