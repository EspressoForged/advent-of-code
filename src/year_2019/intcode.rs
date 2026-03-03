use anyhow::{anyhow, Result};

pub struct Intcode {
    pub memory: Vec<i64>,
    pub pc: usize,
}

impl Intcode {
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            memory: program,
            pc: 0,
        }
    }

    /// Run the program to completion.
    pub fn run(&mut self) -> Result<()> {
        loop {
            let opcode = self.memory[self.pc] % 100;
            match opcode {
                1 => {
                    // Add
                    let p1 = self.memory[self.pc + 1] as usize;
                    let p2 = self.memory[self.pc + 2] as usize;
                    let p3 = self.memory[self.pc + 3] as usize;
                    self.memory[p3] = self.memory[p1] + self.memory[p2];
                    self.pc += 4;
                }
                2 => {
                    // Multiply
                    let p1 = self.memory[self.pc + 1] as usize;
                    let p2 = self.memory[self.pc + 2] as usize;
                    let p3 = self.memory[self.pc + 3] as usize;
                    self.memory[p3] = self.memory[p1] * self.memory[p2];
                    self.pc += 4;
                }
                99 => break,
                _ => return Err(anyhow!("Unknown opcode {} at position {}", opcode, self.pc)),
            }
        }
        Ok(())
    }
}
