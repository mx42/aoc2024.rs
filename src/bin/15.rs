advent_of_code::solution!(15);

use std::collections::vec_deque::VecDeque;
use std::collections::HashMap;

use std::iter::successors;

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Pos {
    x: i16,
    y: i16,
}

impl Pos {
    fn init(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    fn mv(&self, dir: Dir) -> Self {
        match dir {
            Dir::Up => Pos {
                x: self.x,
                y: self.y - 1,
            },
            Dir::Down => Pos {
                x: self.x,
                y: self.y + 1,
            },
            Dir::Left => Pos {
                x: self.x - 1,
                y: self.y,
            },
            Dir::Right => Pos {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn to_p2(&self) -> (Pos, Pos) {
        (
            Pos {
                x: self.x * 2,
                y: self.y,
            },
            Pos {
                x: self.x * 2 + 1,
                y: self.y,
            },
        )
    }

    fn to_gps(&self) -> u32 {
        (self.y as u32) * 100 + (self.x as u32)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Dir {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '<' => Ok(Dir::Left),
            '>' => Ok(Dir::Right),
            '^' => Ok(Dir::Up),
            'v' => Ok(Dir::Down),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Crate,
    CrateL,
    CrateR,
    Bot,
}

impl TryFrom<char> for Tile {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            '@' => Ok(Tile::Bot),
            'O' => Ok(Tile::Crate),
            _ => Err(()),
        }
    }
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            Tile::Empty => write!(f, ".")?,
            Tile::Wall => write!(f, "#")?,
            Tile::Bot => write!(f, "@")?,
            Tile::Crate => write!(f, "O")?,
            Tile::CrateL => write!(f, "[")?,
            Tile::CrateR => write!(f, "]")?,
        }
        Ok(())
    }
}

#[derive(Clone)]
struct State {
    map: HashMap<Pos, Tile>,
    bot: Pos,
    moves: VecDeque<Dir>,
    size: Pos,
    p2: bool,
}

impl State {
    fn to_p2(&self) -> Option<Self> {
        let mut bot: Option<Pos> = None;
        let mut new_map: HashMap<Pos, Tile> = HashMap::new();
        for (p, t) in self.map.iter() {
            let (p1, p2) = p.to_p2();
            if *t == Tile::Bot {
                bot = Some(p1.clone());
                new_map.insert(p1, Tile::Bot);
                new_map.insert(p2, Tile::Empty);
            } else if *t == Tile::Crate {
                new_map.insert(p1, Tile::CrateL);
                new_map.insert(p2, Tile::CrateR);
            } else {
                new_map.insert(p1, *t);
                new_map.insert(p2, *t);
            }
        }

        bot.map(|bot| Self {
            map: new_map,
            bot,
            moves: self.moves.clone(),
            size: Pos::init(self.size.x * 2, self.size.y),
            p2: true,
        })
    }

    fn score(&self) -> u32 {
        self.map
            .iter()
            .filter_map(|(k, v)| match v {
                Tile::Crate => Some(k.to_gps()),
                Tile::CrateL => Some(k.to_gps()),
                _ => None,
            })
            .sum::<u32>()
    }

    // TODO Probably it could be homogenized...
    fn mv(&mut self, pos: Pos, dir: Dir) -> bool {
        let pos_tile = self.map.get(&pos);
        let dest = pos.mv(dir);
        let dest_tile = self.map.get(&dest);
        let doable = match (pos_tile, dest_tile) {
            (None, _) => false,
            (_, None) => true, // ?
            (Some(Tile::Wall), _) => false,
            (Some(Tile::Empty), _) => false,
            (_, Some(Tile::Empty)) => true,
            (_, Some(Tile::Crate)) => self.mv(dest.clone(), dir),
            (_, Some(Tile::CrateL)) => self.mv(dest.clone(), dir),
            (_, Some(Tile::CrateR)) => self.mv(dest.clone(), dir),
            _ => false,
        };
        if doable {
            let pos_tile = self.map.get(&pos).unwrap();
            self.map.insert(dest, *pos_tile);
            self.map.insert(pos, Tile::Empty);
        }
        doable
    }

    fn move_possible(&self, pos: &Pos, dir: Dir) -> bool {
        let pos_tile = self.map.get(pos);
        let dest = pos.mv(dir);
        let dest_tile = self.map.get(&dest);
        match (pos_tile, dest_tile) {
            (None, _) => false,
            (_, None) => false,
            (Some(Tile::Wall), _) => false,
            (Some(Tile::Empty), _) => false,
            (_, Some(Tile::Empty)) => true,
            (_, Some(Tile::Crate)) => self.move_possible(&dest, dir),
            (_, Some(Tile::CrateL)) if dir == Dir::Left || dir == Dir::Right => {
                self.move_possible(&dest, dir)
            }
            (_, Some(Tile::CrateL)) if dir == Dir::Up || dir == Dir::Down => {
                self.move_possible(&dest, dir)
                    && self.move_possible(&dest.clone().mv(Dir::Right), dir)
            }
            (_, Some(Tile::CrateR)) if dir == Dir::Left || dir == Dir::Right => {
                self.move_possible(&dest, dir)
            }
            (_, Some(Tile::CrateR)) if dir == Dir::Up || dir == Dir::Down => {
                self.move_possible(&dest, dir)
                    && self.move_possible(&dest.clone().mv(Dir::Left), dir)
            }
            _ => false,
        }
    }

    fn do_recursive_move(&mut self, pos: Pos, dir: Dir) {
        let dest = pos.mv(dir);
        let dest_tile = self.map.get(&dest).unwrap();
        if dest_tile == &Tile::Crate {
            self.do_recursive_move(dest.clone(), dir);
        } else if dest_tile == &Tile::CrateL {
            self.do_recursive_move(dest.clone(), dir);
            self.do_recursive_move(dest.clone().mv(Dir::Right), dir);
        } else if dest_tile == &Tile::CrateR {
            self.do_recursive_move(dest.clone(), dir);
            self.do_recursive_move(dest.clone().mv(Dir::Left), dir);
        }
        let pos_tile = self.map.get(&pos).unwrap();
        self.map.insert(dest, *pos_tile);
        self.map.insert(pos, Tile::Empty);
    }

    fn mv_p2(&mut self, pos: Pos, dir: Dir) -> bool {
        if !self.move_possible(&pos, dir) {
            return false;
        }
        self.do_recursive_move(pos, dir);
        true
    }

    fn next(&self) -> Option<Self> {
        let mut new_state = self.clone();
        let move_dir = new_state.moves.pop_front()?;
        let dest = new_state.bot.mv(move_dir);
        if (self.p2
            && (move_dir == Dir::Up || move_dir == Dir::Down)
            && new_state.mv_p2(new_state.bot.clone(), move_dir))
            || (new_state.mv(new_state.bot.clone(), move_dir))
        {
            new_state.bot = dest;
        }
        Some(new_state)
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "State:\nSize: {:?} - Bot: {:?}", self.size, self.bot)?;
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                if let Some(t) = self.map.get(&Pos::init(x, y)) {
                    write!(f, "{:?}", t)?;
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "Moves Left: {:?}", self.moves.len())?;
        Ok(())
    }
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let (_, map, moves, size): (bool, HashMap<Pos, Tile>, Vec<Dir>, Pos) =
            input.lines().enumerate().try_fold(
                (true, HashMap::new(), Vec::new(), Pos::init(0, 0)),
                |acc, (y, l)| {
                    let (parse_map, mut map, mut moves, size) = acc;
                    if l.is_empty() {
                        Ok((false, map, moves, size))
                    } else if parse_map {
                        let new_map: Result<Vec<(Pos, Tile)>, ()> = l
                            .chars()
                            .enumerate()
                            .map(|(x, c)| {
                                let p = Pos::init(x as i16, y as i16);
                                Tile::try_from(c).map(|t| (p, t))
                            })
                            .collect();
                        if let Ok(new_map) = new_map {
                            for (p, t) in new_map {
                                map.insert(p, t);
                            }
                            Ok((true, map, moves, Pos::init(l.len() as i16, y as i16 + 1)))
                        } else {
                            Err(())
                        }
                    } else {
                        moves.extend(l.chars().flat_map(Dir::try_from));
                        Ok((false, map, moves, size))
                    }
                },
            )?;
        if let Some((bot, _)) = map.iter().find(|(_, t)| **t == Tile::Bot) {
            Ok(Self {
                bot: bot.clone(),
                map,
                moves: moves.into(),
                size,
                p2: false,
            })
        } else {
            Err(())
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    successors(State::try_from(input).ok(), |st| st.next())
        .last()
        .unwrap()
        .score()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    successors(State::try_from(input).ok()?.to_p2(), |st| st.next())
        .last()
        .unwrap()
        .score()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
