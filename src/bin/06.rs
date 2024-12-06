advent_of_code::solution!(6);

use std::iter::successors;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn to_vec(self) -> (i8, i8) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }

    fn rotate90(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn init(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    fn move_towards(&self, v: (i8, i8), max: &Self) -> Option<Self> {
        let new_x: isize = self.x as isize + v.0 as isize;
        let new_y: isize = self.y as isize + v.1 as isize;
        if new_x >= 0 && (new_x as usize) < max.x && new_y >= 0 && (new_y as usize) < max.y {
            Some(Pos {
                x: new_x as usize,
                y: new_y as usize,
            })
        } else {
            None
        }
    }

    fn is_at_limit(&self, max: &Self) -> bool {
        self.x == max.x - 1 || self.y == max.y - 1
    }
}

#[derive(Clone, PartialEq)]
struct Guard {
    pos: Pos,
    facing: Dir,
}

impl std::fmt::Debug for Guard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self.facing {
            Dir::Up => write!(f, "^"),
            Dir::Left => write!(f, "<"),
            Dir::Right => write!(f, ">"),
            Dir::Down => write!(f, "V"),
        }
    }
}

#[derive(Clone)]
struct State {
    walls: Vec<Pos>,
    walked: Vec<Guard>,
    guard: Guard,
    map_size: Pos,
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Size: {:?}", self.map_size)?;
        for y in 0..self.map_size.y {
            for x in 0..self.map_size.x {
                let pos = Pos { x, y };
                if self.walls.contains(&pos) {
                    write!(f, "#")?;
                    continue;
                }
                // screw the cpu
                let match_walk = self
                    .walked
                    .clone()
                    .into_iter()
                    .filter(|g| g.pos == pos)
                    .collect::<Vec<_>>();
                match match_walk.len() {
                    0 => write!(f, ".")?,
                    1 => write!(f, "{:?}", match_walk[0])?,
                    _ => write!(f, "*")?,
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl State {
    fn step(self) -> Option<Self> {
        if self.is_last_step() {
            return None;
        }
        let guard_walk_direction = self.guard.facing.to_vec();
        let guard_walk = successors(Some(self.guard.pos), |pos| {
            if let Some(new_pos) = pos.move_towards(guard_walk_direction, &self.map_size) {
                if !self.walls.contains(&new_pos) {
                    return Some(new_pos);
                }
            }
            None
        })
        .collect::<Vec<Pos>>();
        let guard_new_pos = guard_walk.last().unwrap().clone();
        let mut walked: Vec<Guard> = self.walked.clone();
        walked.extend(guard_walk.iter().map(|p| Guard {
            pos: p.clone(),
            facing: self.guard.facing,
        }));

        Some(Self {
            guard: Guard {
                pos: guard_new_pos,
                facing: self.guard.facing.rotate90(),
            },
            walked,
            ..self
        })
    }

    fn is_last_step(&self) -> bool {
        self.guard.pos.is_at_limit(&self.map_size)
    }
}

fn parse_line(line: &str, line_nb: usize) -> (Vec<Pos>, Option<Pos>) {
    line.chars()
        .enumerate()
        .fold(
            (Vec::<Pos>::new(), None),
            |(mut wpos, gpos), (pos, chr)| match chr {
                '#' => {
                    wpos.push(Pos::init(pos, line_nb));
                    (wpos, gpos)
                }
                '^' => (wpos, Some(Pos::init(pos, line_nb))),
                _ => (wpos, gpos),
            },
        )
}

fn parse_input(input: &str) -> State {
    let input = input.lines().collect::<Vec<_>>();
    let size = Pos::init(input[0].len(), input.len());
    let data = input
        .into_iter()
        .enumerate()
        .map(|(nb, line)| parse_line(line, nb))
        .fold(
            (Vec::<Pos>::new(), None),
            |(mut gl_wpos, gl_gpos), (wpos, gpos)| {
                gl_wpos.extend(wpos);
                match gpos {
                    Some(_) => (gl_wpos, gpos),
                    _ => (gl_wpos, gl_gpos),
                }
            },
        );
    if data.1.is_none() {
        panic!("Guard position not found?!");
    }
    let guard: Guard = Guard {
        pos: data.1.unwrap(),
        facing: Dir::Up,
    };
    State {
        walls: data.0,
        guard: guard.clone(),
        walked: vec![guard],
        map_size: size,
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let state = parse_input(input);
    let last_state = successors(Some(state), |s| s.clone().step())
        .last()
        .unwrap();
    // println!("{:?}", last_state);
    let mut visited = last_state
        .walked
        .iter()
        .map(|g| &g.pos)
        .collect::<Vec<&Pos>>();
    visited.sort();
    visited.dedup();

    Some(visited.len())
}

pub fn part_two(_input: &str) -> Option<usize> {
    // let state = parse_input(input);
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
