use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::Intcode;
use anyhow::{Context, Result};

/// Solves Year 2019, Day 9: Sensor Boost.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(9))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    // Part 1: Run BOOST program in test mode (input 1)
    let mut vm1 = Intcode::new(program.clone());
    vm1.add_input(1);
    let outputs1 = vm1.run_to_end()?;
    let part1 = *outputs1.last().context("No output from BOOST program (Part 1)")?;

    // Part 2: Run BOOST program in sensor boost mode (input 2)
    let mut vm2 = Intcode::new(program);
    vm2.add_input(2);
    let outputs2 = vm2.run_to_end()?;
    let part2 = *outputs2.last().context("No output from BOOST program (Part 2)")?;

    Ok((part1 as u64, part2 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_09_examples() -> Result<()> {
        // Quine program
        let quine = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut vm = Intcode::new(quine.clone());
        let outputs = vm.run_to_end()?;
        assert_eq!(outputs, quine);

        // 16-digit number
        let large_prod = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut vm = Intcode::new(large_prod);
        let outputs = vm.run_to_end()?;
        let result = outputs[0].to_string();
        assert_eq!(result.len(), 16);

        // Large number
        let large_num = vec![104,1125899906842624,99];
        let mut vm = Intcode::new(large_num);
        let outputs = vm.run_to_end()?;
        assert_eq!(outputs[0], 1125899906842624);

        Ok(())
    }
}
