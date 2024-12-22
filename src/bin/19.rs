advent_of_code::solution!(19);

use std::collections::HashMap;

fn is_possible(pat: String, towels: &Vec<String>) -> bool {
    if towels.contains(&pat) {
        return true;
    }
    for towel in towels {
        if pat.starts_with(towel) {
            let sub_possible = is_possible((pat[towel.len()..]).to_owned(), towels);
            if sub_possible {
                return true;
            }
        }
    }

    false
}

#[derive(Debug)]
struct State {
    towels: Vec<String>,
    simplified: Vec<String>,
    patterns: Vec<String>,
    possible_cache: HashMap<String, usize>,
}

fn simplified_towels(towels: Vec<String>) -> Vec<String> {
    let mut res = Vec::new();
    for t in &towels {
        let remaining: Vec<String> = towels.clone().into_iter().filter(|t2| t2 != t).collect();
        if t.len() == 1 || !is_possible(t.clone(), &remaining) {
            res.push(t.clone());
        }
    }
    res
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(input: &str) -> Result<State, Self::Error> {
        let mut input: Vec<&str> = input.lines().collect();
        if input.len() < 3 {
            return Err(());
        }
        let mut towels: Vec<String> = input.remove(0).split(", ").map(String::from).collect();
        towels.sort_by_key(|t| (t.len(), t.clone()));
        input.remove(0);
        Ok(Self {
            patterns: input.into_iter().map(|s| s.to_owned()).collect(),
            simplified: simplified_towels(towels.clone()),
            towels,
            possible_cache: HashMap::new(),
        })
    }
}

impl State {
    fn is_possible(&self, pat: String) -> bool {
        is_possible(pat, &self.simplified)
    }

    fn nb_possible(&mut self, pat: String) -> usize {
        if let Some(cached) = self.possible_cache.get(&pat) {
            return *cached;
        }
        if pat.is_empty() {
            self.possible_cache.insert(pat, 1);
            return 1;
        }
        let mut res = 0;
        for towel in &self.towels.clone() {
            if pat.starts_with(towel) {
                res += self.nb_possible((pat[towel.len()..]).to_owned());
            }
        }
        self.possible_cache.insert(pat, res);
        res
    }

    fn get_possible_patterns_count(&self) -> usize {
        self.patterns
            .iter()
            .filter(|p| self.is_possible(p.to_string()))
            .count()
    }

    fn get_nb_possible_configurations(&mut self) -> usize {
        self.patterns
            .clone()
            .into_iter()
            .map(|p| self.nb_possible(p.to_string()))
            .sum::<usize>()
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    State::try_from(input)
        .ok()?
        .get_possible_patterns_count()
        .into()
}

pub fn part_two(input: &str) -> Option<usize> {
    State::try_from(input)
        .ok()?
        .get_nb_possible_configurations()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
