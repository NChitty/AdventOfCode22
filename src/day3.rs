use std::collections::HashSet;

pub struct Rucksack {
    contents: String,
    mistake: char,
}

impl Rucksack {
    pub fn from(line: &str) -> Self {
        let len = line.len();
        let first_compartment = &line[0..len / 2];
        let set: HashSet<char> = first_compartment.chars().collect();
        let second_compartment = &line[len / 2..];
        let mut duplicate = '\0';
        for char in second_compartment.chars() {
            if !set.contains(&char) { continue; }
            duplicate = char;
            break;
        }

        Rucksack {
            contents: String::from(line),
            mistake: duplicate,
        }
    }

    pub fn get_contents(&self) -> &String {
        &self.contents
    }

    pub fn get_mistake(&self) -> &char {
        &self.mistake
    }
}

pub fn convert_char(char: char) -> Result<u32, &'static str> {
    if char.is_uppercase() { return Ok((char as u32) - ('A' as u32) + 27); }
    if char.is_lowercase() { return Ok((char as u32) - ('a' as u32) + 1); }

    Err("Character must be in the English alphabet.")
}

pub fn find_intersection(a: &Rucksack, b: &Rucksack, c: &Rucksack) -> char {
    let a_set: HashSet<char> = a.get_contents().chars().collect();
    let b_set: HashSet<char> = b.get_contents().chars().collect();
    let c_set: HashSet<char> = c.get_contents().chars().collect();

    let intersect_ab: HashSet<char> = a_set.intersection(&b_set).copied().collect();
    let final_intersect: HashSet<char> = intersect_ab.intersection(&c_set).copied().collect();
    let char = final_intersect.iter().next();
    match char {
        Some(val) => *val,
        None => panic!("No intersection of all three rucksacks"),
    }
}

#[aoc_generator(day3)]
pub fn get_rucksacks(input: &str) -> Vec<Rucksack> {
    input.lines()
        .map(|l| {
            Rucksack::from(l)
        }).collect()
}

#[aoc(day3, part1)]
pub fn sum_mistakes(input: &[Rucksack]) -> u32 {
    input.iter().map(|r| {
        convert_char(*r.get_mistake()).unwrap()
    }).sum()
}

#[aoc(day3, part2)]
pub fn sum_badges(input: &[Rucksack]) -> u32 {
    input.chunks(3).map(|slice| {
        let badge = find_intersection(&slice[0], &slice[1], &slice[2]);
        convert_char(badge).unwrap()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str() {
        let val = "Hello World!";
        let result = Rucksack::from(val);
        assert_eq!(val, result.get_contents());
    }

    #[test]
    fn item_priority() {
        let result = convert_char('a').unwrap();
        assert_eq!(1, result);
        let result = convert_char('z').unwrap();
        assert_eq!(26, result);
        let result = convert_char('A').unwrap();
        assert_eq!(27, result);
        let result = convert_char('Z').unwrap();
        assert_eq!(52, result);
        let string_chars = String::from("abcdef");
        assert_eq!(3, convert_char(string_chars.as_bytes()[2] as char).unwrap());
    }

    #[test]
    fn find_duplicate() {
        let rucksack = Rucksack::from("abcdea");
        let mistake = rucksack.get_mistake();
        assert_eq!('a', *mistake);
    }

    #[test]
    fn find_badge() {
        let a = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
        let b = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        let c = Rucksack::from("PmmdzqPrVvPwwTWBwg");
        assert_eq!('r', find_intersection(&a, &b, &c));

        let a = Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn");
        let b = Rucksack::from("ttgJtRGJQctTZtZT");
        let c = Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw");
        assert_eq!('Z', find_intersection(&a, &b, &c));
    }
}