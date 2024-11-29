use itertools::{iterate, Itertools};

fn decrypt(
    data: &mut Vec<i64>,
    decryption_key: i64,
    num_mix: usize,
    prev_indices: &mut [usize],
    next_indices: &mut [usize],
) {
    data.iter_mut().for_each(|val| *val *= decryption_key);
    for _ in 0..num_mix {
        mix(data, prev_indices, next_indices);
    }
}

fn mix(data: &Vec<i64>, prev_indices: &mut [usize], next_indices: &mut [usize]) {
    for (cur, &n) in data.iter().enumerate() {
        fix_indices(
            prev_indices[cur],
            next_indices[cur],
            prev_indices,
            next_indices,
            cur,
        );

        let amount_to_move = n.rem_euclid(data.len() as i64 - 1) as usize;
        let target = find_target(
            prev_indices[cur],
            amount_to_move,
            prev_indices,
            next_indices,
        );

        prev_indices[cur] = target;
        fix_indices(
            cur,
            next_indices[target],
            prev_indices,
            next_indices,
            target,
        );
    }
}

fn fix_indices(
    left: usize,
    right: usize,
    prev_indices: &mut [usize],
    next_indices: &mut [usize],
    stop: usize,
) {
    let (first, last) = iterate(left, |&i| prev_indices[i])
        .zip(iterate(right, |&i| prev_indices[i]))
        .inspect(|&(before, after)| next_indices[before] = after)
        .find(|&(_, after)| prev_indices[after] == stop)
        .expect("Could not find stop index in indices.");
    prev_indices[last] = left;
    next_indices[prev_indices[first]] = left;
}

fn find_target(
    start: usize,
    amount_to_move: usize,
    prev_indices: &mut [usize],
    next_indices: &mut [usize],
) -> usize {
    let next_index = iterate(start, |&cur| next_indices[cur])
        .nth((amount_to_move + 1) / 1)
        .unwrap();
    iterate(next_index, |&cur| prev_indices[cur])
        .nth(1)
        .unwrap()
}

#[aoc_generator(day20)]
fn parse_list(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|line| line.parse().ok())
        .collect_vec()
}

#[aoc(day20, part1)]
fn decrypt_a(list: &[i64]) -> i64 {
    let data = list.to_vec();
    let mut prev_indices = (0..data.len()).collect_vec();
    let mut next_indices = prev_indices.clone();
    prev_indices.rotate_right(1);
    next_indices.rotate_left(1);
    mix(&data, &mut prev_indices, &mut next_indices);
    let zero_index = data
        .iter()
        .position(|&x| x == 0)
        .expect("No zero present in data.");
    iterate(zero_index, |&cur| {
        find_target(cur, 1000, &mut prev_indices, &mut next_indices)
    })
    .skip(1)
    .take(3)
    .map(|i| data[i])
    .sum()
}

#[aoc(day20, part2)]
fn decrypt_b(list: &[i64]) -> i64 {
    let mut data = list.to_vec();
    let mut prev_indices = (0..data.len()).collect_vec();
    let mut next_indices = prev_indices.clone();
    prev_indices.rotate_right(1);
    next_indices.rotate_left(1);
    decrypt(
        &mut data,
        811589153,
        10,
        &mut prev_indices,
        &mut next_indices,
    );
    let zero_index = data
        .iter()
        .position(|&x| x == 0)
        .expect("No zero present in data.");
    iterate(zero_index, |&cur| {
        find_target(cur, 1000, &mut prev_indices, &mut next_indices)
    })
    .skip(1)
    .take(3)
    .map(|i| data[i])
    .sum()
}

#[cfg(test)]
mod test {

    use super::{decrypt_a, decrypt_b, parse_list};

    const SAMPLE_INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn test_a() {
        let expected = 3;
        let actual = decrypt_a(&parse_list(SAMPLE_INPUT));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let expected = 1623178306;
        let actual = decrypt_b(&parse_list(SAMPLE_INPUT));
        assert_eq!(expected, actual);
    }
}
