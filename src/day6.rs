fn unique(window: &[char]) -> bool {
    let mut letters: Vec<char> = Vec::new();
    for letter in window {
        if !letters.contains(letter) {
            letters.push(*letter);
        } else {
            return false;
        }
    }
    
    true
}

fn find_start_of_packet(buffer: &Vec<char>) -> usize {
    for (pos, window) in buffer.windows(4).enumerate() {
        if unique(window) { return pos + 4; }
    }
    
    buffer.len()
}

fn find_start_of_message(buffer: &Vec<char>) -> usize {
    for (pos, window) in buffer.windows(14).enumerate() {
        if unique(window) { return pos + 14; }
    }

    buffer.len()
}

#[aoc_generator(day6)]
fn buffer_translator(input: &str) -> Vec<char> {
    String::from(input).chars().collect()
}

#[aoc(day6, part1)]
fn start_of_packet(input: &Vec<char>) -> usize {
    find_start_of_packet(input)
}

#[aoc(day6, part2)]
fn start_ofmessage(input: &Vec<char>) -> usize {
    find_start_of_message(input)
}

#[cfg(test)]
mod test {
    use crate::day6::*;

    #[test]
    fn unique_true() {
        assert!(unique(&['a', 'b', 'c', 'd']));
    }

    #[test]
    fn unique_false() {
        assert!(!unique(&['b', 'b', 'c', 'd']));
        assert!(!unique(&['a', 'b', 'b', 'd']));
        assert!(!unique(&['a', 'b', 'c', 'c']));
        assert!(!unique(&['a', 'b', 'c', 'a']));
        assert!(!unique(&['a', 'b', 'a', 'd']));
        assert!(!unique(&['a', 'd', 'c', 'd']));
    }

    #[test]
    fn position() {
        let test_7 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb").chars().collect();
        let test_5 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz").chars().collect();
        let test_6 = String::from("nppdvjthqldpwncqszvftbrmjlhg").chars().collect();
        let test_10 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").chars().collect();
        let test_11 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").chars().collect();
        assert_eq!(7, find_start_of_packet(&test_7));
        assert_eq!(5, find_start_of_packet(&test_5));
        assert_eq!(6, find_start_of_packet(&test_6));
        assert_eq!(10, find_start_of_packet(&test_10));
        assert_eq!(11, find_start_of_packet(&test_11));
    }
}
