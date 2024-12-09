advent_of_code::solution!(9);

use std::collections::VecDeque;

const MEMORY_SIZE: usize = 100_000;

#[derive(Debug, Clone)]
struct Blocks {
    file_id: Option<u16>,
    size: u8,
}

impl Blocks {
    fn get_checksum(&self, offset: u64) -> u64 {
        if let Some(id) = self.file_id {
            let mut chk: u64 = 0;
            for x in 0..self.size {
                chk += (offset + (x as u64)) * (id as u64);
            }
            chk
        } else {
            0
        }
    }
}

#[derive(Debug)]
struct MemoryP2 {
    memory: VecDeque<Blocks>,
}

impl MemoryP2 {
    fn init(ds: Vec<u32>) -> Self {
        let mut file_nb: u16 = 0;
        let mut is_file = true;
        let mut memory: VecDeque<Blocks> = VecDeque::new();
        ds.into_iter().for_each(|bs| {
            let file_id = if is_file {
                let id = file_nb;
                file_nb += 1;
                is_file = false;
                Some(id)
            } else {
                is_file = true;
                None
            };
            if bs > 0 {
                memory.push_back(Blocks {
                    file_id,
                    size: bs as u8,
                });
            }
        });
        Self { memory }
    }

    fn fill_empty_blocks(&mut self) {
        let mut new_list: VecDeque<Blocks> = VecDeque::new();
        while !self.memory.is_empty() {
            let mut cur = self.memory.pop_front().unwrap();
            if cur.file_id.is_some() {
                new_list.push_back(cur);
                continue;
            }
            // try to find something fitting
            let mut tmp: VecDeque<Blocks> = VecDeque::new();
            let mut inserted = false;
            while let Some(block) = self.memory.pop_back() {
                if block.file_id.is_some() && block.size <= cur.size {
                    cur.size -= block.size;
                    let empty_size = block.size;
                    new_list.push_back(block);
                    inserted = true;

                    self.memory.push_back(Blocks {
                        file_id: None,
                        size: empty_size,
                    });
                    break;
                } else {
                    tmp.push_front(block);
                }
            }
            self.memory.extend(tmp);
            if cur.size > 0 {
                if inserted {
                    self.memory.push_front(cur);
                } else {
                    new_list.push_back(cur);
                }
            }
        }
        self.memory = new_list;
    }

    fn compute_checksum(&self) -> u64 {
        let mut checksum: u64 = 0;
        let mut offset = 0;

        for block in &self.memory {
            checksum += block.get_checksum(offset);
            offset += block.size as u64;
        }
        checksum
    }
}

#[derive(Debug)]
struct MemoryP1 {
    memory: [Option<u16>; MEMORY_SIZE],
}

impl MemoryP1 {
    fn init(ds: Vec<u32>) -> Self {
        let mut memory: [Option<u16>; 100_000] = [None; 100_000];
        let mut offset: usize = 0;
        let mut file_nb: u16 = 0;
        let mut is_file = true;
        ds.into_iter().for_each(|bs| match is_file {
            true => {
                for n in 0..bs {
                    memory[offset + n as usize] = Some(file_nb);
                }
                offset += bs as usize;
                file_nb += 1;
                is_file = false;
            }
            false => {
                offset += bs as usize;
                is_file = true;
            }
        });
        Self { memory }
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

            if !finished
                && self.memory[back_cursor].is_some()
                && self.memory[front_cursor].is_none()
            {
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
    let nbs: Vec<_> = input.chars().flat_map(|c| c.to_digit(10u32)).collect();
    let mut mem = MemoryP1::init(nbs);
    mem.fill_empty_blocks();
    Some(mem.compute_checksum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let nbs: Vec<_> = input.chars().flat_map(|c| c.to_digit(10u32)).collect();
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
        assert_eq!(result, Some(2858));
    }
}
