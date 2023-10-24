use std::cmp::Ordering;
use std::collections::HashSet;

use regex::Regex;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Movement {
    UP(usize),
    DOWN(usize),
    LEFT(usize),
    RIGHT(usize),
}

trait Movable {
    fn do_move(&mut self, parent: &impl Movable, movement: &mut Movement);
    fn get_position(&self) -> Position;
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

#[derive(Debug, Clone, Copy)]
struct Head {
    pos: Position,
}

impl Head {
    fn new() -> Head {
        Head {
            pos: Position::new()
        }
    }
}

impl Movable for Head {
    fn do_move(&mut self, _: &impl Movable, movement: &mut Movement) {
        match movement {
            Movement::UP(amount) if *amount > 0 => {
                self.pos.y += 1;
                *amount -= 1;
            }
            Movement::DOWN(amount) if *amount > 0 => {
                self.pos.y -= 1;
                *amount -= 1;
            }
            Movement::LEFT(amount) if *amount > 0 => {
                self.pos.x -= 1;
                *amount -= 1;
            }
            Movement::RIGHT(amount) if *amount > 0 => {
                self.pos.x += 1;
                *amount -= 1;
            }
            _ => {}
        }
    }

    fn get_position(&self) -> Position {
        self.pos
    }
}

#[derive(Clone)]
struct Knot {
    pos: Position,
    visited: HashSet<Position>,
}

impl Knot {
    fn new() -> Knot {
        Knot {
            pos: Position::new(),
            visited: HashSet::new(),
        }
    }
}

impl Movable for Knot {
    fn do_move(&mut self, parent: &impl Movable, _: &mut Movement) {
        let head_pos = parent.get_position();
        let diff_pos = head_pos.sub(&self.pos);
        match diff_pos {
            Position { x: 0, y: 0 } => {}
            Position { x, y } if x.abs() == 1 && y.abs() == 1 => {}
            Position { x: 0, y } if y.abs() > 1 => {
                self.pos.y = match y.cmp(&0) {
                    Ordering::Less => self.pos.y + y + 1,
                    Ordering::Equal => self.pos.y + y,
                    Ordering::Greater => self.pos.y + y - 1,
                }
            }
            Position { x, y: 0 } if x.abs() > 1 => {
                self.pos.x = match x.cmp(&0) {
                    Ordering::Less => self.pos.x + x + 1,
                    Ordering::Equal => self.pos.x + x,
                    Ordering::Greater => self.pos.x + x - 1,
                }
            }
            Position { x, y } if x.abs() >= 1 && y.abs() >= 1 => {
                let offset_pos = match (x.cmp(&0), y.cmp(&0), x.abs() > y.abs()) {
                    (Ordering::Less, Ordering::Less, false) => Position::from(0, 1),
                    (Ordering::Less, Ordering::Less, true) => Position::from(1, 0),
                    (Ordering::Greater, Ordering::Less, false) => Position::from(0, 1),
                    (Ordering::Greater, Ordering::Less, true) => Position::from(-1, 0),
                    (Ordering::Less, Ordering::Greater, false) => Position::from(0, -1),
                    (Ordering::Less, Ordering::Greater, true) => Position::from(1, 0),
                    (Ordering::Greater, Ordering::Greater, false) => Position::from(0, -1),
                    (Ordering::Greater, Ordering::Greater, true) => Position::from(-1, 0),
                    (_, _, _) => panic!("Offset")
                };
                self.pos.add(diff_pos);
                self.pos.add(offset_pos);
            }
            _ => {}
        }

        self.visited.insert(self.pos);
    }

    fn get_position(&self) -> Position {
        self.pos
    }
}


#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Position {
            x: 0,
            y: 0,
        }
    }

    fn from(x: isize, y: isize) -> Self {
        Position {
            x,
            y,
        }
    }

    fn add(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }

    fn sub(&self, rhs: &Self) -> Self {
        Position {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
