use core::panic;
use std::u8;

#[derive(Default)]
struct Node {
    height: u8,
    up: Option<usize>,
    down: Option<usize>,
    left: Option<usize>,
    right: Option<usize>,
}

fn to_height(character: char) -> u8 {
    if character.is_ascii_lowercase() {
        return ((character as u32 - 'a' as u32) + 1).try_into().unwrap();
    }
    match character {
        'S' => 0,
        'E' => 27,
        _ => panic!("Nothing else in input space")
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Vec<Node> {
    let num_rows = input.lines().count();
    let num_cols = input.lines().next().unwrap().chars().count();
    let mut row = 0;
    let mut nodes = Vec::new();
    for line in input.lines() {
        let mut col = 0;
        for character in line.chars() {
            let height = to_height(character);
            let up = match row {
                0 => None,
                _ => Some((row - 1) * num_cols + col),
            };
            let down = match row + 1 {
                _ if row + 1 == num_rows => None,
                _ => Some((row + 1) * num_cols + col),
            };
            let left = match col {
                0 => None,
                _ => Some(row * num_cols + col - 1)
            };
            let right = match col + 1 {
                _ if col + 1 == num_cols => None,
                _ => Some(row * num_cols + col + 1)
            };
            let node = Node {
                height,
                up,
                down,
                left,
                right,
            };
            nodes.push(node);
            col += 1;
        }
        row += 1;
    }

    nodes
}

#[cfg(test)]
mod test {
    use super::to_height;

    #[test]
    fn to_height_lowercase() {
        assert_eq!(1, to_height('a'));
        assert_eq!(2, to_height('b'));
        assert_eq!(25, to_height('y'));
        assert_eq!(26, to_height('z'));
    }

    #[test]
    fn to_height_uppercase() {
        assert_eq!(0, to_height('S'));
        assert_eq!(27, to_height('E'));
    }
}
