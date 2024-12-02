advent_of_code::solution!(2);

use std::cmp::Ordering;

fn validate_input_p1(input: &[u32]) -> bool {
    input
        .iter()
        .fold(
            (true, 0, None),
            |(is_good, last, ord): (bool, u32, Option<Ordering>), cur: &u32| match (
                is_good, last, cur, ord,
            ) {
                (false, _, _, _) => (false, 0, None),
                (_, 0, cur, None) => (true, *cur, None),
                (_, last, cur, None) if last.abs_diff(*cur) < 4 => {
                    (true, *cur, Some(last.cmp(cur)))
                }
                (_, last, cur, Some(cmp)) if last.abs_diff(*cur) < 4 && last.cmp(cur) == cmp => {
                    (true, *cur, Some(cmp))
                }
                _ => (false, 0, None),
            },
        )
        .0
}

fn validate_input_p2(input: &Vec<u32>) -> bool {
    if validate_input_p1(input) {
        return true;
    }

    for i in 0..input.len() {
        let mut input2 = input.to_owned();
        input2.remove(i);
        if validate_input_p1(&input2) {
            return true;
        }
    }

    false
}

pub fn part_one(input: &str) -> Option<usize> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|nb| nb.parse::<u32>().expect("parsing failed"))
                .collect::<Vec<u32>>()
        })
        .filter(|s| validate_input_p1(s))
        .count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|nb| nb.parse::<u32>().expect("parsing failed"))
                .collect::<Vec<u32>>()
        })
        .filter(validate_input_p2)
        .count()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
