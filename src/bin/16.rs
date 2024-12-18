advent_of_code::solution!(16);

use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn neighbors(&self) -> Vec<Step> {
        let mut res: Vec<Step> = Vec::new();
        if self.x > 0 {
            res.push(Step {
                pos: Pos {
                    x: self.x - 1,
                    y: self.y,
                },
                dir: Dir::Left,
            });
        }
        if self.y > 0 {
            res.push(Step {
                pos: Pos {
                    x: self.x,
                    y: self.y - 1,
                },
                dir: Dir::Up,
            });
        }
        res.push(Step {
            pos: Pos {
                x: self.x + 1,
                y: self.y,
            },
            dir: Dir::Right,
        });
        res.push(Step {
            pos: Pos {
                x: self.x,
                y: self.y + 1,
            },
            dir: Dir::Down,
        });
        res
    }
}

#[derive(Debug)]
struct State {
    map: HashMap<Pos, char>,
    start: Pos,
    // end: Pos,
    // size: Pos,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Step {
    pos: Pos,
    dir: Dir,
}

#[derive(Debug, Clone)]
struct Path {
    steps: VecDeque<Step>,
}

impl Path {
    fn init(pos: Pos) -> Self {
        Self {
            steps: VecDeque::from(vec![Step {
                pos,
                dir: Dir::Right,
            }]),
        }
    }

    fn current(&self) -> Pos {
        self.steps.back().unwrap().pos.clone()
    }
    fn have_visited(&self, pos: &Pos) -> bool {
        self.steps.iter().any(|s| s.pos == *pos)
    }
    fn push(&mut self, step: Step) {
        self.steps.push_back(step);
    }
    fn score(&self) -> u32 {
        let mut score = 0u32;
        let mut cur_dir: Option<Dir> = None;
        let mut steps = self.steps.clone();
        steps.pop_back();
        for step in &steps {
            if cur_dir.is_some() {
                if cur_dir == Some(step.dir) {
                    score += 1;
                } else {
                    score += 1001;
                }
            }
            cur_dir = Some(step.dir);
        }
        score + 1 // duh?!
    }
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(input: &str) -> Result<State, Self::Error> {
        // let size = Pos {
        //     x: input[0].len(),
        //     y: input.len(),
        // };
        // let input: Vec<(Pos, char)> = input
        let input: Vec<_> = input
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        None
                    } else {
                        Some((Pos { x, y }, c))
                    }
                })
            })
            .collect();
        let start = input.iter().find(|(_, c)| *c == 'S').ok_or(())?.0.clone();
        // let end = input.iter().find(|(_, c)| *c == 'E').ok_or(())?.0.clone();
        let mut map = HashMap::new();
        for (k, v) in input.into_iter() {
            map.insert(k, v);
        }
        Ok(State {
            map,
            start,
            // end,
            // size,
        })
    }
}

impl State {
    fn compute_all_paths(&self, path: Path, all_paths: &mut Vec<Path>) {
        let cur = path.current();
        if self.map.get(&cur) == Some(&'E') {
            println!("Finished a path");
            println!("Score {:?}", path.score());
            all_paths.push(path);
            return;
        }
        for next in cur
            .neighbors()
            .iter()
            .filter(|p| self.map.contains_key(&p.pos) && !path.have_visited(&p.pos))
        {
            let mut new_path = path.clone();
            new_path.push(next.clone());
            self.compute_all_paths(new_path, all_paths);
        }
    }

    fn get_paths(&self) -> Vec<Path> {
        let initial_path = Path::init(self.start.clone());
        let mut all_paths: Vec<Path> = Vec::new();
        self.compute_all_paths(initial_path, &mut all_paths);
        all_paths
    }

    // fn print_path(&self, path: &Path) {
    //     println!();
    //     println!("Path score: {}", path.score());
    //     for y in 0..self.size.y {
    //         for x in 0..self.size.x {
    //             let p = Pos { x, y };
    //             if path.have_visited(&p) {
    //                 print!("o");
    //             } else if self.map.contains_key(&p) {
    //                 print!(" ");
    //             } else {
    //                 print!("#");
    //             }
    //         }
    //         println!();
    //     }
    //     println!()
    // }
}

pub fn part_one(input: &str) -> Option<u32> {
    let st = State::try_from(input).ok()?;
    // for p in st.get_paths() {
    //     st.print_path(&p);
    // }
    st.get_paths().iter().map(|p| p.score()).min()
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
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
