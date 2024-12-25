advent_of_code::solution!(20);

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn init(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn neighbors(&self) -> Vec<Pos> {
        let mut res: Vec<Pos> = Vec::new();
        if self.x > 0 {
            res.push(Pos::init(self.x - 1, self.y));
        }
        if self.y > 0 {
            res.push(Pos::init(self.x, self.y - 1));
        }
        res.push(Pos::init(self.x + 1, self.y));
        res.push(Pos::init(self.x, self.y + 1));
        res
    }
}

#[derive(Debug, Clone)]
struct PosState {
    c: char,
    dist: Option<usize>,
}

impl From<char> for PosState {
    fn from(input: char) -> PosState {
        PosState {
            c: input,
            dist: None,
        }
    }
}

#[derive(Debug)]
struct State {
    m: HashMap<Pos, PosState>,
    end: Pos,
    start: Pos,
}

impl State {
    fn new() -> State {
        State {
            m: HashMap::new(),
            end: Pos::init(0, 0),
            start: Pos::init(0, 0),
        }
    }

    fn update(&self, p: Pos, ps: PosState) -> State {
        let end = if ps.c == 'E' {
            p.clone()
        } else {
            self.end.clone()
        };
        let start = if ps.c == 'S' {
            p.clone()
        } else {
            self.start.clone()
        };
        let mut m = self.m.clone();
        m.insert(p, ps);

        State { m, end, start }
    }

    fn fill_distances(&mut self, current: Vec<Pos>, dist: usize) {
        if current.is_empty() {
            return;
        }
        let mut next: Vec<Pos> = Vec::new();
        for cur in &current {
            let mut add = false;
            self.m.entry(cur.clone()).and_modify(|e| {
                if e.dist.is_none() {
                    e.dist = Some(dist);
                    if e.c != '#' {
                        add = true;
                    }
                }
            });
            if add {
                for n in cur.neighbors() {
                    if self.m.contains_key(&n) {
                        next.push(n);
                    }
                }
            }
        }
        next.sort();
        next.dedup();
        self.fill_distances(next, dist + 1);
    }

    // fn print(&self) {
    //     for y in 0..15 {
    //         for x in 0..15 {
    //             let p = Pos::init(x, y);
    //             if let Some(e) = self.m.get(&p) {
    //                 print!("{}{}{}{}{}{}", e.c, e.c, e.c, e.c, e.c, e.c);
    //             }
    //         }
    //         println!();
    //         for x in 0..15 {
    //             let p = Pos::init(x, y);
    //             if let Some(e) = self.m.get(&p) {
    //                 if let Some(d) = e.dist {
    //                     print!("{}{:^4}{}", e.c, d, e.c);
    //                 } else {
    //                     print!("{}{}{}{}{}{}", e.c, e.c, e.c, e.c, e.c, e.c);
    //                 }
    //             }
    //         }
    //         println!();
    //         for x in 0..15 {
    //             let p = Pos::init(x, y);
    //             if let Some(e) = self.m.get(&p) {
    //                 print!("{}{}{}{}{}{}", e.c, e.c, e.c, e.c, e.c, e.c);
    //             }
    //         }
    //         println!();
    //     }
    // }

    fn get_time_saving_cheats(&self) -> HashMap<usize, usize> {
        let mut cheats: HashMap<usize, usize> = HashMap::new();

        for (p, ps) in self.m.iter() {
            if ps.c == '#' {
                continue;
            }
            for n in p.neighbors() {
                if let Some(ns) = self.m.get(&n) {
                    if ns.c == '#' && ns.dist.is_some() && ns.dist < ps.dist {
                        let cheat_save =
                            (ps.dist.unwrap() as isize) - (ns.dist.unwrap() as isize) - 1isize;
                        if cheat_save > 0 {
                            *cheats.entry(cheat_save as usize).or_default() += 1;
                        }
                    }
                }
            }
        }
        cheats
    }
}

fn parse_input(input: &str) -> State {
    let mut state = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| (Pos::init(x, y), PosState::from(c)))
                .collect::<Vec<(Pos, PosState)>>()
        })
        .fold(State::new(), |st, (p, ps)| st.update(p, ps));
    state.fill_distances(vec![state.end.clone()], 0);
    state
}

pub fn part_one(input: &str) -> Option<usize> {
    parse_input(input)
        .get_time_saving_cheats()
        .iter()
        .filter_map(|(k, v)| if *k >= 100 { Some(v) } else { None })
        .sum::<usize>()
        .into()
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
