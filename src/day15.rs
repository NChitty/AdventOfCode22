use std::{cmp::min, collections::HashMap, isize};

use regex::Regex;

const LOCATION_REGEX: &str = "Sensor at x=(?<sensor_x>[-\\d]*?), y=(?<sensor_y>[-\\d]*?): closest beacon is at x=(?<beacon_x>[-\\d]*?), y=(?<beacon_y>[-\\d]*?)$";

fn get_disallowed_locations(
    sensor_beacon_map: &HashMap<(isize, isize), (isize, isize)>,
    min_pos: (isize, isize),
    max_pos: (isize, isize),
) -> Vec<Vec<bool>> {
    let mut possible_locations =
        vec![vec![true; max_pos.0.abs_diff(min_pos.0) + 1]; max_pos.1.abs_diff(min_pos.1) + 1];
    sensor_beacon_map.keys().for_each(|key| {
        let beacon_pos: (isize, isize) = *sensor_beacon_map.get(key).expect("No value for key");
        let max_delta: isize = (beacon_pos.0.abs_diff(key.0) + beacon_pos.1.abs_diff(key.1))
            .try_into()
            .expect("Could not convert to isize.");
        let min_beacon_x = min(key.0 - max_delta, min_pos.0);
        let max_beacon_x = min(key.0 + max_delta, max_pos.0);
        let min_beacon_y = min(key.1 - max_delta, min_pos.1);
        let max_beacon_y = min(key.1 + max_delta, max_pos.1);
        (min_beacon_x..=max_beacon_x).for_each(|x| {
            (min_beacon_y..=max_beacon_y)
                .filter(|y| {
                    let x_delta: isize = x
                        .abs_diff(key.0)
                        .try_into()
                        .expect("Could not convert to isize");
                    let y_delta: isize = y
                        .abs_diff(key.1)
                        .try_into()
                        .expect("Could not convert to isize");
                    x_delta + y_delta <= max_delta
                })
                .for_each(|y| {
                    let vec_x = x.abs_diff(min_pos.0);
                    let vec_y = y.abs_diff(min_pos.1);
                    possible_locations[vec_y][vec_x] = false;
                });
        });
    });
    sensor_beacon_map.values().for_each(|pos| {
        let vec_x = pos.0.abs_diff(min_pos.0);
        let vec_y = pos.1.abs_diff(min_pos.1);
        possible_locations[vec_y][vec_x] = true;
    });
    possible_locations
}

#[aoc_generator(day15, part1)]
fn get_sensor_beacon_map(
    input: &str,
) -> (
    HashMap<(isize, isize), (isize, isize)>,
    (isize, isize),
    (isize, isize),
) {
    let mut min_pos = (isize::MAX, isize::MAX);
    let mut max_pos = (isize::MIN, isize::MIN);
    let matcher = Regex::new(LOCATION_REGEX).expect("Could not build regex.");
    let mut map = HashMap::new();
    input.lines().for_each(|line| {
        if let Some(cap) = matcher.captures(line) {
            let sensor_pos: (isize, isize) = (
                cap.name("sensor_x")
                    .expect("No match for sensor x position")
                    .as_str()
                    .parse()
                    .expect("Could not parse sensor x as integer"),
                cap.name("sensor_y")
                    .expect("No match for sensor y position")
                    .as_str()
                    .parse()
                    .expect("Could not parse sensor y as integer"),
            );
            if sensor_pos.0 < min_pos.0 {
                min_pos.0 = sensor_pos.0;
            }
            if sensor_pos.1 < min_pos.1 {
                min_pos.1 = sensor_pos.1;
            }
            if sensor_pos.0 > max_pos.0 {
                max_pos.0 = sensor_pos.0;
            }
            if sensor_pos.1 > max_pos.1 {
                max_pos.1 = sensor_pos.1;
            }
            let beacon_pos: (isize, isize) = (
                cap.name("beacon_x")
                    .expect("No match for beacon x position")
                    .as_str()
                    .parse()
                    .expect("Could not parse beacon x as integer"),
                cap.name("beacon_y")
                    .expect("No match for beacon y position")
                    .as_str()
                    .parse()
                    .expect("Could not parse beacon y as integer"),
            );
            if beacon_pos.0 < min_pos.0 {
                min_pos.0 = beacon_pos.0;
            }
            if beacon_pos.1 < min_pos.1 {
                min_pos.1 = beacon_pos.1;
            }
            if beacon_pos.0 > max_pos.0 {
                max_pos.0 = beacon_pos.0;
            }
            if beacon_pos.1 > max_pos.1 {
                max_pos.1 = beacon_pos.1;
            }
            map.insert(sensor_pos, beacon_pos);
        }
    });
    (map, min_pos, max_pos)
}

#[aoc(day15, part1)]
fn count_disallowed_locations_in_row(
    input: &(
        HashMap<(isize, isize), (isize, isize)>,
        (isize, isize),
        (isize, isize),
    ),
) -> usize {
    let possible_locations = get_disallowed_locations(&input.0, input.1, input.2);
    let vec_y = 2_000_000isize.abs_diff(input.1 .1);
    let disallowed = possible_locations[vec_y]
        .iter()
        .filter(|&&is_allowed| !is_allowed)
        .count();
    disallowed
}

#[cfg(test)]
mod test {
    use super::{get_disallowed_locations, get_sensor_beacon_map};

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
    fn sample_count_part1() {
        let obs = get_sensor_beacon_map(SAMPLE_INPUT);
        let expected = 26;
        let possible_locations = get_disallowed_locations(&obs.0, obs.1, obs.2);
        let vec_y = 10isize.abs_diff(obs.1 .1);
        let actual = possible_locations[vec_y]
            .iter()
            .filter(|&&is_allowed| !is_allowed)
            .count();
        assert_eq!(expected, actual);
    }
}
