use crate::utils::parser::{parse_str_lines, unsigned_number, Parse};
use crate::utils::read_input;
use anyhow::Result;
use nom::{character::complete::one_of, sequence::pair, IResult, Parser};

/// The parsed instruction: Direction ('L' or 'R') and Amount.
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub dir: char,
    pub amount: u64,
}

impl Parse for Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        pair(one_of("LR"), unsigned_number)
            .map(|(dir, amount)| Instruction { dir, amount })
            .parse(input)
    }
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(instructions: &[Instruction]) -> Result<(u64, u64)> {
    let mut abs_pos: i64 = 50;
    let mut p1_count: u64 = 0;
    let mut p2_count: u64 = 0;

    for inst in instructions {
        let amount = inst.amount as i64;
        let old_pos = abs_pos;

        match inst.dir {
            'R' => {
                abs_pos += amount;
                let clicks = abs_pos.div_euclid(100) - old_pos.div_euclid(100);
                p2_count += clicks as u64;
            }
            'L' => {
                abs_pos -= amount;
                let clicks = (old_pos - 1).div_euclid(100) - (abs_pos - 1).div_euclid(100);
                p2_count += clicks as u64;
            }
            _ => unreachable!("Parser guarantees L or R"),
        }

        if abs_pos.rem_euclid(100) == 0 {
            p1_count += 1;
        }
    }

    Ok((p1_count, p2_count))
}

/// Entry point for Day 01
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2025, 1)?;
    let instructions = parse_str_lines(&content, Instruction::parse)?;
    calculate_solution(&instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

    #[test]
    fn test_day_01_solution() -> Result<()> {
        let instructions = parse_str_lines(TEST_INPUT, Instruction::parse)?;
        let (p1, p2) = calculate_solution(&instructions)?;
        assert_eq!(p1, 3);
        assert_eq!(p2, 6);
        Ok(())
    }
}
