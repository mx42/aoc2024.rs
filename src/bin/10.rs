advent_of_code::solution!(10);

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Topo {
    map: Vec<Vec<u8>>,
    max: Pos,
}

impl Topo {
    fn init(input: &str) -> Result<Self, ()> {
        let map: Vec<Vec<u8>> = input
            .strip_suffix("\n")
            .unwrap_or(input)
            .lines()
            .map(|l| {
                l.chars()
                    .flat_map(|c| c.to_digit(10u32))
                    .map(|d| d as u8)
                    .collect()
            })
            .collect();
        let max = Pos {
            x: map[0].len(),
            y: map.len(),
        };
        Ok(Self { map, max })
    }

    fn get_trailheads(&self) -> Vec<Pos> {
        self.map
            .clone()
            .into_iter()
            .enumerate()
            .flat_map(|(y, l)| {
                l.into_iter()
                    .enumerate()
                    .map(move |(x, c)| (Pos { x, y }, c))
            })
            .filter_map(|(p, c)| if c == 0 { Some(p) } else { None })
            .collect()
    }

    fn get_neighbors(&self, p: &Pos) -> Vec<(Pos, u8)> {
        let mut res: Vec<(Pos, u8)> = Vec::new();
        if p.x > 0 {
            let x = p.x - 1;
            let y = p.y;
            res.push((Pos { x, y }, self.map[y][x]));
        }
        if p.y > 0 {
            let x = p.x;
            let y = p.y - 1;
            res.push((Pos { x, y }, self.map[y][x]));
        }
        if p.x < self.max.x - 1 {
            let x = p.x + 1;
            let y = p.y;
            res.push((Pos { x, y }, self.map[y][x]));
        }
        if p.y < self.max.y - 1 {
            let x = p.x;
            let y = p.y + 1;
            res.push((Pos { x, y }, self.map[y][x]));
        }
        res
    }

    fn get_successors(&self, p: (Pos, u8)) -> Vec<(Pos, u8)> {
        self.get_neighbors(&p.0)
            .into_iter()
            .filter_map(|(p2, d2)| if d2 == p.1 + 1 { Some((p2, d2)) } else { None })
            .collect()
    }

    fn dfs(&self, p: (Pos, u8), path: &mut Vec<(Pos, u8)>, all_routes: &mut Vec<Vec<(Pos, u8)>>) {
        if p.1 == 9 {
            all_routes.push(path.clone());
            return;
        }

        for n in self.get_successors(p) {
            path.push(n.clone());
            self.dfs(n, path, all_routes);
            path.pop();
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let topo = Topo::init(input).ok()?;
    let ths = topo.get_trailheads();
    let mut score = 0;

    for th in ths {
        let mut all_routes: Vec<Vec<(Pos, u8)>> = Vec::new();
        let mut path: Vec<(Pos, u8)> = vec![(th.clone(), 0)];
        topo.dfs((th, 0), &mut path, &mut all_routes);
        let mut ends = all_routes
            .into_iter()
            .map(|v| v.last().unwrap().0.clone())
            .collect::<Vec<Pos>>();
        ends.sort();
        ends.dedup();
        score += ends.len();
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<usize> {
    let topo = Topo::init(input).ok()?;
    let ths = topo.get_trailheads();
    let mut all_routes: Vec<Vec<(Pos, u8)>> = Vec::new();

    for th in ths {
        let mut path: Vec<(Pos, u8)> = vec![(th.clone(), 0)];
        topo.dfs((th, 0), &mut path, &mut all_routes);
    }
    Some(all_routes.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
