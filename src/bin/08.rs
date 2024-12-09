advent_of_code::solution!(8);

use itertools::Itertools;
use std::collections::HashMap;
use std::iter::successors;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i8,
    y: i8,
}

impl Pos {
    fn init(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    fn diff(self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn is_valid(&self, max: &Self) -> bool {
        self.x >= 0 && self.x < max.x && self.y >= 0 && self.y < max.y
    }
}

fn get_antinodes(p1: &Pos, p2: &Pos, max: &Pos) -> Vec<Pos> {
    let d1 = p1.diff(*p2);
    let d2 = p2.diff(*p1);
    let p1b = p1.add(d2);
    let p2b = p2.add(d1);
    let mut res: Vec<Pos> = Vec::new();
    if p1b.is_valid(max) {
        res.push(p1b);
    }
    if p2b.is_valid(max) {
        res.push(p2b);
    }
    res
}

fn get_antinodes_p2(p1: Pos, p2: Pos, max: &Pos) -> Vec<Pos> {
    let d1 = p2.diff(p1);
    let mut succ1: Vec<Pos> = successors(Some(p1), |p| {
        let s = p.add(d1);
        if s.is_valid(max) {
            Some(s)
        } else {
            None
        }
    })
    .collect();
    let d2 = p1.diff(p2);
    let succ2: Vec<Pos> = successors(Some(p2), |p| {
        let s = p.add(d2);
        if s.is_valid(max) {
            Some(s)
        } else {
            None
        }
    })
    .collect();
    succ1.extend(succ2);
    succ1.into_iter().map(|p| p.to_owned()).collect()
}

fn parse_line(line: &str, y: usize) -> Vec<(char, Pos)> {
    line.chars()
        .enumerate()
        .filter_map(|(x, c)| match c {
            '.' => None,
            c => Some((c, Pos::init(x as i8, y as i8))),
        })
        .collect()
}

// fn print_map(size: &Pos, antennas: &HashMap<char, Vec<Pos>>, antinodes: &Vec<Pos>) {
//     for y in 0..size.y {
//         for x in 0..size.x {
//             let p = Pos::init(x, y);
//             let mut found = false;
//             for (c, ps) in antennas.iter() {
//                 if ps.contains(&p) {
//                     print!("{}", c);
//                     found = true;
//                     break;
//                 }
//             }
//             if found == false {
//                 if antinodes.contains(&p) {
//                     print!("#");
//                 } else {
//                     print!(".");
//                 }
//             }
//         }
//         println!();
//     }
// }

fn parse_input(input: &str) -> (Pos, HashMap<char, Vec<Pos>>) {
    let mut antenna: HashMap<char, Vec<Pos>> = HashMap::new();
    let lines = input.lines().collect::<Vec<_>>();
    let map_size = Pos::init(lines[0].len() as i8, lines.len() as i8);
    lines
        .into_iter()
        .enumerate()
        .flat_map(|(y, l)| parse_line(l, y))
        .for_each(|(c, p)| {
            antenna.entry(c).or_default().push(p);
        });
    (map_size, antenna)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (map_size, antenna) = parse_input(input);
    let mut antinodes: Vec<Pos> = antenna
        .values()
        .flat_map(|ps| {
            ps.iter()
                .combinations(2)
                .flat_map(|a| get_antinodes(a[0], a[1], &map_size))
                .collect::<Vec<_>>()
        })
        .collect();
    antinodes.sort();
    antinodes.dedup();
    Some(antinodes.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map_size, antenna) = parse_input(input);
    let mut antinodes: Vec<Pos> = antenna
        .values()
        .flat_map(|ps| {
            ps.iter()
                .combinations(2)
                .flat_map(|a| get_antinodes_p2(*a[0], *a[1], &map_size))
                .collect::<Vec<_>>()
        })
        .collect();
    antinodes.sort();
    antinodes.dedup();
    Some(antinodes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
