advent_of_code::solution!(12);

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn origin() -> Self {
        Self { x: 0, y: 0 }
    }

    fn diff(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    fn init(x: usize, y: usize) -> Self {
        Self {
            x: x as isize,
            y: y as isize,
        }
    }

    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos {
                x: self.x - 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y - 1,
            },
            Pos {
                x: self.x + 1,
                y: self.y,
            },
            Pos {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Region {
    c: char,
    ps: Vec<Pos>,
}

impl Region {
    fn init(c: char, p: Pos) -> Self {
        Self { c, ps: vec![p] }
    }

    fn fill(&mut self, remains: &mut Vec<(char, Pos)>) {
        if remains.is_empty() {
            return;
        }
        let mut found: bool = false;
        for i in 0..remains.len() {
            if remains[i].0 == self.c
                && remains[i].1.neighbors().iter().any(|p| self.ps.contains(p))
            {
                let (_, p) = remains.remove(i);
                self.ps.push(p);
                found = true;
                break;
            }
        }
        if found {
            self.fill(remains);
        }
    }

    fn area(&self) -> usize {
        self.ps.len()
    }

    fn perimeter(&self) -> usize {
        self.ps
            .clone()
            .into_iter()
            .flat_map(|p| p.neighbors())
            .filter(|p| !self.ps.contains(p))
            .collect::<Vec<_>>()
            .len()
    }

    fn sides(&self) -> usize {
        // Count sides or count angles...

        // Outward:

        // .X
        // XAA <- 2 neighbors are missing from region -> 1 angle
        // .AA
        // XAX <- 3 neighbors are missing from region -> 2 angles
        // .X.

        // .A.
        // XAX <- 2 neighbors are missing BUT coords cancel out -> 0 angles
        // .A.

        // .X.
        // XAX <- 4 neighbors missing -> 4 angles ...
        // .X.

        // Inward:

        // .XA <- same foreign neighbor for 2 pieces -> 1 angle
        // AAA

        // AXA <- same foreign neighbor for 3 pieces -> 2 angles
        // AAA

        // AAA
        // AXA <- same foreign neighbor for 4 pieces -> 4 angles
        // AAA

        // println!("Region {}", self.c);

        let data: Vec<(Pos, Vec<Pos>)> = self
            .ps
            .clone()
            .into_iter()
            .map(|p| {
                (
                    p.clone(),
                    p.neighbors()
                        .into_iter()
                        .filter(|pn| !self.ps.contains(pn))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();

        let mut neighbors_map: HashMap<Pos, Vec<Pos>> = HashMap::new();
        let mut outward_angles = 0;
        data.iter().for_each(|(p, ps)| {
            let l = match ps.len() {
                2 => {
                    match ps
                        .iter()
                        .fold(Pos::origin(), |acc, p2| acc.add(&p.diff(p2)))
                    {
                        Pos { x: 0, y: 0 } => 0,
                        _ => 1,
                    }
                }
                3 => 2,
                4 => 4,
                _ => 0,
            };
            // let neighbors_vec = ps
            //     .iter()
            //     .fold(Pos::origin(), |acc, p2| acc.add(&p.diff(p2)));
            // println!("  * Outward angle(s) around: {:?} -> {}", p, l);
            // println!("     Neighbors vec: {:?}", neighbors_vec);
            outward_angles += l;
            for pn in ps {
                neighbors_map.entry(pn.clone()).or_default().push(p.clone());
            }
        });
        // println!("  Outward Angles: {}", outward_angles);
        let inward_angles = neighbors_map
            .values()
            .map(|ps| match ps.len() {
                2 => 1,
                3 => 2,
                4 => 4,
                _ => 0,
            })
            .sum::<usize>();
        // println!("Region {:} -> inward angles: {}, outward angles: {}", self.c, inward_angles, outward_angles);
        // println!("   -> sides: {}", inward_angles + outward_angles);
        inward_angles + outward_angles
    }
}

// fn count_sides(path: &mut Vec<Pos>) -> usize {
//     if path.len() <= 1 {
//         return 0;
//     }

//     let start = path.remove(0);
//     let directions = [
//         Pos { x: 0, y: 1 },  // up
//         Pos { x: 1, y: 0 },  // right
//         Pos { x: 0, y: -1 }, // down
//         Pos { x: -1, y: 0 }, // left
//     ];
//     let steps = successors(Some((start, 1, None)), |(cur, sides, d_idx)| {
//         // println!("Current: {:?}", cur);
//         if path.is_empty() {
//             return None;
//         }
//         for i in 0..4 {
//             let dir = directions[(d_idx.unwrap_or(0) + i) % 4].clone();
//             let next = cur.add(&dir);
//             if let Some(ofs) = path.iter().position(|p| p == &next) {
//                 // println!("  Trying direction {:?} -> {:?} (Found!)", dir, next);
//                 let new_cur = path.remove(ofs);
//                 let new_sides = sides
//                     + if d_idx.is_some() && i > 0 {
//                         // println!("(new direction, incrementing sides: {:?})", sides + 1);
//                         1
//                     } else {
//                         // println!("(same direction, sides: {:?})", sides);
//                         0
//                     };
//                 return Some((new_cur, new_sides, Some(d_idx.unwrap_or(0) + i)));
//                 // } else {
//                 // println!("  Trying direction {:?} -> {:?} (Not found)", dir, next);
//             }
//         }
//         let new_cur = path.remove(0);
//         // println!("No consecutive neighbor found -> Trying new side (sides: {})", sides + 1);
//         Some((new_cur, sides + 1, None))
//     })
//     .collect::<Vec<_>>();
//     steps.last().unwrap().1
// }

fn parse_input(input: &str) -> Vec<Region> {
    let mut res: Vec<Region> = Vec::new();
    let mut pcs = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (c, Pos::init(x, y)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    // pcs.sort();
    while !pcs.is_empty() {
        let (c, p) = pcs.remove(0);
        let mut r: Region = Region::init(c, p);
        r.fill(&mut pcs); // Should filter only by relevant letters
        res.push(r);
    }
    res
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .iter()
        .map(|r| r.area() * r.perimeter())
        .sum::<usize>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    // println!("{}", input);
    parse_input(input)
        .iter()
        .map(|r| r.area() * r.sides())
        .sum::<usize>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
        // assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
        // assert_eq!(result, Some(80));
    }
}

// A region of R plants with price 12 * 10 = 120.    => KO 12 * 11
// A region of I plants with price 4 * 4 = 16.       => OK
// A region of C plants with price 14 * 22 = 308.    => OK
// A region of F plants with price 10 * 12 = 120.    => KO 10 * 13
// A region of V plants with price 13 * 10 = 130.    => KO 13 * 11
// A region of J plants with price 11 * 12 = 132.    => OK
// A region of C plants with price 1 * 4 = 4.        => OK
// A region of E plants with price 13 * 8 = 104.     => KO 13 * 9
// A region of I plants with price 14 * 16 = 224.    => OK
// A region of M plants with price 5 * 6 = 30.       => KO 5 * 7
// A region of S plants with price 3 * 6 = 18.       => OK
