advent_of_code::solution!(9);

const MEMORY_SIZE: usize = 100_000;

#[derive(Debug)]
struct MemoryP2 {
}

impl MemoryP2 {
    fn init(ds: Vec<u32>) -> Self {
       todo!() 
    }

    fn fill_empty_blocks(&mut self) {
        todo!()
    }

    fn compute_checksum(&self) -> u64 {
        todo!()
    }
}


#[derive(Debug)]
struct MemoryP1 {
    memory: [Option<u16>; MEMORY_SIZE],
}

impl MemoryP1 {
    fn init(ds: Vec<u32>) -> Self {
        let mut memory: [Option<u16>; 100_000] = [None; 100_000]; //[16; 100_000] = [0; 100_000];
        let mut offset: usize = 0;
        let mut file_nb: u16 = 0;
        let mut is_file = true;
        ds
            .into_iter()
            .for_each(|bs| {
                match is_file {
                    true => {
                        for n in 0..bs {
                            memory[offset + n as usize] = Some(file_nb);
                        }
                        offset += bs as usize;
                        file_nb += 1;
                        is_file = false;
                    },
                    false => {
                        offset += bs as usize;
                        is_file = true;
                    }
                }
            });
        Self {
            memory,
        }
    }

    fn fill_empty_blocks(&mut self) {
        let mut front_cursor = 0;
        let mut back_cursor = MEMORY_SIZE - 1;
        let mut finished = false;

        while !finished {
            while self.memory[back_cursor].is_none() {
                back_cursor -= 1;
            }
            while self.memory[front_cursor].is_some() {
                front_cursor += 1;
            }
            if back_cursor < front_cursor {
                finished = true;
            }
            
            if !finished && self.memory[back_cursor].is_some() && self.memory[front_cursor].is_none() {
                self.memory[front_cursor] = self.memory[back_cursor];
                self.memory[back_cursor] = None;
            }

        }
    }
    fn compute_checksum(&self) -> u64 {
        let mut cursor = 0;
        let mut sum: u64 = 0;
        while let Some(data) = self.memory[cursor] {
            sum += (data as u64) * (cursor as u64);
            cursor += 1;
        }
        sum
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let nbs: Vec<_> = input.chars().flat_map(|c|c.to_digit(10u32)).collect();
    let mut mem = MemoryP1::init(nbs);
    mem.fill_empty_blocks();
    Some(mem.compute_checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nbs: Vec<_> = input.chars().flat_map(|c|c.to_digit(10u32)).collect();
    let mut mem = MemoryP2::init(nbs);
    mem.fill_empty_blocks();
    Some(mem.compute_checksum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
