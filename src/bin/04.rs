advent_of_code::solution!(4);

#[derive(Debug, Clone)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn init(x: usize, y: usize) -> Self {
        Self { x, y }
    }
    fn get_pos_from_origin(&self) -> Vec<Pos> {
        (0..self.y)
            .flat_map(|y| (0..self.x).map(|x| Pos::init(x, y)).collect::<Vec<Pos>>())
            .collect::<Vec<Pos>>()
    }
    fn diag_pos(&self, max: &Pos) -> Option<Vec<Pos>> {
        if self.y == 0 || self.y >= max.y - 1 || self.x == 0 || self.x >= max.x - 1 {
            None
        } else {
            Some(vec![
                // Northwest
                Pos::init(self.x - 1, self.y - 1),
                // Northeast
                Pos::init(self.x + 1, self.y - 1),
                // Southwest
                Pos::init(self.x - 1, self.y + 1),
                // Southeast
                Pos::init(self.x + 1, self.y + 1),
            ])
        }
    }

    fn xmas_pos(&self, max: &Pos) -> Vec<Vec<Pos>> {
        let mut res: Vec<Vec<Pos>> = Vec::new();
        if self.y >= 3 {
            if self.x >= 3 {
                // Northwest
                res.push(
                    (1..4)
                        .map(|z| Pos::init(self.x - z as usize, self.y - z as usize))
                        .collect(),
                );
            }
            // North
            res.push(
                (1..4)
                    .map(|y| Pos::init(self.x, self.y - y as usize))
                    .collect(),
            );
            if self.x < max.x - 3 {
                // Northeast
                res.push(
                    (1..4)
                        .map(|z| Pos::init(self.x + z as usize, self.y - z as usize))
                        .collect(),
                );
            }
        }
        if self.x >= 3 {
            // West
            res.push(
                (1..4)
                    .map(|x| Pos::init(self.x - x as usize, self.y))
                    .collect(),
            );
        }
        if self.x < max.x - 3 {
            // East
            res.push(
                (1..4)
                    .map(|x| Pos::init(self.x + x as usize, self.y))
                    .collect(),
            );
        }
        if self.y < max.y - 3 {
            if self.x >= 3 {
                // Southwest
                res.push(
                    (1..4)
                        .map(|z| Pos::init(self.x - z as usize, self.y + z as usize))
                        .collect(),
                );
            }
            // South
            res.push(
                (1..4)
                    .map(|y| Pos::init(self.x, self.y + y as usize))
                    .collect(),
            );
            if self.x < max.x - 3 {
                // Southeast
                res.push(
                    (1..4)
                        .map(|z| Pos::init(self.x + z as usize, self.y + z as usize))
                        .collect(),
                );
            }
        }
        // println!("Surrounding pos for {:?}", self);
        // println!("{:#?}", res);
        res
    }
    fn get_char_in_input(&self, input: &[&str]) -> char {
        input[self.y].chars().nth(self.x).unwrap_or('0')
    }
}

fn mas_at_pos(input: &[&str], ps: &[Pos]) -> bool {
    for (p, l) in ps.iter().zip("MAS".chars()) {
        let c = p.get_char_in_input(input);
        if c != l {
            return false;
        }
    }
    true
}

fn cross_mas_at_pos(input: &[&str], ps: &[Pos]) -> bool {
    let letters = ps
        .iter()
        .map(|p| p.get_char_in_input(input))
        .collect::<Vec<char>>();
    letters.iter().all(|&c| c == 'M' || c == 'S')
        && letters[0] != letters[3]
        && letters[1] != letters[2]
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .collect::<Vec<_>>();
    let input = input.as_slice();
    let max = Pos::init(input[0].len(), input.len());
    max.get_pos_from_origin()
        .iter()
        .filter(|p| p.get_char_in_input(input) == 'X')
        .flat_map(|p| p.xmas_pos(&max))
        .filter(|ps| mas_at_pos(input, ps))
        .collect::<Vec<_>>()
        .len()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = input
        .strip_suffix("\n")
        .unwrap_or(input)
        .lines()
        .collect::<Vec<_>>();
    let input = input.as_slice();
    let max = Pos::init(input[0].len(), input.len());
    max.get_pos_from_origin()
        .iter()
        .filter(|p| p.get_char_in_input(input) == 'A')
        .flat_map(|p| p.diag_pos(&max))
        .filter(|ps| cross_mas_at_pos(input, ps))
        .collect::<Vec<_>>()
        .len()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
