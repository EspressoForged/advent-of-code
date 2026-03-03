use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};

/// Solves Year 2019, Day 21: Springdroid Adventure.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(21))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let part1 = run_springscript(&program, &["NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK"])?;
    let part2 = run_springscript(&program, &["NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T", "OR H T", "AND T J", "RUN"])?;

    Ok((part1, part2))
}

fn run_springscript(program: &[i64], script: &[&str]) -> Result<u64> {
    let mut vm = Intcode::new(program.to_vec());
    let mut script_input = script.join("\n");
    script_input.push('\n');

    for c in script_input.chars() {
        vm.add_input(c as i64);
    }

    let mut last_output = 0;
    loop {
        match vm.step()? {
            Status::Halted => break,
            Status::Output(val) => {
                if val > 255 {
                    last_output = val;
                } else {
                    // Debug print ASCII if it fails?
                    // print!("{}", val as u8 as char);
                }
            }
            Status::NeedsInput => {
                return Err(anyhow::anyhow!("VM needs input but script is finished"));
            }
        }
    }

    Ok(last_output as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        let input = read_input(Year(2019), Day(21))?;
        let program: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let result = run_springscript(&program, &["NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK"])?;
        assert!(result > 0);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        let input = read_input(Year(2019), Day(21))?;
        let program: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let result = run_springscript(&program, &["NOT A J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "NOT E T", "NOT T T", "OR H T", "AND T J", "RUN"])?;
        assert!(result > 0);
        Ok(())
    }
}
