use anyhow::{anyhow, Context, Result};
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

    fn get_param(&self, index: usize, mode: i64) -> Result<i64> {
        let val = *self
            .memory
            .get(self.pc + index)
            .with_context(|| format!("Missing parameter at index {} for PC {}", index, self.pc))?;
        match mode {
            0 => {
                let pos = val as usize;
                self.memory.get(pos).copied().with_context(|| {
                    format!(
                        "Invalid memory access at position {} (from param {})",
                        pos, val
                    )
                })
            }
            1 => Ok(val), // Immediate mode
            _ => Err(anyhow!("Unknown parameter mode {}", mode)),
        }
    }

    fn set_memory(&mut self, index: usize, val: i64) -> Result<()> {
        let pos_val = *self.memory.get(self.pc + index).with_context(|| {
            format!(
                "Missing memory pointer at index {} for PC {}",
                index, self.pc
            )
        })?;
        let pos = pos_val as usize;
        if pos >= self.memory.len() {
            return Err(anyhow!(
                "Memory access out of bounds at position {} (from param {})",
                pos,
                pos_val
            ));
        }
        self.memory[pos] = val;
        Ok(())
    }

    /// Run the program until it halts, produces output, or needs input.
    ///
    /// # Errors
    /// Returns an error if an invalid opcode or memory access is encountered.
    pub fn step(&mut self) -> Result<Status> {
        loop {
            let instruction = *self
                .memory
                .get(self.pc)
                .with_context(|| format!("Instruction pointer out of bounds at PC {}", self.pc))?;
            let opcode = instruction % 100;
            let mode1 = (instruction / 100) % 10;
            let mode2 = (instruction / 1000) % 10;

            match opcode {
                1 => {
                    // Add
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    self.set_memory(3, v1 + v2)?;
                    self.pc += 4;
                }
                2 => {
                    // Multiply
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    self.set_memory(3, v1 * v2)?;
                    self.pc += 4;
                }
                3 => {
                    // Input
                    if let Some(val) = self.input.pop_front() {
                        self.set_memory(1, val)?;
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
                    self.set_memory(3, if v1 < v2 { 1 } else { 0 })?;
                    self.pc += 4;
                }
                8 => {
                    // Equals
                    let v1 = self.get_param(1, mode1)?;
                    let v2 = self.get_param(2, mode2)?;
                    self.set_memory(3, if v1 == v2 { 1 } else { 0 })?;
                    self.pc += 4;
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
        loop {
            let last_output = self.output.back().copied();
            let status = self.step()?;
            match status {
                Status::Output(v) if Some(v) == last_output => continue,
                _ => return Ok(status),
            }
        }
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
