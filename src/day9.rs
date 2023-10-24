use std::collections::{HashSet};

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Movement {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

impl Movement {
    fn get_amount(&self) -> usize {
        match self {
            Movement::UP(amount) => *amount,
            Movement::DOWN(amount) => *amount,
            Movement::LEFT(amount) => *amount,
            Movement::RIGHT(amount) => *amount
        }
    }
}

#[derive(Clone)]
struct Knot {
    pos: (isize, isize),
    visited: HashSet<(isize, isize)>,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            pos: (0, 0),
            visited: HashSet::new(),
        }
    }

    fn move_as_head(&mut self, movement: &mut Movement) {
        match movement {
            Movement::UP(amount) if *amount > 0 => {
                self.pos.1 += 1;
                *amount -= 1;
            }
            Movement::DOWN(amount) if *amount > 0 => {
                self.pos.1 -= 1;
                *amount -= 1;
            }
            Movement::LEFT(amount) if *amount > 0 => {
                self.pos.0 -= 1;
                *amount -= 1;
            }
            Movement::RIGHT(amount) if *amount > 0 => {
                self.pos.0 += 1;
                *amount -= 1;
            }
            _ => {}
        }
    }

    fn move_as_follower(&mut self, parent: &Knot) {
        let diff_pos =
            (parent.get_position().0 - self.pos.0, parent.get_position().1 - self.pos.1);
        match diff_pos {
            (x, y) if x.abs() > 1 => {
                if y.abs() == 1 {
                    self.pos.0 += x / 2;
                    self.pos.1 += y;
                } else {
                    self.pos.0 += x / 2;
                }
            }
            (x, y) if y.abs() > 1 => {
                if x.abs() == 1 {
                    self.pos.0 += x;
                    self.pos.1 += y / 2;
                } else {
                    self.pos.1 += y / 2;
                }
            }
            (_, _) => {}
        }

        self.visited.insert(self.pos);
    }

    fn get_position(&self) -> &(isize, isize) {
        &self.pos
    }
}

#[aoc_generator(day9)]
pub(crate) fn to_movements(input: &str) -> Vec<Movement> {
    let mut movements = Vec::new();
    let matcher = Regex::new("^(?<dir>[DLRU]) (?<num>\\d+)$")
        .expect("Could not build regex.");
    for line in input.lines() {
        if let Some(cap) = matcher.captures(line) {
            match &cap["dir"] {
                "U" => movements.push(Movement::UP(cap.name("num")
                    .expect("No match")
                    .as_str()
                    .parse()
                    .expect("Could not parse"))),
                "D" => movements.push(Movement::DOWN(cap.name("num")
                    .expect("No match")
                    .as_str()
                    .parse()
                    .expect("Could not parse"))),
                "L" => movements.push(Movement::LEFT(cap.name("num")
                    .expect("No match")
                    .as_str()
                    .parse()
                    .expect("Could not parse"))),
                "R" => movements.push(Movement::RIGHT(cap.name("num")
                    .expect("No match")
                    .as_str()
                    .parse()
                    .expect("Could not parse"))),
                _ => panic!("Unknown char in input."),
            }
        }
    }
    movements
}

#[aoc(day9, part1)]
fn do_movements(input: &[Movement]) -> usize {
    let mut movements = Vec::from(input);
    let mut knots = build_rope(2);

    iter_moves(&mut movements, &mut knots).len()
}

fn iter_moves<'a>(movements: &'a mut [Movement], knots: &'a mut Vec<Knot>) -> &'a HashSet<(isize, isize)> {
    movements.iter_mut().for_each(|movement| {
        while movement.get_amount() > 0 {
            do_move(knots, movement)
        }
    });

    &knots.last().unwrap().visited
}

fn build_rope(num_knots: usize) -> Vec<Knot> {
    let mut knots = Vec::new();
    (0..num_knots).for_each(|_| {
        knots.push(Knot::new());
    });

    knots
}

fn do_move(knots: &mut Vec<Knot>, movement: &mut Movement) {
    knots.first_mut().unwrap().move_as_head(movement);
    for i in 0..knots.len() - 1 {
        move_follower(knots, i);
    }
}

fn move_follower(knots: &mut [Knot], parent_idx: usize) {
    let binding = knots.to_owned();
    let parent = binding.get(parent_idx).unwrap();
    knots.get_mut(parent_idx + 1)
        .unwrap()
        .move_as_follower(parent);
}

