advent_of_code::solution!(14);

use std::collections::HashMap;
use std::iter::successors;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    fn add_with_wrap(&self, v: &Velocity, wrap: &Pos) -> Self {
        Self {
            x: (((wrap.x as i16) + (self.x as i16) + (v.x as i16)) as u16) % wrap.x,
            y: (((wrap.y as i16) + (self.y as i16) + (v.y as i16)) as u16) % wrap.y,
        }
    }
}

impl TryFrom<&&str> for Pos {
    type Error = ();

    fn try_from(input: &&str) -> Result<Pos, Self::Error> {
        let coords: Vec<u16> = input[2..]
            .split(",")
            .map(|s| s.parse::<u16>().map_err(|_| ()))
            .collect::<Result<_, ()>>()?;
        let coords = coords.as_slice();
        match coords {
            [x, y] => Ok(Self { x: *x, y: *y }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Velocity {
    x: i8,
    y: i8,
}

impl TryFrom<&&str> for Velocity {
    type Error = ();

    fn try_from(input: &&str) -> Result<Self, Self::Error> {
        let coords: Vec<i8> = input[2..]
            .split(",")
            .map(|s| s.parse::<i8>().map_err(|_| ()))
            .collect::<Result<_, ()>>()?;
        let coords = coords.as_slice();
        match coords {
            [x, y] => Ok(Self { x: *x, y: *y }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Bot {
    p: Pos,
    v: Velocity,
}

impl Bot {
    fn next(&self, wrap: &Pos) -> Self {
        Self {
            v: self.v.clone(),
            p: self.p.add_with_wrap(&self.v, wrap),
        }
    }
}

impl TryFrom<&str> for Bot {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let parts = input.split_whitespace().collect::<Vec<_>>();
        let parts = parts.as_slice();
        match parts {
            [p, v] => Ok(Self {
                p: Pos::try_from(p).map_err(|_| ())?,
                v: Velocity::try_from(v).map_err(|_| ())?,
            }),
            _ => Err(()),
        }
    }
}

struct State {
    bots: Vec<Bot>,
    size: Pos,
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            bots: input
                .lines()
                .map(Bot::try_from)
                .collect::<Result<Vec<Bot>, ()>>()?,
            size: Pos { x: 101, y: 103 },
        })
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let p = Pos { x, y };
                if self.bots.iter().any(|b| b.p == p) {
                    write!(f, "*")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl State {
    fn anomaly(&self) -> bool {
        let mut ys: HashMap<u16, usize> = HashMap::new();
        let mut xs: HashMap<u16, usize> = HashMap::new();

        self.bots.iter().for_each(|b| {
            *ys.entry(b.p.y).or_default() += 1;
            *xs.entry(b.p.x).or_default() += 1;
        });
        let mut xs = xs.into_values().collect::<Vec<usize>>();
        let mut ys = ys.into_values().collect::<Vec<usize>>();
        ys.sort();
        xs.sort();
        ys[ys.len() - 1] > 30 && xs[xs.len() - 1] > 30
    }

    fn quadrants(&self) -> [u32; 4] {
        self.bots
            .iter()
            .map(|b| b.p.clone())
            .fold([0, 0, 0, 0], |[ul, ur, ll, lr], p| match p {
                Pos { x: px, y: py } if px < self.size.x / 2 && py < self.size.y / 2 => {
                    [ul + 1, ur, ll, lr]
                }
                Pos { x: px, y: py } if px < self.size.x / 2 && py > self.size.y / 2 => {
                    [ul, ur, ll + 1, lr]
                }
                Pos { x: px, y: py } if px > self.size.x / 2 && py < self.size.y / 2 => {
                    [ul, ur + 1, ll, lr]
                }
                Pos { x: px, y: py } if px > self.size.x / 2 && py > self.size.y / 2 => {
                    [ul, ur, ll, lr + 1]
                }
                _ => [ul, ur, ll, lr],
            })
    }

    fn next(&self) -> Option<State> {
        Some(Self {
            bots: self
                .bots
                .clone()
                .into_iter()
                .map(|b| b.next(&self.size))
                .collect(),
            size: self.size.clone(),
        })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    successors(State::try_from(input).ok(), |st| st.next())
        .nth(100)
        .unwrap()
        .quadrants()
        .iter()
        .product::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    successors(State::try_from(input).ok(), |s| s.next())
        .enumerate()
        .find(|(i, st)| st.anomaly() || *i > 10000)
        .map(|(i, st)| {
            if st.anomaly() {
                println!("{:?}", st);
                Some(i)
            } else {
                None
            }
        })?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
