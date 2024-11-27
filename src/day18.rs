use std::{
    collections::{hash_map::Entry, HashMap},
    ops::Add,
    str::FromStr,
};

use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vals: (isize, isize, isize) = s
            .split(",")
            .filter_map(|val| val.parse::<isize>().ok())
            .collect_tuple()
            .ok_or("Expect tuple")?;
        let x = vals.0;
        let y = vals.1;
        let z = vals.2;
        Ok(Self { x, y, z })
    }
}

impl Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

enum Delta {
    Up,
    Down,
    Left,
    Right,
    Front,
    Back,
}

impl Delta {
    fn mask(&self) -> u8 {
        match self {
            Delta::Up => 1 << 5,
            Delta::Down => 1 << 4,
            Delta::Left => 1 << 3,
            Delta::Right => 1 << 2,
            Delta::Front => 1 << 1,
            Delta::Back => 1,
        }
    }

    fn delta_pos(&self) -> Position {
        match self {
            Delta::Up => Position { x: 0, y: 1, z: 0 },
            Delta::Down => Position { x: 0, y: -1, z: 0 },
            Delta::Left => Position { x: -1, y: 0, z: 0 },
            Delta::Right => Position { x: 1, y: 0, z: 0 },
            Delta::Front => Position { x: 0, y: 0, z: -1 },
            Delta::Back => Position { x: 0, y: 0, z: 1 },
        }
    }

    fn inverse(&self) -> Delta {
        match self {
            Delta::Up => Delta::Down,
            Delta::Down => Delta::Up,
            Delta::Left => Delta::Right,
            Delta::Right => Delta::Left,
            Delta::Front => Delta::Back,
            Delta::Back => Delta::Front,
        }
    }

    fn deltas() -> Vec<Delta> {
        vec![
            Delta::Up,
            Delta::Down,
            Delta::Left,
            Delta::Right,
            Delta::Front,
            Delta::Back,
        ]
    }
}

#[aoc_generator(day18)]
fn to_positions(input: &str) -> Vec<Position> {
    input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect_vec()
}

#[aoc(day18, part1)]
fn count_visible_faces(positions: &[Position]) -> u32 {
    let mut position_visible_faces: HashMap<Position, u8> = HashMap::new();
    positions.iter().for_each(|position| {
        // insert
        position_visible_faces.entry(*position).or_insert(0x3F);
        // check faces
        for delta in Delta::deltas() {
            let adjacent_pos = *position + delta.delta_pos();
            // update
            let entry = position_visible_faces
                .entry(adjacent_pos)
                .and_modify(|val| *val &= !delta.mask());
            match entry {
                Entry::Occupied(_) => {
                  let cur_val = position_visible_faces.get_mut(position).expect("Could not get current position");
                  *cur_val &= !delta.inverse().mask();
                },
                Entry::Vacant(_) => (),
            }
        }
    });
    position_visible_faces
        .values()
        .map(|byte| byte.count_ones())
        .sum()
}

#[cfg(test)]
mod test {
    use super::{count_visible_faces, to_positions};

  const SAMPLE_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

  #[test]
  fn sample_part1() {
    let positions = to_positions(SAMPLE_INPUT);
    let expected = 64;
    let actual = count_visible_faces(&positions);
    assert_eq!(expected, actual);
  }
}
