use crate::utils::{read_input, Year, Day};
use anyhow::{Context, Result};

/// Solves Year 2015, Day 1: Not Quite Lisp.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2015), Day(1)).context("Failed to read input for day 01")?;

    let mut floor: i32 = 0;
    let mut part2: Option<usize> = None;

    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {}
        }
        if part2.is_none() && floor == -1 {
            part2 = Some(i + 1);
        }
    }

    let p2_result = part2.context("Santa never entered the basement")? as u64;

    Ok((floor as u64, p2_result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_01_examples() -> Result<()> {
        let cases = [
            ("(())", 0),
            ("()()", 0),
            ("(((", 3),
            ("(()(()(", 3),
            ("))(((((", 3),
            ("())", -1),
            ("))(", -1),
            (")))", -3),
            (")())())", -3),
        ];

        for (input, expected) in cases {
            let mut floor = 0;
            for c in input.chars() {
                match c {
                    '(' => floor += 1,
                    ')' => floor -= 1,
                    _ => {}
                }
            }
            assert_eq!(floor, expected);
        }
        Ok(())
    }

    #[test]
    fn test_day_01_part2_examples() {
        let cases = [(")", 1), ("()())", 5)];

        for (input, expected) in cases {
            let mut floor = 0;
            let mut pos = None;
            for (i, c) in input.chars().enumerate() {
                match c {
                    '(' => floor += 1,
                    ')' => floor -= 1,
                    _ => {}
                }
                if floor == -1 {
                    pos = Some(i + 1);
                    break;
                }
            }
            assert_eq!(pos.unwrap(), expected);
        }
    }
}

