use crate::utils::read_input;
use crate::year_2019::intcode::Intcode;
use anyhow::Result;

/// Solves Year 2019, Day 2: 1202 Program Alarm.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(2019, 2)?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();

    // Part 1: Restore the gravity assist program to the "1202 program alarm" state
    let part1 = run_with_params(&program, 12, 2)?;

    // Part 2: Find the noun and verb that produce the output 19690720
    let mut part2 = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            if run_with_params(&program, noun, verb)? == 19690720 {
                part2 = 100 * noun + verb;
                break;
            }
        }
        if part2 != 0 {
            break;
        }
    }

    Ok((part1 as u64, part2 as u64))
}

fn run_with_params(program: &[i64], noun: i64, verb: i64) -> Result<i64> {
    let mut mem = program.to_vec();
    mem[1] = noun;
    mem[2] = verb;
    let mut vm = Intcode::new(mem);
    vm.run()?;
    Ok(vm.memory[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_02_example() {
        let mut vm = Intcode::new(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        vm.run().unwrap();
        assert_eq!(vm.memory[3], 70);
        assert_eq!(vm.memory[0], 3500);
    }
}
