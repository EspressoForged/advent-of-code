use anyhow::{anyhow, Result};
use std::collections::VecDeque;

/// Represents the execution status of an Intcode virtual machine.
#[derive(Debug, PartialEq, Eq)]
pub enum Status {
    /// The program has reached a halt instruction (99).
    Halted,
    /// The program is waiting for input (opcode 3).
    NeedsInput,
    /// The program has produced an output (opcode 4).
    Output(i64),
}

/// An Intcode virtual machine.
pub struct Intcode {
    /// The program memory.
    pub memory: Vec<i64>,
    /// The current instruction pointer.
    pub pc: usize,
    /// The relative base for relative mode (mode 2).
    pub relative_base: i64,
    /// The input queue.
    pub input: VecDeque<i64>,
    /// The output queue.
    pub output: VecDeque<i64>,
}

impl Intcode {
    /// Creates a new Intcode VM with the given program.
    #[must_use]
    pub fn new(program: Vec<i64>) -> Self {
        Self {
            memory: program,
            pc: 0,
            relative_base: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
        }
    }

    /// Adds a value to the input queue.
    pub fn add_input(&mut self, val: i64) {
        self.input.push_back(val);
    }

    /// Retrieves and removes the oldest value from the output queue.
    #[allow(dead_code)]
    pub fn get_output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    fn ensure_memory(&mut self, address: usize) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
    }

    fn get_mem(&mut self, address: usize) -> i64 {
        self.ensure_memory(address);
        self.memory[address]
    }

    fn set_mem(&mut self, address: usize, val: i64) -> Result<()> {
        self.ensure_memory(address);
        self.memory[address] = val;
        Ok(())
    }

    fn get_param(&mut self, index: usize, mode: i64) -> Result<i64> {
        let val = self.get_mem(self.pc + index);
        match mode {
            0 => Ok(self.get_mem(val as usize)), // Position mode
            1 => Ok(val),                        // Immediate mode
            2 => Ok(self.get_mem((self.relative_base + val) as usize)), // Relative mode
            _ => Err(anyhow!("Unknown parameter mode {}", mode)),
        }
    }

    fn get_target_address(&mut self, index: usize, mode: i64) -> Result<usize> {
        let val = self.get_mem(self.pc + index);
        match mode {
            0 => Ok(val as usize),                        // Position mode
            2 => Ok((self.relative_base + val) as usize), // Relative mode
            _ => Err(anyhow!("Invalid mode {} for target address", mode)),
        }
    }

    /// Run the program until it halts, produces output, or needs input.
    ///
    /// # Errors
    /// Returns an error if an invalid opcode or memory access is encountered.
    pub fn step(&mut self) -> Result<Status> {
        loop {
            let instruction = self.get_mem(self.pc);
            let opcode = instruction % 100;
            let mode1 = (instruction / 100) % 10;
            let mode2 = (instruction / 1000) % 10;
            let mode3 = (instruction / 10000) % 10;

            match opcode {
                1 => {
                    // Add
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    let target = self.get_target_address(3, mode3)?;
                    self.set_mem(target, v1 + v2)?;
                    self.pc += 4;
                }
                2 => {
                    // Multiply
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    let target = self.get_target_address(3, mode3)?;
                    self.set_mem(target, v1 * v2)?;
                    self.pc += 4;
                }
                3 => {
                    // Input
                    if let Some(val) = self.input.pop_front() {
                        let target = self.get_target_address(1, mode1)?;
                        self.set_mem(target, val)?;
                        self.pc += 2;
                    } else {
                        return Ok(Status::NeedsInput);
                    }
                }
                4 => {
                    // Output
                    let v1 = self.get_param(1, mode1)?;
                    self.output.push_back(v1);
                    self.pc += 2;
                    return Ok(Status::Output(v1));
                }
                5 => {
                    // Jump-if-true
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    if v1 != 0 {
                        self.pc = v2 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    // Jump-if-false
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    if v1 == 0 {
                        self.pc = v2 as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    // Less than
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    let target = self.get_target_address(3, mode3)?;
                    self.set_mem(target, if v1 < v2 { 1 } else { 0 })?;
                    self.pc += 4;
                }
                8 => {
                    // Equals
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    let target = self.get_target_address(3, mode3)?;
                    self.set_mem(target, if v1 == v2 { 1 } else { 0 })?;
                    self.pc += 4;
                }
                9 => {
                    // Adjust relative base
                    let v1 = self.get_param(1, mode1)?;
                    self.relative_base += v1;
                    self.pc += 2;
                }
                99 => return Ok(Status::Halted),
                _ => return Err(anyhow!("Unknown opcode {} at position {}", opcode, self.pc)),
            }
        }
    }

    /// Run the program to completion or until it needs input.
    ///
    /// # Errors
    /// Returns an error if execution fails.
    pub fn run(&mut self) -> Result<Status> {
        self.step()
    }

    /// Helper to run until halt and collect all outputs.
    ///
    /// # Errors
    /// Returns an error if the program requests input or execution fails.
    pub fn run_to_end(&mut self) -> Result<Vec<i64>> {
        let mut outputs = Vec::new();
        loop {
            match self.step()? {
                Status::Halted => break,
                Status::Output(v) => outputs.push(v),
                Status::NeedsInput => {
                    return Err(anyhow!("Program requested input but none available"))
                }
            }
        }
        Ok(outputs)
    }
}
