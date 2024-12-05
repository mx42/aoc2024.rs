advent_of_code::solution!(5);

#[derive(Debug)]
struct Rules {
    rules: Vec<(u32, u32)>
}

impl From<&[&str]> for Rules {
    fn from(lines: &[&str]) -> Self {
        let mut rules: Vec<(u32, u32)> = lines
            .into_iter()
            .map(
                |line| {
                    let pages = line
                        .split("|")
                        .map(|page| page.parse::<u32>().expect("invalid numeric in rules"))
                        .collect::<Vec<u32>>();
                    match pages.as_slice() {
                        [a, b] => (*a, *b),
                        _ => panic!("invalid input: invalid rule"),
                    }
                }
            )
            .collect();
        rules.sort();
        Self { rules }
    }
}

impl Rules {
    fn pages_after(&self, page: u32) -> Vec::<u32> {
        self
            .rules
            .clone()
            .into_iter()
            .filter_map(|r| match r {
                (p1, p2) if p1 == page => Some(p2),
                _ => None
            })
            .collect()
    }

    fn pages_before(&self, page: u32) -> Vec::<u32> {
        self
            .rules
            .clone()
            .into_iter()
            .filter_map(|r| match r {
                (p1, p2) if p2 == page => Some(p1),
                _ => None
            })
            .collect()
    }

    fn get_correct_pages_order(&self, pages: Vec<u32>) -> Vec<u32> {
        let mut pages_with_order: Vec<(u32, Vec<u32>, Vec<u32>)> = pages
            .clone()
            .into_iter()
            .map(|p|
                (
                    p,
                    self.pages_before(p).into_iter().filter(|pp| pages.contains(pp)).collect(),
                    self.pages_after(p).into_iter().filter(|pp| pages.contains(pp)).collect(),
                )
            )
            .collect();
        pages_with_order.sort_by_key(
            |(p, before, after)| (before.len(), after.len())
        );
        pages_with_order.into_iter().map(|(p, _, _)| p).collect()
    }
}

#[derive(Debug)]
struct ImpressionOrder {
    pages: Vec<u32>
}

impl ImpressionOrder {
    fn middle_page(&self) -> Option<u32> {
        let length = self.pages.len();
        if length % 2 == 0 {
            None
        } else {
            Some(self.pages[ (length / 2) as usize ])
        }
    }

    fn fix_order(&self, rules: &Rules) -> Self {
        Self {
            pages: rules.get_correct_pages_order(self.pages.clone())
        }
    }
    
    fn is_valid(&self, rules: &Rules) -> bool {
        let mut invalid_pages: Vec<u32> = Vec::new();
        for p in self.pages.clone() {
            if invalid_pages.contains(&p) {
                return false;
            }
            invalid_pages.extend(rules.pages_before(p));
        }
        true
    }
}

impl TryFrom<&&str> for ImpressionOrder {
    type Error = ();
    fn try_from(line: &&str) -> Result<Self, Self::Error> {
        let pages = line.split(",").flat_map(|page| page.parse::<u32>()).collect::<Vec<u32>>();
        Ok(Self { pages })
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .collect::<Vec<&str>>();
    let input = input
        .split(|s|s.is_empty())
        .collect::<Vec<&[&str]>>();

    let rules: Rules = input[0].into();
    input[1]
        .into_iter()
        .flat_map(ImpressionOrder::try_from)
        .filter_map(|o| if o.is_valid(&rules) { o.middle_page() } else { None })
        .sum::<u32>()
        .into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = input
        .lines()
        .collect::<Vec<&str>>();
    let input = input
        .split(|s|s.is_empty())
        .collect::<Vec<&[&str]>>();

    let rules: Rules = input[0].into();
    input[1]
        .into_iter()
        .flat_map(ImpressionOrder::try_from)
        .filter_map(|o| if !o.is_valid(&rules) { o.fix_order(&rules).middle_page() } else { None })
        .sum::<u32>()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
