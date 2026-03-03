use crate::utils::{read_input, Year, Day};
use crate::year_2019::intcode::Intcode;
use anyhow::{Context, Result};

/// Solves Year 2019, Day 5: Sunny with a Chance of Asteroids.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(5))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    // Part 1: Input is 1
    let mut vm1 = Intcode::new(program.clone());
    vm1.add_input(1);
    let outputs1 = vm1.run_to_end()?;
    let part1 = *outputs1
        .last()
        .context("No output produced for Part 1")?;

    // Part 2: Input is 5
    let mut vm2 = Intcode::new(program);
    vm2.add_input(5);
    let outputs2 = vm2.run_to_end()?;
    let part2 = *outputs2
        .last()
        .context("No output produced for Part 2")?;

    Ok((part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_comparisons() {
        // Equal to 8? (Position mode)
        let prog = vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut vm = Intcode::new(prog.clone());
        vm.add_input(8);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 1);

        let mut vm = Intcode::new(prog);
        vm.add_input(7);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 0);

        // Less than 8? (Position mode)
        let prog = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        let mut vm = Intcode::new(prog.clone());
        vm.add_input(7);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 1);

        let mut vm = Intcode::new(prog);
        vm.add_input(8);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 0);

        // Equal to 8? (Immediate mode)
        let prog = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        let mut vm = Intcode::new(prog.clone());
        vm.add_input(8);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 1);

        let mut vm = Intcode::new(prog);
        vm.add_input(7);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 0);
    }

    #[test]
    fn test_day_05_jumps() {
        // Jump tests (Zero? output 0 else 1)
        let prog = vec![3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9];
        let mut vm = Intcode::new(prog.clone());
        vm.add_input(0);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 0);

        let mut vm = Intcode::new(prog);
        vm.add_input(10);
        assert_eq!(*vm.run_to_end().unwrap().last().unwrap(), 1);
    }
}

