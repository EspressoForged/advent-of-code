use crate::utils::parser::{parse_str_lines, Parse};
use crate::utils::read_input;
use anyhow::Result;
use nom::{character::complete::digit1, combinator::map, IResult, Parser};

/// Represents a bank of batteries as a sequence of single-digit joltage values.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatteryBank {
    pub joltages: Vec<u8>,
}

impl Parse for BatteryBank {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(digit1, |s: &str| BatteryBank {
            joltages: s
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(|d| d as u8)
                .collect(),
        })
        .parse(input)
    }
}

/// Generic function to find the largest number formed by a subsequence of length `k`.
fn find_max_subsequence_val(bank: &[u8], k: usize) -> u64 {
    let n = bank.len();
    if n < k {
        return 0;
    }

    let mut current_pos = 0;
    let mut result_val: u64 = 0;

    for i in 0..k {
        let digits_needed_after_this = k - 1 - i;
        let end_search = n - digits_needed_after_this;
        let slice = &bank[current_pos..end_search];

        let (offset, &digit) = slice
            .iter()
            .enumerate()
            .max_by(|(i_a, val_a), (i_b, val_b)| {
                let val_cmp = val_a.cmp(val_b);
                if val_cmp == std::cmp::Ordering::Equal {
                    i_b.cmp(i_a)
                } else {
                    val_cmp
                }
            })
            .expect("Slice should not be empty based on logic");

        result_val = result_val * 10 + (digit as u64);
        current_pos += offset + 1;
    }

    result_val
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(banks: &[BatteryBank]) -> Result<(u64, u64)> {
    let total_joltage_p1: u64 = banks
        .iter()
        .map(|bank| find_max_subsequence_val(&bank.joltages, 2))
        .sum();

    let total_joltage_p2: u64 = banks
        .iter()
        .map(|bank| find_max_subsequence_val(&bank.joltages, 12))
        .sum();

    Ok((total_joltage_p1, total_joltage_p2))
}

/// Entry point for Day 03
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2025, 3)?;
    let banks = parse_str_lines(&content, BatteryBank::parse)?;
    calculate_solution(&banks)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "987654321111111\n811111111111119\n234234234234278\n818181911112111";

    #[test]
    fn test_day_03_solution() -> Result<()> {
        let banks = parse_str_lines(TEST_INPUT, BatteryBank::parse)?;
        let (p1, p2) = calculate_solution(&banks)?;
        assert_eq!(p1, 357);
        assert_eq!(p2, 3121910778619);
        Ok(())
    }

    #[test]
    fn test_part_2_examples() {
        let b1 = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 1, 1, 1, 1, 1, 1];
        assert_eq!(find_max_subsequence_val(&b1, 12), 987654321111);

        let b2 = vec![8, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 9];
        assert_eq!(find_max_subsequence_val(&b2, 12), 811111111119);

        let b3 = vec![2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8];
        assert_eq!(find_max_subsequence_val(&b3, 12), 434234234278);

        let b4 = vec![8, 1, 8, 1, 8, 1, 9, 1, 1, 1, 1, 2, 1, 1, 1];
        assert_eq!(find_max_subsequence_val(&b4, 12), 888911112111);
    }
}
