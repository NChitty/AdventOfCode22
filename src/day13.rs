use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
enum PacketData {
    List(Vec<PacketData>),
    Item(isize),
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (PacketData::Item(lhs), PacketData::Item(rhs)) => lhs.partial_cmp(&rhs),
            (PacketData::List(lhs), PacketData::List(rhs)) => lhs.partial_cmp(&rhs),
            (PacketData::Item(lhs), PacketData::List(rhs)) => {
                vec![PacketData::Item(*lhs)].partial_cmp(&rhs)
            }
            (PacketData::List(lhs), PacketData::Item(rhs)) => {
                lhs.partial_cmp(&vec![PacketData::Item(*rhs)])
            }
        }
    }
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PacketData::Item(lhs), PacketData::Item(rhs)) => lhs.cmp(&rhs),
            (PacketData::List(lhs), PacketData::List(rhs)) => lhs.cmp(&rhs),
            (PacketData::Item(lhs), PacketData::List(rhs)) => {
                vec![PacketData::Item(*lhs)].cmp(&rhs)
            }
            (PacketData::List(lhs), PacketData::Item(rhs)) => {
                lhs.cmp(&vec![PacketData::Item(*rhs)])
            }
        }
    }
}

fn find_matching_bracket(s: &str, open_index: usize) -> Option<usize> {
    let mut depth = 0;

    for (i, c) in s.chars().skip(open_index).enumerate() {
        match c {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }

    None
}

impl FromStr for PacketData {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data: Vec<PacketData> = Vec::new();
        let mut depth = 0;
        let mut chars = s.chars().enumerate();
        while let Some((i, char)) = chars.next() {
            match char {
                ']' => {
                    break;
                }
                '[' => {
                    if depth == 0 {
                        depth += 1;
                        continue;
                    }
                    let close_index =
                        find_matching_bracket(&s, i).ok_or("No matching close bracket.")?;
                    let parsed_list = s[i..(close_index + i + 1)].parse()?;
                    data.push(parsed_list);
                    let _ = chars
                        .nth(close_index)
                        .ok_or("Skipped past last character.")?;
                }
                digit if digit.is_ascii_digit() => {
                    let mut num_str = String::new();
                    num_str.push(digit);

                    while let Some((_, next_char)) = chars.next() {
                        if next_char.is_ascii_digit() {
                            num_str.push(next_char);
                        } else {
                            break;
                        }
                    }

                    let number = num_str.parse().map_err(|_| "Failed to parse number.")?;
                    data.push(PacketData::Item(number));
                }
                ',' => {}
                ' ' => {}
                _ => return Err("Illegal character in packet.".to_string()),
            }
        }
        Ok(Self::List(data))
    }
}

#[aoc_generator(day13, part1)]
fn parse_input_as_pairs(input: &str) -> Vec<(PacketData, PacketData)> {
    let mut packet_pairs = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        let b = lines.next().expect("No next.");
        let packet_pair = (
            line.parse().expect("Failure to parse"),
            b.parse().expect("Failure to parse"),
        );
        packet_pairs.push(packet_pair);
    }

    packet_pairs
}

#[aoc_generator(day13, part2)]
fn parse_input(input: &str) -> Vec<PacketData> {
    let mut packets = vec![
        PacketData::List(vec![PacketData::List(vec![PacketData::Item(2)])]),
        PacketData::List(vec![PacketData::List(vec![PacketData::Item(6)])]),
    ];
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            continue;
        }
        packets.push(line.parse().expect("Failure to parse"));
    }

    packets
}

#[aoc(day13, part1)]
fn sum_correct_indices(packet_pairs: &[(PacketData, PacketData)]) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .filter(|(_, ref pair)| pair.0 <= pair.1)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn multiply_decoder_packets(packets: &[PacketData]) -> usize {
    let mut packets_vec = packets.to_vec();
    packets_vec.sort();
    packets_vec
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            **item == PacketData::List(vec![PacketData::List(vec![PacketData::Item(2)])])
                || **item == PacketData::List(vec![PacketData::List(vec![PacketData::Item(6)])])
        })
        .map(|(i, _)| i + 1)
        .product()
}

#[cfg(test)]
mod test {
    use crate::day13::{multiply_decoder_packets, parse_input};

    use super::{parse_input_as_pairs, sum_correct_indices, PacketData};

    #[test]
    fn both_ints_correct() {
        let left = PacketData::Item(1);
        let right = PacketData::Item(2);
        assert!(left < right);
    }

    #[test]
    fn both_ints_wrong() {
        let left = PacketData::Item(2);
        let right = PacketData::Item(1);
        assert!(left > right);
    }

    #[test]
    fn same_sized_list_correct() {
        let left = PacketData::List(vec![PacketData::Item(1)]);
        let right = PacketData::List(vec![PacketData::Item(1)]);
        assert!(left == right);
    }

    #[test]
    fn conversion() {
        let left = PacketData::List(vec![
            PacketData::Item(2),
            PacketData::Item(3),
            PacketData::Item(4),
        ]);
        let right = PacketData::Item(4);
        assert!(left < right);
    }

    #[test]
    fn nested() {
        let left = PacketData::List(vec![PacketData::Item(9)]);
        let right = PacketData::List(vec![PacketData::List(vec![
            PacketData::Item(8),
            PacketData::Item(7),
            PacketData::Item(6),
        ])]);
        assert!(left > right);
    }

    #[test]
    fn complex_nest() {
        let left = PacketData::List(vec![PacketData::List(vec![PacketData::List(vec![])])]);
        let right = PacketData::List(vec![PacketData::List(vec![])]);
        assert!(left > right);
    }

    #[test]
    fn parse_packet() {
        let expected = PacketData::List(vec![
            PacketData::List(vec![PacketData::Item(1)]),
            PacketData::List(vec![
                PacketData::Item(2),
                PacketData::Item(3),
                PacketData::Item(4),
            ]),
        ]);
        let actual = "[[1],[2,3,4]]".parse().expect("Could not parse.");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_empty_packet() {
        let expected = PacketData::List(vec![PacketData::List(vec![PacketData::List(vec![])])]);
        let actual = "[[[]]]".parse().expect("Could not parse.");
        assert_eq!(expected, actual);
    }
    const SAMPLE_TEXT_PART_1: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn sample_sum_part1() {
        let pairs = parse_input_as_pairs(SAMPLE_TEXT_PART_1);
        let expected = 13;
        let actual = sum_correct_indices(&pairs);
        assert!(expected == actual);
    }

    #[test]
    fn sample_product() {
        let pairs = parse_input(SAMPLE_TEXT_PART_1);
        let expected = 140;
        let actual = multiply_decoder_packets(&pairs);
        assert!(expected == actual);
    }
}
