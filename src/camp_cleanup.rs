#[derive(Debug, PartialEq)]
struct CleaningAssignment {
    low: u32,
    high: u32,
}

impl CleaningAssignment {
    fn from(str: &str) -> (Self, Self) {
        let line = String::from(str);
        let ranges: Vec<&str> = line.split(',').collect();
        let mut a: Option<CleaningAssignment> = None;
        let mut b: Option<CleaningAssignment> = None;
        for str in ranges {
            let nums: Vec<&str> = str.split('-').collect();
            let low = nums.first().expect("Index not found").parse().expect("Not a number.");
            let high = nums.get(1).expect("Index not found").parse().expect("Not a number.");
            if a.is_none() {
                a = Option::Some(CleaningAssignment {
                    low,
                    high,
                });
            }
            b = Option::Some(CleaningAssignment {
                low,
                high,
            });
        }

        (a.unwrap(), b.unwrap())
    }

    fn fully_contained(pair: &(CleaningAssignment, CleaningAssignment)) -> bool {
        pair.0.low <= pair.1.low && pair.0.high >= pair.1.high
            || pair.1.low <= pair.0.low && pair.1.high >= pair.0.high
    }

    fn partial_overlap(pair: &(CleaningAssignment, CleaningAssignment)) -> bool {
        if pair.0.low <= pair.1.high && pair.1.low <= pair.0.low { return true; }
        if pair.1.low <= pair.0.high && pair.0.low <= pair.1.low { return true; }
        false
    }
}

#[aoc_generator(day4)]
fn make_cleaning_pairs(input: &str) -> Vec<(CleaningAssignment, CleaningAssignment)> {
    input.lines().map(|line| {
        CleaningAssignment::from(line)
    }).collect()
}

#[aoc(day4, part1)]
fn count_fully_overlapped(input: &[(CleaningAssignment, CleaningAssignment)]) -> u32 {
    input.iter().map(|pair| {
        if CleaningAssignment::fully_contained(pair) { return 1; }
        0
    }).sum()
}

#[aoc(day4, part2)]
fn count_partially_overlapped(input: &[(CleaningAssignment, CleaningAssignment)]) -> u32 {
    input.iter().map(|pair| {
        if CleaningAssignment::partial_overlap(pair) { return 1; }
        0
    }).sum()
}


#[cfg(test)]
mod tests {
    use crate::camp_cleanup::CleaningAssignment;

    #[test]
    fn cleaning_assignment_from_str() {
        let pair = CleaningAssignment::from("2-4,6-8");
        let a = CleaningAssignment { low: 2, high: 4 };
        let b = CleaningAssignment { low: 6, high: 8 };
        assert_eq!((a, b), pair);
    }

    #[test]
    fn fully_contained() {
        let pair_true = CleaningAssignment::from("2-8,3-7");
        assert!(CleaningAssignment::fully_contained(&pair_true));
        let pair_true = CleaningAssignment::from("6-6,4-6");
        assert!(CleaningAssignment::fully_contained(&pair_true));
        let pair_false = CleaningAssignment::from("2-4,6-8");
        assert!(!CleaningAssignment::fully_contained(&pair_false));
        let pair_false = CleaningAssignment::from("2-7,5-8");
        assert!(!CleaningAssignment::fully_contained(&pair_false));
    }

    #[test]
    fn partially_contained() {
        let pair_true = CleaningAssignment::from("5-7,7-9");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("6-9,5-8");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("3-7,2-8");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("2-8,3-7");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("6-6,4-6");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("6-6,4-7");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("4-7,4-7");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_true = CleaningAssignment::from("2-6,4-8");
        assert!(CleaningAssignment::partial_overlap(&pair_true));

        let pair_false = CleaningAssignment::from("1-2,3-4");
        assert!(!CleaningAssignment::partial_overlap(&pair_false));

        let pair_false = CleaningAssignment::from("3-4,1-2");
        assert!(!CleaningAssignment::partial_overlap(&pair_false));
    }
}