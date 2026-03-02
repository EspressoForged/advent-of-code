use crate::utils::parser::{parse_str_lines, unsigned_number};
use crate::utils::read_input;
use anyhow::Result;
use nom::{character::complete::space1, sequence::separated_pair, IResult, Parser};
use std::collections::HashMap;

/// Parser for a single line of input: two numbers separated by spaces.
fn parse_line(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(unsigned_number, space1, unsigned_number).parse(input)
}

/// Core logic for Year 2024, Day 01
fn calculate_solution(pairs: &[(u64, u64)]) -> Result<(u64, u64)> {
    let mut left: Vec<u64> = pairs.iter().map(|p| p.0).collect();
    let mut right: Vec<u64> = pairs.iter().map(|p| p.1).collect();

    // --- Part 1: Total Distance ---
    left.sort_unstable();
    right.sort_unstable();

    let total_distance: u64 = left
        .iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum();

    // --- Part 2: Similarity Score ---
    let mut right_counts = HashMap::new();
    for &r in &right {
        *right_counts.entry(r).or_insert(0u64) += 1;
    }

    let similarity_score: u64 = left
        .iter()
        .map(|&l| l * right_counts.get(&l).unwrap_or(&0))
        .sum();

    Ok((total_distance, similarity_score))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2024, 1)?;
    let pairs = parse_str_lines(&content, parse_line)?;
    calculate_solution(&pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_day_01_solution() -> Result<()> {
        let pairs = parse_str_lines(TEST_INPUT, parse_line)?;
        let (p1, p2) = calculate_solution(&pairs)?;
        assert_eq!(p1, 11);
        assert_eq!(p2, 31);
        Ok(())
    }
}
