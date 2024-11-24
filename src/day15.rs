use std::{
    collections::HashMap,
    isize,
    ops::{Neg, Range},
    str::FromStr,
};

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

    /// Generates a line with positive slope from the (min x, y) to (x, max y)
    fn min_x_to_max_y(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.covered_distance() as isize - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.covered_distance() as isize + 1,
            },
        }
    }

    /// Generates a line with negative slope from the (min x, y) to (x, min y)
    fn min_x_to_min_y(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x - self.covered_distance() as isize - 1,
                y: self.sensor.y,
            },
            end: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.covered_distance() as isize - 1,
            },
        }
    }

    /// Generates a line with negative slope from the (x, max y) to (max x, min y)
    fn max_y_to_max_x(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y + self.covered_distance() as isize + 1,
            },
            end: Point {
                x: self.sensor.x + self.covered_distance() as isize + 1,
                y: self.sensor.y,
            },
        }
    }

    /// Generates a line with positive slope from the (x, min y) to (max x, y)
    fn min_y_to_max_x(&self) -> Line {
        Line {
            start: Point {
                x: self.sensor.x,
                y: self.sensor.y - self.covered_distance() as isize - 1,
            },
            end: Point {
                x: self.sensor.x + self.covered_distance() as isize + 1,
                y: self.sensor.y,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn slope(&self) -> isize {
        (self.end.y - self.start.y) / (self.end.x - self.start.x)
    }

    fn y_intercept(&self) -> isize {
        self.slope() * self.start.x.neg() + self.start.y
    }

    fn y(&self, x: isize) -> isize {
        self.slope() * x + self.y_intercept()
    }

    fn extend(&self, other: &Line) -> Option<Line> {
        if self.slope() != other.slope() {
            return None;
        }
        if self.y_intercept() != other.y_intercept() {
            return None;
        }

        let x = self.start.x.min(other.start.x);
        let y = self.y(x);
        let start = Point { x, y };

        let x = self.end.x.max(other.end.x);
        let y = self.y(x);
        let end = Point { x, y };

        Some(Line { start, end })
    }

    fn intersection(&self, other: &Line) -> Option<Point> {
        if self.slope() == other.slope() {
            return None;
        }

        let y_diff = other.y_intercept() - self.y_intercept();
        let slope_diff = self.slope() - other.slope();
        let x = y_diff / slope_diff;
        let y = self.y(x);

        Some(Point { x, y })
    }
}

fn fold_extendable(
    intercept_line: &HashMap<isize, Vec<Line>>,
) -> impl Fn(Vec<Line>, &isize, Line) -> Vec<Line> + '_ {
    |mut extensions, y_intercept, line| {
        extensions.extend(
            intercept_line
                .get(y_intercept)
                .iter()
                .flat_map(|v| v.iter())
                .filter_map(|other| line.extend(other)),
        );
        extensions
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| line.parse().expect("Could not parse line"))
        .collect()
}

fn solve_part1(pairs: &[Pair], row: isize) -> usize {
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

fn solve_part2(pairs: &[Pair], max_coord: isize) -> usize {
    let min_x_to_max_y = pairs
        .iter()
        .map(Pair::min_x_to_max_y)
        .into_group_map_by(Line::y_intercept);
    let positive_slopes = pairs
        .iter()
        .map(Pair::min_y_to_max_x)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_extendable(&min_x_to_max_y));
    let max_y_to_max_x = pairs
        .iter()
        .map(Pair::max_y_to_max_x)
        .into_group_map_by(Line::y_intercept);
    let negative_slopes = pairs
        .iter()
        .map(Pair::min_x_to_min_y)
        .into_grouping_map_by(Line::y_intercept)
        .fold(vec![], fold_extendable(&max_y_to_max_x));
    let Point { x, y } = positive_slopes.values().flatten().cartesian_product(negative_slopes.values().flatten()).find_map(|(positive, negative)| {
        positive.intersection(negative).filter(|p| {
            p.x >= 0
                && p.y >= 0
                && p.x <= max_coord
                && p.y <= max_coord
                && pairs.iter().all(|pair| !pair.is_covered(p))
        })
    }).expect("No intercept found");
    (x * 4_000_000 + y) as usize
}

#[aoc(day15, part1)]
fn solution_part1(input: &[Pair]) -> usize {
    solve_part1(input, 2_000_000)
}

#[aoc(day15, part2)]
fn solution_part2(input: &[Pair]) -> usize {
    solve_part2(input, 4_000_000)
}

#[cfg(test)]
mod test {

    use super::{parse_input, solve_part1, solve_part2};

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
        let actual = solve_part1(&input, 10);
        assert_eq!(expected, actual);
    }

    #[test]
    fn sample_to_lines_slopes() {
        let input = parse_input(SAMPLE_INPUT);
        let positive_slopes = input
            .iter()
            .flat_map(|pair| [pair.min_x_to_max_y(), pair.min_y_to_max_x()]);
        let negative_slopes = input
            .iter()
            .flat_map(|pair| [pair.max_y_to_max_x(), pair.min_x_to_min_y()]);

        positive_slopes.for_each(|line| assert!(line.slope() > 0));
        negative_slopes.for_each(|line| assert!(line.slope() < 0));
    }

    #[test]
    fn sample_solve_part1() {
        let input = parse_input(SAMPLE_INPUT);
        let expected = 56_000_011;
        let actual = solve_part2(&input, 20);
        assert_eq!(expected, actual);
    }
}
