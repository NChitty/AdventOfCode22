use regex::Regex;
use std::ops::Add;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
enum Operation {
    ADD(u64),
    MUL(u64),
    SQUARE,
}

impl Operation {
    fn get_new_worry_level(&self, item: &mut u64) {
        match self {
            Operation::ADD(rhs) => {
                *item += rhs;
            }
            Operation::MUL(rhs) => {
                *item *= rhs;
            }
            Operation::SQUARE => {
                *item = *item * *item;
            }
        }
    }
}

const OPERATION_REGEX: &str = "old (?<op>[*+]) (?<operand>old|\\d*)";

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let caps = Regex::new(OPERATION_REGEX).unwrap().captures(s);
        match caps {
            None => Err("String does not match".to_owned()),
            Some(caps) => match &caps["op"] {
                "*" => match &caps["operand"] {
                    "old" => Ok(Operation::SQUARE),
                    number => Ok(Operation::MUL(
                        number.parse().expect("Could not parse operand"),
                    )),
                },
                "+" => match &caps["operand"] {
                    "old" => Ok(Operation::MUL(2)),
                    number => Ok(Operation::ADD(
                        number.parse().expect("Could not parse operand"),
                    )),
                },
                _ => Err("Unknown operation".to_owned()),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Test(u64, usize, usize);

impl Test {
    fn get_result(&self, worry_level: &u64) -> usize {
        if worry_level % self.0 == 0 {
            self.1
        } else {
            self.2
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: Test,
    test_count: u64,
}

const NUM_MATCHER: &str = "(?<num>\\d+)$";

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        lines.next(); // consume monkey number line
        let starting_items = lines.next().unwrap();
        let mut starting_items_split = starting_items.split(": ");
        starting_items_split.next(); // consume "Starting items"
        let separated_list = starting_items_split.next().unwrap();
        let items: Vec<u64> = separated_list
            .split(", ")
            .map(|item| item.parse().expect("Could not parse item worry level"))
            .collect();
        let operation = Operation::from_str(lines.next().unwrap()).unwrap();
        let matcher = Regex::new(NUM_MATCHER).unwrap();
        let test = Test(
            matcher.captures(lines.next().unwrap()).unwrap()["num"]
                .parse()
                .unwrap(),
            matcher.captures(lines.next().unwrap()).unwrap()["num"]
                .parse()
                .unwrap(),
            matcher.captures(lines.next().unwrap()).unwrap()["num"]
                .parse()
                .unwrap(),
        );

        Ok({
            Self {
                items,
                operation,
                test,
                test_count: 0,
            }
        })
    }
}

impl Monkey {
    /// Returns a list of tuples which represents the monkey thrown to and the item thrown
    fn take_turn(&mut self, divisor: u64) -> Vec<(usize, u64)> {
        self.items
            .drain(0..self.items.len())
            .map(|mut item| {
                self.operation.get_new_worry_level(&mut item);
                self.test_count += 1;
                item %= divisor;
                (self.test.get_result(&item), item.to_owned())
            })
            .collect()
    }
}

#[aoc_generator(day11)]
fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkey_lines: Vec<String> = Vec::new();
    let mut to_push: String = String::new();
    input.lines().for_each(|line| {
        if line.is_empty() {
            monkey_lines.push(to_push.clone());
            to_push.clear();
        }
        if to_push.is_empty() {
            to_push += line;
        } else {
            to_push += "\n".to_owned().add(line).as_str();
        }
    });
    monkey_lines.push(to_push.clone());
    monkey_lines
        .iter()
        .map(|monkey_str| Monkey::from_str(monkey_str).unwrap())
        .collect()
}

fn do_round(monkeys: &mut [Monkey], divisor: u64) {
    for monkey_idx in 0..monkeys.len() {
        let items_to_move = monkeys.get_mut(monkey_idx).unwrap().take_turn(divisor);
        for (idx, item) in items_to_move {
            monkeys.get_mut(idx).unwrap().items.push(item);
        }
    }
}

fn calc_monkey_business(monkeys: &mut [Monkey]) -> u64 {
    monkeys.sort_by(|a, b| b.test_count.cmp(&a.test_count));
    let mut product = 1;
    monkeys[0..=1]
        .iter()
        .map(|monkey| monkey.test_count)
        .for_each(|amount| product *= amount);
    product
}

#[aoc(day11, part1)]
fn monkey_business_part_1(input: &[Monkey]) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::from(input);
    let divisor: u64 = monkeys.iter().map(|monkey| monkey.test.0).product();
    for _ in 0..20 {
        do_round(&mut monkeys, divisor);
    }

    calc_monkey_business(&mut monkeys)
}

#[aoc(day11, part2)]
fn monkey_business_part_2(input: &[Monkey]) -> u64 {
    let mut monkeys: Vec<Monkey> = Vec::from(input);
    let divisor = monkeys.iter().map(|monkey| monkey.test.0).product();
    for _ in 0..10_000 {
        do_round(&mut monkeys, divisor);
    }

    calc_monkey_business(&mut monkeys)
}

#[cfg(test)]
mod test {
    use crate::day11::{monkey_business_part_1, parse_monkeys, Monkey, Operation, Test, monkey_business_part_2};
    use std::str::FromStr;

    #[test]
    fn op_from_string() {
        let should_be_mul_op = Operation::from_str("  Operation: new = old * 19").unwrap();
        assert_eq!(Operation::MUL(19), should_be_mul_op);

        let should_be_add_op = Operation::from_str("  Operation: new = old + 6").unwrap();
        assert_eq!(Operation::ADD(6), should_be_add_op);

        let should_be_square_op = Operation::from_str("  Operation: new = old * old").unwrap();
        assert_eq!(Operation::SQUARE, should_be_square_op);

        let should_be_mul2_op = Operation::from_str("  Operation: new = old + old").unwrap();
        assert_eq!(Operation::MUL(2), should_be_mul2_op);
    }

    #[test]
    fn monkey_from_string() {
        let items: Vec<u64> = vec![79, 98];
        let operation = Operation::MUL(19);
        let test = Test(23, 2, 3);
        let expected_monkey = Monkey {
            items,
            operation,
            test,
            test_count: 0,
        };

        assert_eq!(
            expected_monkey,
            Monkey::from_str(
                "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3"
            )
            .unwrap()
        );
    }

    const PART_1: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn p1_parse() {
        let monkeys = parse_monkeys(PART_1);
        let expected = vec![
            Monkey {
                items: vec![79, 98],
                operation: Operation::MUL(19),
                test: Test(23, 2, 3),
                test_count: 0
            },
            Monkey {
                items: vec![54, 65, 75, 74],
                operation: Operation::ADD(6),
                test: Test(19, 2, 0),
                test_count: 0
            },
            Monkey {
                items: vec![79, 60, 97],
                operation: Operation::SQUARE,
                test: Test(13, 1, 3),
                test_count: 0
            },
            Monkey {
                items: vec![74],
                operation: Operation::ADD(3),
                test: Test(17, 0, 1),
                test_count: 0
            }
        ];
        assert_eq!(expected, monkeys);
    }

    #[test]
    fn p1_full_run() {
        let monkeys = parse_monkeys(PART_1);
        let output = monkey_business_part_1(&monkeys.clone());
        assert_eq!(10605, output);
    }

    #[test]
    fn p2_full_run() {
        let monkeys = parse_monkeys(PART_1);
        let output = monkey_business_part_2(&monkeys.clone());
        assert_eq!(2713310158, output);
    }
}
