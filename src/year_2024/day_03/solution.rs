use crate::utils::parser::unsigned_number;
use crate::utils::read_input;
use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::value,
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
}

/// Parses a `mul(X,Y)` instruction.
fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    delimited(
        tag("mul("),
        separated_pair(unsigned_number, tag(","), unsigned_number),
        tag(")"),
    )
    .map(|(x, y)| Instruction::Mul(x, y))
    .parse(input)
}

/// Parses a `do()` instruction.
fn parse_do(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Do, tag("do()")).parse(input)
}

/// Parses a `don't()` instruction.
fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Dont, tag("don't()")).parse(input)
}

/// Core logic for Year 2024, Day 03
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    let mut enabled = true;

    let mut remaining = input;
    while !remaining.is_empty() {
        // Try to parse any of the instructions
        let res: IResult<&str, Instruction> = alt((parse_mul, parse_do, parse_dont)).parse(remaining);

        match res {
            Ok((next_input, instruction)) => {
                match instruction {
                    Instruction::Mul(x, y) => {
                        let product = x * y;
                        p1_sum += product;
                        if enabled {
                            p2_sum += product;
                        }
                    }
                    Instruction::Do => enabled = true,
                    Instruction::Dont => enabled = false,
                }
                remaining = next_input;
            }
            Err(_) => {
                // If no instruction matches, skip one character and try again
                let res: IResult<&str, char> = anychar.parse(remaining);
                if let Ok((next_input, _)) = res {
                    remaining = next_input;
                } else {
                    break;
                }
            }
        }
    }

    Ok((p1_sum, p2_sum))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2024, 3)?;
    calculate_solution(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_P1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_P2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_day_03_part1() -> Result<()> {
        let (p1, _) = calculate_solution(TEST_INPUT_P1)?;
        assert_eq!(p1, 161);
        Ok(())
    }

    #[test]
    fn test_day_03_part2() -> Result<()> {
        let (_, p2) = calculate_solution(TEST_INPUT_P2)?;
        assert_eq!(p2, 48);
        Ok(())
    }
}
