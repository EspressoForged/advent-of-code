use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};

/// Solves Year 2019, Day 19: Tractor Beam.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(19))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let part1 = count_affected_points(&program, 50, 50)?;
    let part2 = find_square(&program, 100)?;

    Ok((part1 as u64, part2 as u64))
}

fn is_affected(program: &[i64], x: i64, y: i64) -> Result<bool> {
    let mut vm = Intcode::new(program.to_vec());
    vm.add_input(x);
    vm.add_input(y);
    
    match vm.step()? {
        Status::Output(val) => Ok(val == 1),
        _ => Err(anyhow::anyhow!("VM did not produce output for coordinate ({}, {})", x, y)),
    }
}

fn count_affected_points(program: &[i64], width: i64, height: i64) -> Result<u64> {
    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            if is_affected(program, x, y)? {
                count += 1;
            }
        }
    }
    Ok(count)
}

fn find_square(program: &[i64], size: i64) -> Result<u64> {
    let mut x = 0;
    let offset = size - 1;
    // Start from y=size to ensure y-offset is valid and skip the origin
    for y in size.. {
        // Find left edge of beam for current row y
        while !is_affected(program, x, y)? {
            x += 1;
            // Safety break if we wander too far, but beam is expected to exist
            if x > y * 2 {
                break;
            }
        }
        
        // If bottom-left is (x, y), check if top-right (x+offset, y-offset) is in beam
        if is_affected(program, x + offset, y - offset)? {
            // Found the square! Top-left corner is (x, y - offset)
            return Ok((x as u64) * 10000 + (y as u64 - offset as u64));
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_10x10() -> Result<()> {
        let input = read_input(Year(2019), Day(19))?;
        let program: Vec<i64> = input
            .trim()
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        
        // The problem description example uses a different beam that has 27 points in 10x10.
        // For my specific input, the 10x10 area has 7 points.
        let result = count_affected_points(&program, 10, 10)?;
        assert_eq!(result, 7);
        Ok(())
    }
}
