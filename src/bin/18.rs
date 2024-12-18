advent_of_code::solution!(18);

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn init(x: u8, y: u8) -> Self {
        Self { x, y }
    }
    fn neighbors(&self, max: &Pos) -> Vec<Pos> {
        let mut res = Vec::new();
        if self.x > 0 {
            res.push(Pos {
                x: self.x - 1,
                ..self.clone()
            });
        }
        if self.y > 0 {
            res.push(Pos {
                y: self.y - 1,
                ..self.clone()
            });
        }
        if self.x < max.x {
            res.push(Pos {
                x: self.x + 1,
                ..self.clone()
            });
        }
        if self.y < max.y {
            res.push(Pos {
                y: self.y + 1,
                ..self.clone()
            });
        }
        res
    }
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                let mut v: Vec<_> = l.split(",").map(|n| n.parse::<u8>().unwrap()).collect();
                if v.len() < 2 {
                    return None;
                }
                Some(Pos::init(v.remove(0), v.remove(0)))
            }
        })
        .collect()
}

fn bfs(walls: &HashSet<Pos>, start: Pos, end: &Pos) -> Option<Vec<Pos>> {
    let mut queue: VecDeque<Pos> = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut parent_map: HashMap<Pos, Pos> = HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start);
    while let Some(current) = queue.pop_front() {
        if current == *end {
            let mut path = vec![];
            let mut parent = current.clone();
            while let Some(p) = parent_map.get(&parent) {
                path.push(p.clone());
                parent = p.clone();
            }
            path.reverse();
            return Some(path);
        }
        for n in current.neighbors(end) {
            if visited.contains(&n) {
                continue;
            }
            if walls.contains(&n) {
                continue;
            }
            queue.push_back(n.clone());
            visited.insert(n.clone());
            parent_map.insert(n, current.clone());
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<usize> {
    let coords = parse_input(input);
    let (limit, end) = if coords.len() < 100 {
        (12, Pos::init(6, 6))
    } else {
        (1024, Pos::init(70, 70))
    };
    let walls: HashSet<Pos> = coords.into_iter().take(limit).collect();
    bfs(&walls, Pos::init(0, 0), &end).map(|r| r.len())
}

fn get_first_failure(coords: &[Pos], end: &Pos) -> usize {
    let mut last_success = 0;
    let mut first_failure = coords.len();

    while last_success < first_failure - 1 {
        let limit = last_success + (first_failure - last_success) / 2;
        // println!(" {} <~~ {}? ~~> {}", last_success, limit, first_failure);
        let walls: HashSet<Pos> = coords.iter().take(limit + 1).cloned().collect();

        if bfs(&walls, Pos::init(0, 0), end).is_some() {
            // println!("        -> Success");
            last_success = limit;
        } else {
            // println!("        -> Failure");
            first_failure = limit;
        }
    }
    first_failure
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = parse_input(input);
    let end = if coords.len() < 100 {
        Pos::init(6, 6)
    } else {
        Pos::init(70, 70)
    };
    let first_failure = get_first_failure(&coords, &end);
    coords.get(first_failure).unwrap().to_string().into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6,1".into()));
    }
}
