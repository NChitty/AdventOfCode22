use std::collections::HashMap;
use std::str::FromStr;

use regex::Regex;

#[derive(Debug, PartialEq)]
enum Instruction {
    NOOP,
    ADDX(isize),
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let matcher = Regex::new("^(?<noop>noop)|((?<addx>addx) (?<num>-?\\d+))$")
            .expect("Could not build regex");
        let Some(cap) = matcher.captures(s) else {
            println!("No match");
            return Err("No match".to_owned());
        };
        if cap.name("noop").is_some() {
            return Ok(Instruction::NOOP);
        } else if cap.name("addx").is_some() {
            return Ok(Instruction::ADDX(
                cap.name("num")
                    .unwrap()
                    .as_str()
                    .parse()
                    .expect("Parsing error"),
            ));
        }

        Err("Could not parse".to_owned())
    }
}

struct CPU {
    register_x: isize,
    cycle: usize,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_x: 1,
            cycle: 0,
        }
    }

    fn do_instr(&mut self, instr: &Instruction) -> HashMap<usize, isize> {
        match instr {
            Instruction::NOOP => {
                self.cycle += 1;
                HashMap::from([(self.cycle, self.register_x)])
            }
            Instruction::ADDX(x) => {
                self.cycle += 2;
                self.register_x += x;
                HashMap::from([
                    (self.cycle - 2, self.register_x - x),
                    (self.cycle - 1, self.register_x - x),
                    (self.cycle, self.register_x),
                ])
            }
        }
    }
}

#[aoc_generator(day10)]
fn parse_instr(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for lines in input.lines() {
        instructions.push(Instruction::from_str(lines).expect("Did not parse"));
    }

    instructions
}

fn run_cpu(cpu: &mut CPU, instructions: &[Instruction]) -> HashMap<usize, isize> {
    let mut counts: HashMap<usize, isize> = HashMap::from([(0, 1)]);
    for instr in instructions {
        cpu.do_instr(instr).iter().for_each(|(&cycle, &regx)| {
            counts.insert(cycle, regx);
        });
    }

    counts
}

fn sum_signal_strengths(indices: &[usize], cycle_reg_x_map: &HashMap<usize, isize>) -> isize {
    indices
        .iter()
        .map(|&idx| isize::try_from(idx).unwrap() * cycle_reg_x_map.get(&(idx - 1)).unwrap())
        .sum()
}

fn draw_crt(cycle_reg_x_map: &HashMap<usize, isize>) -> String {
    let mut crt = String::new();
    let cycle_idx = |col: usize, row: usize| col + row * 40;
    for row in 0..6 {
        for col in 0..40 {
            let cycle_count = cycle_idx(col, row);
            let x_reg = cycle_reg_x_map.get(&cycle_count).unwrap();
            if ((x_reg - 1)..=(x_reg + 1)).contains(&isize::try_from(col).unwrap()) {
                crt.push('#');
                continue;
            }
            crt.push('.');
        }
        crt.push('\n');
    }

    crt
}

#[aoc(day10, part1)]
fn get_signal_strengh_sum(instructions: &[Instruction]) -> isize {
    sum_signal_strengths(
        &[20, 60, 100, 140, 180, 220],
        &run_cpu(&mut CPU::new(), instructions),
    )
}

#[aoc(day10, part2)]
fn get_crt_display(instructions: &[Instruction]) -> String {
    let mut crt = draw_crt(&run_cpu(&mut CPU::new(), instructions));
    crt.insert(0, '\n');

    crt
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::day10::{CPU, draw_crt, Instruction, parse_instr, run_cpu, sum_signal_strengths};

    const CYCLE_EXAMPLE: &str = "noop
addx 3
addx -5";

    const SIGNAL_STR_EXAMPLE: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    const CRT_EXAMPLE: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

    #[test]
    fn test_instruction_from_string() {
        let instructions_correct = vec![
            Instruction::NOOP,
            Instruction::ADDX(3),
            Instruction::ADDX(-5),
        ];
        let instructions_parsed: Vec<Instruction> = parse_instr(CYCLE_EXAMPLE);
        assert_eq!(instructions_correct, instructions_parsed);
    }

    #[test]
    fn test_x_register() {
        let counts_correct: HashMap<usize, isize> =
            HashMap::from([(0, 1), (1, 1), (2, 1), (3, 4), (4, 4), (5, -1)]);

        let instructions: Vec<Instruction> = parse_instr(CYCLE_EXAMPLE);
        let mut cpu = CPU::new();
        let counts_calculated = run_cpu(&mut cpu, &instructions);

        assert_eq!(counts_correct, counts_calculated);
    }

    #[test]
    fn test_signal_strength() {
        let instructions = parse_instr(SIGNAL_STR_EXAMPLE);
        let counts = run_cpu(&mut CPU::new(), &instructions);
        let cycle_indices: [usize; 6] = [20, 60, 100, 140, 180, 220];

        let sum: isize = sum_signal_strengths(&cycle_indices, &counts);

        assert_eq!(13140, sum);
    }

    #[test]
    fn test_crt() {
        let instructions = parse_instr(SIGNAL_STR_EXAMPLE);
        let reg_map = run_cpu(&mut CPU::new(), &instructions);
        let crt = draw_crt(&reg_map);

        assert_eq!(CRT_EXAMPLE, crt);
    }
}
