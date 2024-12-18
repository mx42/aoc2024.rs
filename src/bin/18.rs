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
    // fn to_string(&self) -> String {
    //     format!("{},{}", self.x, self.y)
    // }
}

fn parse_input(input: &str) -> HashMap<Pos, usize> {
    input
        .lines()
        .enumerate()
        .filter_map(|(n, l)| {
            if l.is_empty() {
                None
            } else {
                let mut v: Vec<_> = l.split(",").map(|n| n.parse::<u8>().unwrap()).collect();
                if v.len() < 2 {
                    return None;
                }
                Some((Pos::init(v.remove(0), v.remove(0)), n))
            }
        })
        .collect()
}

fn bfs(walls: &HashMap<Pos, usize>, start: Pos, end: &Pos) -> Option<Vec<Pos>> {
    let mut queue: VecDeque<Pos> = VecDeque::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut parent_map: HashMap<Pos, Pos> = HashMap::new();

    let mut blockage_queue: Vec<Pos> = Vec::new();

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
            if walls.contains_key(&n) {
                if !blockage_queue.contains(&n) {
                    blockage_queue.push(n);
                }
                continue;
            }
            queue.push_back(n.clone());
            visited.insert(n.clone());
            parent_map.insert(n, current.clone());
        }
    }
    blockage_queue.sort_by_cached_key(|p| walls.get(p).unwrap());

    // Sort blockage queue by wall number ?!
    for new_start in blockage_queue {
        let mut queue: VecDeque<Pos> = VecDeque::new();
        let mut visited = visited.clone();
        queue.push_back(new_start.clone());
        while let Some(current) = queue.pop_front() {
            if current == *end {
                println!("Found end by removing {:?}!", new_start);
                println!("Wall at time: {}", walls.get(&new_start)?);
                break;
            }
            for n in current.neighbors(end) {
                if visited.contains(&n) {
                    continue;
                }
                if walls.contains_key(&n) {
                    continue;
                }
                queue.push_back(n.clone());
                visited.insert(n.clone());
            }
        }
    }

    None
}

fn print_map(size: &Pos, walls: &HashMap<Pos, usize>, path: &[Pos]) {
    for y in 0..=size.y {
        for x in 0..=size.x {
            let p = Pos::init(x, y);
            if path.contains(&p) {
                print!("O");
            } else if walls.contains_key(&p) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut coords = parse_input(input);
    let (limit, end) = if coords.len() < 100 {
        // Test case
        (12, Pos::init(6, 6))
    } else {
        (1024, Pos::init(70, 70))
    };
    coords.retain(|_, v| *v < limit);

    bfs(&coords, Pos::init(0, 0), &end).map(|r| r.len())
}

pub fn part_two(input: &str) -> Option<String> {
    let coords = parse_input(input);
    let end = if coords.len() < 100 {
        Pos::init(6, 6)
    } else {
        Pos::init(70, 70)
    };

    print_map(&end, &coords, &[]);

    let route = bfs(&coords, Pos::init(0, 0), &end);
    println!("{:?}", route);
    None
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
