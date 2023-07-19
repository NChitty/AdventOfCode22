#[derive(Debug, PartialEq, Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Win,
    Lose,
    Draw
}

impl Choice {
    fn get_choice(input: char) -> Self {
        match input {
            'A' | 'a' => Choice::Rock,
            'B' | 'b' => Choice::Paper,
            'C' | 'c' => Choice::Scissors,
            'X' | 'x' => Choice::Rock,
            'Y' | 'y' => Choice::Paper,
            'Z' | 'z' => Choice::Scissors,
            _ => panic!("Illegal character found")
        }
    }

    fn get_score(&self) -> usize {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

impl RoundResult {
    /// Returns the round result based on the encoding from AOC22 D2 P2
    ///
    /// # Arguments
    ///
    /// * `input` - A character
    fn get_round_result(input: char) -> Self {
        match input {
            'X' | 'x' => RoundResult::Lose,
            'Y' | 'y' => RoundResult::Draw,
            'Z' | 'z' => RoundResult::Win,
            _ => panic!("Illegal character found"),
        }
    }

    fn get_choice(&self, opp_choice: &Choice) -> Choice {
        match self {
            RoundResult::Draw => *opp_choice,
            RoundResult::Win => match opp_choice {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            RoundResult::Lose => match opp_choice {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
        }
    }

    fn determine_result(choices: &(Choice, Choice)) -> Self {
        match choices {
            (Choice::Scissors, Choice::Paper)
            | (Choice::Paper, Choice::Rock)
            | (Choice::Rock, Choice::Scissors) => RoundResult::Win,
            (Choice::Paper, Choice::Scissors)
            | (Choice::Scissors, Choice::Rock)
            | (Choice::Rock, Choice::Paper) => RoundResult::Lose,
            _ => RoundResult::Draw,
        }
    }

    fn get_score(&self) -> usize {
        match self {
            RoundResult::Win => 6,
            RoundResult::Draw => 3,
            RoundResult::Lose => 0,
        }
    }
}

#[aoc_generator(day2, part1)]
fn parse_input_choices(input: &str) -> Vec<(Choice, Choice)> {
    input.lines().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        (Choice::get_choice(chars[2]), Choice::get_choice(chars[0]))
    }).collect()
}

#[aoc_generator(day2, part2)]
fn parse_input_opp_round_result(input: &str) -> Vec<(Choice, RoundResult)> {
    input.lines().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        (Choice::get_choice(chars[0]), RoundResult::get_round_result(chars[2]))
    }).collect()
}

#[aoc(day2, part1)]
fn get_tourney_result_choices(input: &[(Choice, Choice)]) -> usize {
    input.iter().map(|round| {
        round.0.get_score() + RoundResult::determine_result(round).get_score()
    }).sum()
}

#[aoc(day2, part2)]
fn get_tourney_result(input: &[(Choice, RoundResult)]) -> usize {
    input.iter().map(|round| {
        round.1.get_choice(&round.0).get_score() + round.1.get_score()
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_win_cons() {
        let result = RoundResult::determine_result(&(Choice::Scissors, Choice::Paper));
        assert_eq!(result, RoundResult::Win);
        let result = RoundResult::determine_result(&(Choice::Paper, Choice::Rock));
        assert_eq!(result, RoundResult::Win);
        let result = RoundResult::determine_result(&(Choice::Rock, Choice::Scissors));
        assert_eq!(result, RoundResult::Win);
    }

    #[test]
    fn test_loss_cons() {
        let result = RoundResult::determine_result(&(Choice::Paper, Choice::Scissors));
        assert_eq!(result, RoundResult::Lose);
        let result = RoundResult::determine_result(&(Choice::Rock, Choice::Paper));
        assert_eq!(result, RoundResult::Lose);
        let result = RoundResult::determine_result(&(Choice::Scissors, Choice::Rock));
        assert_eq!(result, RoundResult::Lose);
    }

    #[test]
    fn test_draw_cons() {
        let result = RoundResult::determine_result(&(Choice::Paper, Choice::Paper));
        assert_eq!(result, RoundResult::Draw);
        let result = RoundResult::determine_result(&(Choice::Rock, Choice::Rock));
        assert_eq!(result, RoundResult::Draw);
        let result = RoundResult::determine_result(&(Choice::Scissors, Choice::Scissors));
        assert_eq!(result, RoundResult::Draw);
    }
}

