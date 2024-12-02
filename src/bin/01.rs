advent_of_code::solution!(1);

use std::collections::HashMap;

fn parse_line(input: &str) -> (u32, u32) {
    let input = input
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<&str>>();
    let input = input.as_slice();
    match input {
        [a, b] => (
            a.parse::<u32>().expect("parse error"),
            b.parse::<u32>().expect("parse error"),
        ),
        _ => panic!("parse error!"),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right): (Vec<_>, Vec<_>) = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .map(parse_line)
        .unzip();
    left.sort();
    right.sort();
    left.into_iter()
        .zip(right)
        .map(|(a, b)| a.abs_diff(b))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right): (Vec<_>, Vec<_>) = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .map(parse_line)
        .unzip();
    let mut right_count: HashMap<u32, u8> = HashMap::new();
    right
        .iter()
        .for_each(|u| *right_count.entry(*u).or_default() += 1);
    left.into_iter()
        .map(|c| c * *right_count.entry(c).or_default() as u32)
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
