advent_of_code::solution!(13);

use std::num::ParseIntError;

#[derive(Debug)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn from_str(x: &str, y: &str) -> Result<Pos, ParseIntError> {
        let x = x.strip_suffix(",").unwrap_or(x)[2..].parse::<i32>()?;
        let y = y.strip_suffix(",").unwrap_or(y)[2..].parse::<i32>()?;
        Ok(Pos { x, y })
    }
}

#[derive(Debug)]
struct ClawMachine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl ClawMachine {
    // 8400 = 94a + 22b
    // 5400 = 34a + 67b
    fn solve(&self) -> Option<(i32, i32)> {
        let Pos { x: a1, y: a2 } = self.a;
        let Pos { x: b1, y: b2 } = self.b;
        let Pos { x: c1, y: c2 } = self.prize;

        // a1 x + b1 y = c1
        // a2 x + b2 y = c2

        // (b2 * a1 * x) + (b2 * b1 * y) = (c1 * b2)
        // (b1 * a2 * x) + (b1 * b2 * y) = (c2 * b1)
        // (c1 * b2) - (c2 * b1) = (b2 * a1 * x) - (b1 * a2 * x)
        //                 ...   = ((b2 * a1) - (b1 * a2)) * x
        // x = (c... ) / (a...)

        let x = ((c1 * b2) - (c2 * b1)) / ((b2 * a1) - (b1 * a2));
        let y = (c1 - (a1 * x)) / b1;

        if self.a.x * x + self.b.x * y == self.prize.x
            && self.a.y * x + self.b.y * y == self.prize.y
        {
            Some((x, y))
        } else {
            None
        }
    }

    fn solve_p2(&self) -> Option<(i64, i64)> {
        let factor: i64 = 10000000000000;

        let Pos { x: a1, y: a2 } = self.a;
        let Pos { x: b1, y: b2 } = self.b;
        let Pos { x: c1, y: c2 } = self.prize;

        let a1: i64 = a1 as i64;
        let a2: i64 = a2 as i64;
        let b1: i64 = b1 as i64;
        let b2: i64 = b2 as i64;
        let c1: i64 = (c1 as i64) + factor;
        let c2: i64 = (c2 as i64) + factor;

        let x = ((c1 * b2) - (c2 * b1)) / ((b2 * a1) - (b1 * a2));
        let y = (c1 - (a1 * x)) / b1;

        if (a1 * x + b1 * y == c1) && (a2 * x + b2 * y == c2) {
            Some((x, y))
        } else {
            None
        }
    }

    fn from_str(input: Vec<&str>) -> Result<Self, ()> {
        let mut pos: Vec<Pos> = input
            .iter()
            .map(
                |l| match l.split_whitespace().collect::<Vec<_>>().as_slice() {
                    ["Button", _, x, y] => Pos::from_str(x, y).map_err(|_| ()),
                    ["Prize:", x, y] => Pos::from_str(x, y).map_err(|_| ()),
                    _ => Err(()),
                },
            )
            .collect::<Result<Vec<Pos>, ()>>()?;
        Ok(ClawMachine {
            a: pos.remove(0),
            b: pos.remove(0),
            prize: pos.remove(0),
        })
    }
}

fn parse_input(input: &str) -> Result<Vec<ClawMachine>, ()> {
    input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .fold((Vec::new(), Vec::new()), |(mut acc, mut res), line| {
            if line.is_empty() {
                if acc.len() == 3 {
                    res.push(ClawMachine::from_str(acc));
                    (Vec::new(), res)
                } else {
                    panic!("invalid input, expecting sections of 3 lines");
                }
            } else {
                acc.push(line);
                (acc, res)
            }
        })
        .1
        .into_iter()
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_input(input)
        .ok()?
        .into_iter()
        .flat_map(|m| m.solve())
        .map(|(a, b)| (a as u32) * 3 + (b as u32))
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    parse_input(input)
        .ok()?
        .into_iter()
        .flat_map(|m| m.solve_p2())
        .map(|(a, b)| (a as u64) * 3 + (b as u64))
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }
}
