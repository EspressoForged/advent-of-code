use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};
use num::Integer;

/// Solves Year 2019, Day 7: Amplification Circuit.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(7))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let part1 = find_max_thruster_signal(&program, 0..=4, false)?;
    let part2 = find_max_thruster_signal(&program, 5..=9, true)?;

    Ok((part1 as u64, part2 as u64))
}

fn find_max_thruster_signal(
    program: &[i64],
    phase_range: std::ops::RangeInclusive<i64>,
    feedback_loop: bool,
) -> Result<i64> {
    let phases: Vec<i64> = phase_range.collect();
    let mut max_signal = 0;

    for permutation in get_permutations(phases) {
        let signal = if feedback_loop {
            run_feedback_loop(program, &permutation)?
        } else {
            run_serial_amplifiers(program, &permutation)?
        };
        if signal > max_signal {
            max_signal = signal;
        }
    }

    Ok(max_signal)
}

fn run_serial_amplifiers(program: &[i64], phases: &[i64]) -> Result<i64> {
    let mut current_signal = 0;
    for &phase in phases {
        let mut vm = Intcode::new(program.to_vec());
        vm.add_input(phase);
        vm.add_input(current_signal);
        let outputs = vm.run_to_end()?;
        current_signal = *outputs.last().context("No output from amplifier")?;
    }
    Ok(current_signal)
}

fn run_feedback_loop(program: &[i64], phases: &[i64]) -> Result<i64> {
    let mut vms: Vec<Intcode> = phases
        .iter()
        .map(|&phase| {
            let mut vm = Intcode::new(program.to_vec());
            vm.add_input(phase);
            vm
        })
        .collect();

    let mut current_signal = 0;
    let mut last_e_output = 0;
    let mut all_halted = false;
    let last_index = vms.len() - 1;

    while !all_halted {
        for (i, vm) in vms.iter_mut().enumerate() {
            vm.add_input(current_signal);

            // Step once to get the next event (Output, Halted, or NeedsInput).
            // The Intcode::step() method internally loops through instructions.
            match vm.step()? {
                Status::Output(out) => {
                    current_signal = out;
                    if i == last_index {
                        last_e_output = out;
                    }
                }
                Status::Halted => {
                    if i == last_index {
                        all_halted = true;
                    }
                }
                Status::NeedsInput => {}
            }
        }
    }

    Ok(last_e_output)
}

fn get_permutations(mut items: Vec<i64>) -> Vec<Vec<i64>> {
    let mut results = Vec::new();
    fn generate(k: usize, items: &mut Vec<i64>, results: &mut Vec<Vec<i64>>) {
        if k == 1 {
            results.push(items.clone());
            return;
        }
        for i in 0..k {
            generate(k - 1, items, results);
            if k.is_even() {
                items.swap(i, k - 1);
            } else {
                items.swap(0, k - 1);
            }
        }
    }
    let len = items.len();
    generate(len, &mut items, &mut results);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_07_part1_examples() -> Result<()> {
        let prog1 = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(find_max_thruster_signal(&prog1, 0..=4, false)?, 43210);

        let prog2 = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(find_max_thruster_signal(&prog2, 0..=4, false)?, 54321);

        let prog3 = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(find_max_thruster_signal(&prog3, 0..=4, false)?, 65210);
        Ok(())
    }

    #[test]
    fn test_day_07_part2_examples() -> Result<()> {
        let prog1 = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
            1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(find_max_thruster_signal(&prog1, 5..=9, true)?, 139629729);

        let prog2 = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(find_max_thruster_signal(&prog2, 5..=9, true)?, 18216);
        Ok(())
    }
}
