advent_of_code::solution!(7);

use itertools::{repeat_n, Itertools};

#[derive(Debug)]
struct Calib {
    res: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Ops {
    Add,
    Mul,
    Concat,
}

#[derive(Debug)]
enum Equ {
    Nb(u64),
    Op(u64, Ops, Box<Equ>),
}

impl Equ {
    fn init(nbs: &[u64], ops: &[Ops]) -> Equ {
        match (nbs, ops) {
            ([h], []) => Equ::Nb(*h),
            ([h, t @ ..], [op, top @ ..]) => Equ::Op(*h, *op, Box::new(Equ::init(t, top))),
            _ => todo!(),
        }
    }

    fn add(&mut self, lhs: u64) -> u64 {
        match self {
            Equ::Nb(x) => return *x + lhs,
            Equ::Op(llhs, _, _) => *llhs += lhs,
        }
        self.solve()
    }

    fn mul(&mut self, lhs: u64) -> u64 {
        match self {
            Equ::Nb(x) => return *x * lhs,
            Equ::Op(llhs, _, _) => *llhs *= lhs,
        }
        self.solve()
    }

    fn concat(&mut self, lhs: u64) -> u64 {
        match self {
            Equ::Nb(x) => return format!("{}{}", lhs, x).parse::<u64>().unwrap(),
            Equ::Op(x, _, _) => *x = format!("{}{}", lhs, x).parse::<u64>().unwrap(),
        }
        self.solve()
    }

    fn solve(&mut self) -> u64 {
        match self {
            Equ::Nb(x) => *x,
            Equ::Op(lhs, Ops::Add, rhs) => rhs.add(*lhs),
            Equ::Op(lhs, Ops::Mul, rhs) => rhs.mul(*lhs),
            Equ::Op(lhs, Ops::Concat, rhs) => rhs.concat(*lhs),
        }
    }
}

impl Ops {
    fn possible_ops_p1(l: usize) -> Vec<Vec<Ops>> {
        repeat_n(vec![Ops::Add, Ops::Mul].into_iter(), l - 1)
            .multi_cartesian_product()
            .collect()
    }
    fn possible_ops_p2(l: usize) -> Vec<Vec<Ops>> {
        repeat_n(vec![Ops::Add, Ops::Mul, Ops::Concat].into_iter(), l - 1)
            .multi_cartesian_product()
            .collect()
    }
}

impl TryFrom<&str> for Calib {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut numbers: Vec<u64> = input
            .chars()
            .filter(|c| *c != ':')
            .collect::<String>()
            .split_whitespace()
            .flat_map(|n| n.parse::<u64>())
            .collect();
        let res = numbers.remove(0);
        Ok(Self { res, numbers })
    }
}

impl Calib {
    fn check_ops(&self, ops: Vec<Ops>) -> bool {
        let nbs = self.numbers.clone();
        let mut eq = Equ::init(nbs.as_slice(), ops.as_slice());
        let res = eq.solve();
        res == self.res
    }

    fn solve_p1(&self) -> bool {
        Ops::possible_ops_p1(self.numbers.len())
            .into_iter()
            .any(|ops| self.check_ops(ops))
    }
    fn solve_p2(&self) -> bool {
        Ops::possible_ops_p2(self.numbers.len())
            .into_iter()
            .any(|ops| self.check_ops(ops))
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .flat_map(Calib::try_from)
        .filter_map(|c| if c.solve_p1() { Some(c.res) } else { None })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .flat_map(Calib::try_from)
        .filter_map(|c| if c.solve_p2() { Some(c.res) } else { None })
        .sum::<u64>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
