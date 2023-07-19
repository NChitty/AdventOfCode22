use std::cmp::Ordering;

#[derive(Clone)]
struct Elf {
    food: Vec<usize>,
    total: usize
}

impl Elf {
    pub fn new() -> Self {
        Elf {
            food: Vec::new(),
            total: 0
        }
    }

    pub fn add_food_item(&mut self, cal: usize) {
        self.food.push(cal);
        self.update_total();
    }

    fn update_total(&mut self) {
        self.total = self.food.iter().sum();
    }

    pub fn get_total(&self) -> usize {
        self.total
    }
}

impl Eq for Elf {}

impl PartialEq<Self> for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.get_total() == other.get_total()
    }
}

impl PartialOrd<Self> for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.get_total().partial_cmp(&other.get_total())
    }
}

impl Ord for Elf {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_total().cmp(&other.get_total())
    }
}

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();
    let mut line_iter = input.lines();
    let mut cur_line = line_iter.next();
    let mut cur_elf = Elf::new();
    while cur_line.is_some() {
        match cur_line {
            Some("") => {
                elves.push(cur_elf);
                cur_elf = Elf::new();
            },
            Some(num) => {
                cur_elf.add_food_item(num.parse().expect("Need a number"));
            }
            None => {
                elves.push(cur_elf);
                cur_elf = Elf::new();
            }
        }
        cur_line = line_iter.next();
    }

    elves
}

#[aoc(day1, part1)]
fn top_elf(input: &[Elf]) -> usize {
    let mut elves = Vec::from(input);
    elves.sort_by(|a, b| b.cmp(a));
    elves[0].get_total()
}

#[aoc(day1, part2)]
fn top_three_elves_sum(input: &[Elf]) -> usize {
    let mut elves = Vec::from(input);
    elves.sort_by(|a, b| b.cmp(a));
    elves[0..=2].iter().map(|elf| elf.get_total()).sum()
}
