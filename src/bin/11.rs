advent_of_code::solution!(11);

use memoize::memoize;

#[memoize]
fn get_child_stones_count(stone: u64, depth: u8) -> usize {
    if depth == 0 {
        return 1;
    }
    let digits = stone.checked_ilog10().unwrap_or(0) + 1;
    match (stone, digits) {
        (0, _) => get_child_stones_count(1, depth - 1),
        (n, digits) if digits % 2 == 0 && digits > 0 => {
            let div = 10u64.pow(digits / 2);
            get_child_stones_count(n / div, depth - 1) + get_child_stones_count(n % div, depth - 1)
        }
        _ => get_child_stones_count(stone * 2024, depth - 1),
    }
}

// fn update_stones(stones: &Vec<u64>) -> Vec<u64> {
//     stones.into_iter().flat_map(|st|
//         match (st, st.checked_ilog10().unwrap_or(0) + 1) {
//             (0, _) => vec![1],
//             (n, digits) if digits % 2 == 0 && digits > 0 => {
//                 let div = 10u64.pow(digits / 2);
//                 vec![
//                     n / div,
//                     n % div,
//                 ]
//             },
//             _ => vec![st * 2024],
//         }
//     ).collect()
// }

fn parse_input(input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    input.split_whitespace().map(|c| c.parse::<u64>()).collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .ok()?
        .into_iter()
        .map(|st| get_child_stones_count(st, 25))
        .sum::<usize>()
        .into()
    // successors(Some(stones), |st| Some(update_stones(st)))
    //     .nth(25)
    //     .unwrap()
    //     .len()
    //     .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    parse_input(input)
        .ok()?
        .into_iter()
        .map(|st| get_child_stones_count(st, 75))
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one("125 17");
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
