advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    re.captures_iter(input)
        .map(|c| {
            let (_, [a, b]) = c.extract();
            a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()
        })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"(?:(mul)\((\d+),(\d+)\))|(?:(do)(\()(\)))|(?:(don't)(\()(\)))").unwrap();
    re.captures_iter(input)
        .map(|c| match c.extract() {
            (_, ["mul", a, b]) => (None, a.parse::<u32>().unwrap() * b.parse::<u32>().unwrap()),
            (_, ["do", _, _]) => (Some(true), 0),
            (_, ["don't", _, _]) => (Some(false), 0),
            _ => panic!("invalid parsing"),
        })
        .fold((true, 0), |(flag, acc), (op_flag, op_res)| {
            match (flag, acc, op_flag, op_res) {
                (_, acc, Some(new_flag), _) => (new_flag, acc),
                (false, acc, None, _) => (false, acc),
                (true, acc, None, new) => (true, acc + new),
            }
        })
        .1
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
