advent_of_code::solution!(17);

use std::iter::successors;

#[derive(Debug, Clone)]
struct State {
    registers: [i32; 3],
    memory: Vec<i32>,
    ptr: usize,
    output: Vec<i32>,
}

impl State {
    fn dv(&self, reg: usize, combo: i32) -> Option<Self> {
        println!(
            "DIV reg 0 {} / 2^{} => reg {}",
            self.registers[0], combo, reg
        );
        let two: i32 = 2;
        let res = self.registers[0].checked_div(two.pow(combo as u32))?;
        let mut new_registers = self.registers.clone();
        new_registers[reg] = res;
        Some(Self {
            registers: new_registers,
            ptr: self.ptr + 2,
            ..self.clone()
        })
    }

    fn bxl(&self, combo: i32) -> Option<Self> {
        println!("BXL reg 1 ^ {} => reg 1", combo);
        let res = self.registers[1] ^ combo;
        let mut new_registers = self.registers.clone();
        new_registers[1] = res;
        Some(Self {
            registers: new_registers,
            ptr: self.ptr + 2,
            ..self.clone()
        })
    }

    fn bst(&self, combo: i32) -> Option<Self> {
        println!("BST {} % 8 => reg 1", combo);
        let mut new_registers = self.registers.clone();
        new_registers[1] = combo % 8;
        Some(Self {
            registers: new_registers,
            ptr: self.ptr + 2,
            ..self.clone()
        })
    }

    fn jnz(&self, combo: i32) -> Option<Self> {
        if self.registers[0] == 0 {
            println!("JNZ reg 0 == 0 => NO-OP");
            Some(Self {
                ptr: self.ptr + 2,
                ..self.clone()
            })
        } else {
            println!("JNZ reg 0 != 0 => JMP {}", combo);
            Some(Self {
                ptr: combo as usize,
                ..self.clone()
            })
        }
    }

    fn bxc(&self) -> Option<Self> {
        println!("BXC reg 1 ^ reg 2 => reg 1");
        let mut new_registers = self.registers.clone();
        new_registers[1] = self.registers[1] ^ self.registers[2];
        Some(Self {
            registers: new_registers,
            ptr: self.ptr + 2,
            ..self.clone()
        })
    }

    fn out(&self, combo: i32) -> Option<Self> {
        println!("OUT {} % 8 => out", combo);
        let mut new_out = self.output.clone();
        new_out.push(combo % 8);
        Some(Self {
            output: new_out,
            ptr: self.ptr + 2,
            ..self.clone()
        })
    }

    fn run(&self) -> Self {
        successors(Some(self.clone()), |st| st.step())
            .last()
            .unwrap()
    }

    fn step(&self) -> Option<Self> {
        println!("Current state\n{:?}", self);
        if self.ptr >= self.memory.len() {
            return None;
        }
        let opcode = self.memory.get(self.ptr).unwrap();
        let combo = *self.memory.get(self.ptr + 1).unwrap();
        if combo < 0 || combo > 6 {
            return None;
        }
        let combo_value = match combo {
            0..=3 => combo,
            4 => self.registers[0],
            5 => self.registers[1],
            6 => self.registers[2],
            _ => panic!(),
        };
        match opcode {
            0 => self.dv(0, combo_value),
            1 => self.bxl(combo_value),
            2 => self.bst(combo_value),
            3 => self.jnz(combo_value),
            4 => self.bxc(),
            5 => self.out(combo_value),
            6 => self.dv(1, combo_value),
            7 => self.dv(2, combo_value),
            _ => None,
        }
    }
    fn get_output(&self) -> String {
        let vec: Vec<String> = self.output.iter().map(|n| n.to_string()).collect();
        vec.join(",")
    }
}

impl TryFrom<&str> for State {
    type Error = ();
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut input = input
            .lines()
            .filter_map(|l| {
                if l.is_empty() {
                    None
                } else {
                    Some(
                        l.split(": ")
                            .skip(1)
                            .flat_map(|s| {
                                s.split(",")
                                    .map(|n| n.parse::<i32>().unwrap())
                                    .collect::<Vec<_>>()
                            })
                            .collect::<Vec<_>>(),
                    )
                }
            })
            .flatten()
            .collect::<Vec<_>>();
        if input.len() < 4 {
            Err(())
        } else {
            let registers: [i32; 3] = [input.remove(0), input.remove(0), input.remove(0)];
            Ok(State {
                registers,
                memory: input,
                ptr: 0,
                output: Vec::new(),
            })
        }
    }
}

pub fn part_one(input: &str) -> Option<String> {
    State::try_from(input).ok().map(|s| s.run().get_output())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".into()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