#[aoc(day9, part2)]
fn do_movements_long(input: &[Movement]) -> usize {
    let mut movements = Vec::from(input);
    let mut knots = build_rope(10);

    iter_moves(&mut movements, &mut knots).len()
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;
    use crate::day9::{build_rope, do_movements, do_movements_long, iter_moves, Knot, Movement, to_movements};

    const EXAMPLE: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const EXAMPLE_P2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_to_movement() {
        let movement_list = to_movements(EXAMPLE);

        assert_eq!(Movement::RIGHT(4), movement_list.get(0).cloned().unwrap());
        assert_eq!(Movement::UP(4), movement_list.get(1).cloned().unwrap());
        assert_eq!(Movement::LEFT(3), movement_list.get(2).cloned().unwrap());
        assert_eq!(Movement::DOWN(1), movement_list.get(3).cloned().unwrap());
        assert_eq!(Movement::RIGHT(4), movement_list.get(4).cloned().unwrap());
        assert_eq!(Movement::DOWN(1), movement_list.get(5).cloned().unwrap());
        assert_eq!(Movement::LEFT(5), movement_list.get(6).cloned().unwrap());
        assert_eq!(Movement::RIGHT(2), movement_list.get(7).cloned().unwrap());
    }

    #[test]
    fn test_head_move_up() {
        let mut head = Knot::new();
        let mut movement = Movement::UP(1);

        head.move_as_head(&mut movement);

        assert_eq!(Movement::UP(0), movement);
        assert_eq!((0, 1), head.pos);
    }

    #[test]
    fn test_head_move_down() {
        let mut head = Knot::new();
        let mut movement = Movement::DOWN(1);

        head.move_as_head(&mut movement);

        assert_eq!(Movement::DOWN(0), movement);
        assert_eq!((0, -1), head.pos);
    }

    #[test]
    fn test_head_move_right() {
        let mut head = Knot::new();
        let mut movement = Movement::RIGHT(1);

        head.move_as_head(&mut movement);

        assert_eq!(Movement::RIGHT(0), movement);
        assert_eq!((1, 0), head.pos);
    }

    #[test]
    fn test_head_move_left() {
        let mut head = Knot::new();
        let mut movement = Movement::LEFT(1);

        head.move_as_head(&mut movement);

        assert_eq!(Movement::LEFT(0), movement);
        assert_eq!((-1, 0), head.pos);
    }

    #[test]
    fn test_cmp_set_p1() {
        let mut cmp_set: HashSet<(isize, isize)> = HashSet::new();
        cmp_set.insert((0, 0));
        cmp_set.insert((1, 0));
        cmp_set.insert((2, 0));
        cmp_set.insert((3, 0));
        cmp_set.insert((4, 1));
        cmp_set.insert((4, 2));
        cmp_set.insert((3, 2));
        cmp_set.insert((2, 2));
        cmp_set.insert((1, 2));
        cmp_set.insert((4, 3));
        cmp_set.insert((3, 3));
        cmp_set.insert((3, 4));
        cmp_set.insert((2, 4));
        let mut movement_list = to_movements(EXAMPLE);
        let mut knots = build_rope(2);
        let tail_pos = iter_moves(&mut movement_list, &mut knots);

        assert_eq!(cmp_set, *tail_pos);
    }

    #[test]
    fn test_sample() {
        let movement_list = to_movements(EXAMPLE);
        assert_eq!(13, do_movements(movement_list.as_slice()));
    }

    #[test]
    fn test_sample_p1_long() {
        let movement_list = to_movements(EXAMPLE);
        assert_eq!(1, do_movements_long(movement_list.as_slice()));
    }

    #[test]
    fn test_cmp_set_p2() {
        let mut cmp_set: HashSet<(isize, isize)> = HashSet::new();
        cmp_set.insert((0, 0));
        cmp_set.insert((1, 1));
        cmp_set.insert((2, 2));
        cmp_set.insert((1, 3));
        cmp_set.insert((2, 4));
        cmp_set.insert((3, 5));
        cmp_set.insert((4, 5));
        cmp_set.insert((5, 5));
        cmp_set.insert((6, 4));
        cmp_set.insert((7, 3));
        cmp_set.insert((8, 2));
        cmp_set.insert((9, 1));
        cmp_set.insert((10, 0));
        cmp_set.insert((9, -1));
        cmp_set.insert((8, -2));
        cmp_set.insert((7, -3));
        cmp_set.insert((6, -4));
        cmp_set.insert((5, -5));
        cmp_set.insert((4, -5));
        cmp_set.insert((3, -5));
        cmp_set.insert((2, -5));
        cmp_set.insert((1, -5));
        cmp_set.insert((0, -5));
        cmp_set.insert((-1, -5));
        cmp_set.insert((-2, -5));
        cmp_set.insert((-3, -4));
        cmp_set.insert((-4, -3));
        cmp_set.insert((-5, -2));
        cmp_set.insert((-6, -1));
        cmp_set.insert((-7, 0));
        cmp_set.insert((-8, 1));
        cmp_set.insert((-9, 2));
        cmp_set.insert((-10, 3));
        cmp_set.insert((-11, 4));
        cmp_set.insert((-11, 5));
        cmp_set.insert((-11, 6));

        let mut movement_list = to_movements(EXAMPLE_P2);
        let mut knots = build_rope(10);
        let tail_pos = iter_moves(&mut movement_list, &mut knots);

        let _ = dbg!(cmp_set.difference(tail_pos));
        assert_eq!(cmp_set, *tail_pos);
    }

    #[test]
    fn test_sample_p2() {
        let movement_list = to_movements(EXAMPLE_P2);
        assert_eq!(36, do_movements_long(movement_list.as_slice()));
    }
}
