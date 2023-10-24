use std::collections::{HashMap, HashSet};

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
    let mut head = Head::new();
    let mut tail = Knot::new();
    let mut vec = Vec::from(input);

    vec.iter_mut().for_each(|movement| {
        while movement.get_amount() > 0 {
            head.do_move(&tail, movement);
            tail.do_move(&head, movement);
        }
    });

    tail.visited.len()
}

#[aoc(day9, part2)]
fn do_movements_vector(input: &[Movement]) -> usize {
    let dc = Head::new();
    let mut dcm = Movement::UP(0);
    let mut head = Head::new();
    let mut knots = vec![
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
        Knot::new(),
    ];
    let mut vec = Vec::from(input);

    vec.iter_mut().for_each(|movement| {
        while movement.get_amount() > 0 {
            let mut sentinel = true;
            let immut_knots = knots.clone();
            head.do_move(&dc, movement);
            let mut prev = immut_knots.first().unwrap();
            for item in knots.iter_mut() {
                if sentinel {
                    sentinel = false;
                    item.do_move(&head, &mut dcm);
                } else {
                    item.do_move(prev, &mut dcm);
                }
                prev = item;
            }
        }
    });

    knots.last().unwrap().visited.len()
}

#[cfg(test)]
mod test {
    use crate::rope_bridge::{do_movements, do_movements_vector, Head, Knot, Movable, Movement, Position, to_movements};

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
        let dc = Head::new();
        let mut head = Head::new();
        let mut movement = Movement::UP(1);

        head.do_move(&dc, &mut movement);

        assert_eq!(Movement::UP(0), movement);
        assert_eq!(Position::from(0, 1), head.pos);
    }

    #[test]
    fn test_head_move_down() {
        let dc = Head::new();
        let mut head = Head::new();
        let mut movement = Movement::DOWN(1);

        head.do_move(&dc, &mut movement);

        assert_eq!(Movement::DOWN(0), movement);
        assert_eq!(Position::from(0, -1), head.pos);
    }

    #[test]
    fn test_head_move_right() {
        let dc = Head::new();
        let mut head = Head::new();
        let mut movement = Movement::RIGHT(1);

        head.do_move(&dc, &mut movement);

        assert_eq!(Movement::RIGHT(0), movement);
        assert_eq!(Position::from(1, 0), head.pos);
    }

    #[test]
    fn test_head_move_left() {
        let dc = Head::new();
        let mut head = Head::new();
        let mut movement = Movement::LEFT(1);

        head.do_move(&dc, &mut movement);

        assert_eq!(Movement::LEFT(0), movement);
        assert_eq!(Position::from(-1, 0), head.pos);
    }

    #[test]
    fn test_tail_move_logic_basic() {
        let mut movement = Movement::RIGHT(2);
        let mut head = Head::new();
        let mut tail = Knot::new();
        head.do_move(&tail, &mut movement);
        tail.do_move(&head, &mut movement);

        assert_eq!(Position::new(), tail.pos);
        assert_eq!(Position::from(1, 0), head.pos);

        head.do_move(&tail, &mut movement);
        tail.do_move(&head, &mut movement);

        assert_eq!(Position::from(1, 0), tail.pos);
        assert_eq!(Position::from(2, 0), head.pos);
        assert_eq!(Movement::RIGHT(0), movement);
    }

    #[test]
    fn test_sample() {
        let movement_list = to_movements(EXAMPLE);
        assert_eq!(13, do_movements(movement_list.as_slice()));
    }

    #[test]
    fn test_sample_v1_vector() {
        let movement_list = to_movements(EXAMPLE);
        assert_eq!(1, do_movements_vector(movement_list.as_slice()));
    }

    #[test]
    fn test_sample_p2() {
        let movement_list = to_movements(EXAMPLE_P2);
        assert_eq!(36, do_movements_vector(movement_list.as_slice()));
    }
}
