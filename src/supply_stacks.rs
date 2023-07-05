use regex::Regex;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Command {
    count: usize,
    from: usize,
    to: usize,
}

fn level(line: &str) -> Vec<Option<char>> {
    let mut level: Vec<Option<char>> = Vec::new();
    let re = Regex::new(r"(?:(?:   )|\[(?P<item>[A-Z])\]) ?")
        .expect("Level regex does not compile.");
    for crate_match in re.captures_iter(line) {
        level.push(crate_match.name("item").map(|m| {
            m.as_str()
                .chars()
                .next()
                .expect("Single character in capture group")
        }));
    }

    level
}

fn to_stacks(levels: Vec<Vec<Option<char>>>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![vec!{}; levels[0].len()]; 

    for level in levels.iter().rev() {
        for (pos, item) in level.iter().enumerate() {
            match item {
                Some(crate_item) => stacks[pos].push(*crate_item),
                None => (),
            }
        }
    }

    stacks
}

impl Command {
    fn from(cmd: &str) -> Self {
        let re = Regex::new(r"^move (?<count>\d+) from (?<from_stack>\d+) to (?<to_stack>\d+)$")
            .unwrap();
        let Some(captured_cmd) = re.captures(cmd) else {
            panic!("No match");
        };

        Command {
            count: (&captured_cmd["count"]).parse().expect("NaN"),
            from: (&captured_cmd["from_stack"]).parse().expect("NaN"), 
            to: (&captured_cmd["to_stack"]).parse().expect("NaN"),
        }
    }

    fn get_count(&self) -> usize {
        self.count
    }

    fn get_from(&self) -> usize {
        self.from
    }

    fn get_to(&self) -> usize {
        self.to
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
  let mut lines = input.lines();  
  let mut levels = Vec::new();
  let mut cmds: Vec<Command> = Vec::new();

  for line in &mut lines {
    if !line.contains('[') { break; }
    levels.push(level(line));
  }
  
  let stacks = to_stacks(levels);

  lines.next(); // blank line after stacks

  for line in lines {
    cmds.push(Command::from(line));
  }

  (stacks, cmds)
}

#[aoc(day5, part1)]
fn cargo_mover9000(input: &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut stacks: Vec<Vec<char>> = (*input.0).to_vec();
    let commands: Vec<Command> = (*input.1).to_vec();

    for command in commands {
        let count = command.get_count();
        let from = command.get_from();
        let to = command.get_to();
        for _ in 0..count {
            let moved = stacks[from - 1].pop().expect("No crate present");
            stacks[to - 1].push(moved);
        }
    }

    let mut top_crates = String::new();

    for stack in stacks.iter() {
        match stack.last() {
            Some(top) => top_crates.push(*top),
            None => (),
        }
    }

    top_crates
}

#[aoc(day5, part2)]
fn cargo_mover9001(input: &(Vec<Vec<char>>, Vec<Command>)) -> String {
    let mut stacks: Vec<Vec<char>> = (*input.0).to_vec();
    let commands: Vec<Command> = (*input.1).to_vec();

    for command in commands {
        let from = command.get_from();
        let at = stacks[from - 1].len() - command.get_count();
        let to = command.get_to();
        let mut moved = stacks[from - 1].split_off(at);
        stacks[to - 1].append(&mut moved);
    }

    let mut top_crates = String::new();

    for stack in stacks.iter() {
        match stack.last() {
            Some(top) => top_crates.push(*top),
            None => (),
        }
    }

    top_crates
}

#[cfg(test)]
mod test {
    use std::assert_eq;

    use crate::supply_stacks::Command;
    use crate::supply_stacks::*;

    #[test] 
    fn command_from_str() {
        let one_digit = Command::from("move 1 from 1 to 1");
        let two_digit = Command::from("move 12 from 13 to 14");
        let three_digit = Command::from("move 129 from 139 to 149");
        let zeros_digit = Command::from("move 01 from 10 to 101");

        assert_eq!(1, one_digit.count);
        assert_eq!(1, one_digit.from);
        assert_eq!(1, one_digit.to);

        assert_eq!(12, two_digit.count);
        assert_eq!(13, two_digit.from);
        assert_eq!(14, two_digit.to);

        assert_eq!(129, three_digit.count);
        assert_eq!(139, three_digit.from);
        assert_eq!(149, three_digit.to);

        assert_eq!(1, zeros_digit.count);
        assert_eq!(10, zeros_digit.from);
        assert_eq!(101, zeros_digit.to);
    }

    #[test]
    fn level_crates() {
        let crates = "    [D]    
[N] [C]    
[Z] [M] [P]";
        let mut levels = Vec::new();
        for line in String::from(crates).lines() {
           levels.push(level(line));
        }
        
        assert_eq!(vec!{None, Some('D'), None}, levels[0]);
        assert_eq!(vec!{Some('N'), Some('C'), None}, levels[1]);
        assert_eq!(vec!{Some('Z'), Some('M'), Some('P')}, levels[2]);
    }

    #[test]
    fn stacks_crates() {
        let crates = "    [D]    
[N] [C]    
[Z] [M] [P]";
        let levels: Vec<Vec<Option<char>>> = crates.lines()
                .map(|line| level(line)).collect();

        let stacks = to_stacks(levels);

        assert_eq!(vec!{'Z', 'N'}, stacks[0]);
        assert_eq!(vec!{'M', 'C', 'D'}, stacks[1]);
        assert_eq!(vec!{'P'}, stacks[2]);
    }
}
