use crate::utils::parser::{parse_str_lines, unsigned_number, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{
    bytes::complete::tag, multi::separated_list1, sequence::separated_pair, IResult, Parser,
};

/// Represents a range of IDs: start (inclusive) to end (inclusive).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Range {
    pub start: u64,
    pub end: u64,
}

impl Parse for Range {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(unsigned_number, tag("-"), unsigned_number)
            .map(|(start, end)| Range { start, end })
            .parse(input)
    }
}

/// Represents a line containing a comma-separated list of ranges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeGroup {
    pub ranges: Vec<Range>,
}

impl Parse for RangeGroup {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_list1(tag(","), Range::parse)
            .map(|ranges| RangeGroup { ranges })
            .parse(input)
    }
}

/// Checks if a number is "invalid" according to the puzzle rules.
fn is_invalid_id(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let num_digits = n.ilog10() + 1;
    if !num_digits.is_multiple_of(2) {
        return false;
    }
    let half_len = num_digits / 2;
    let divisor = 10u64.pow(half_len);
    let upper = n / divisor;
    let lower = n % divisor;
    upper == lower
}

/// Checks if a number is "invalid" according to Part 2 rules.
fn is_recursive_id(n: u64) -> bool {
    if n < 10 {
        return false;
    }
    let num_digits = n.ilog10() + 1;
    for k in 2..=num_digits {
        if num_digits.is_multiple_of(k) {
            let len_pattern = num_digits / k;
            let mut multiplier = 0u64;
            let step = 10u64.pow(len_pattern);
            let mut current_power = 1u64;
            for _ in 0..k {
                multiplier += current_power;
                current_power *= step;
            }
            if n.is_multiple_of(multiplier) {
                return true;
            }
        }
    }
    false
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(groups: &[RangeGroup]) -> Result<(u64, u64)> {
    let mut invalid_id_sum: u64 = 0;
    let mut recursive_id_sum: u64 = 0;

    for group in groups {
        for range in &group.ranges {
            for id in range.start..=range.end {
                if is_invalid_id(id) {
                    invalid_id_sum += id;
                }
                if is_recursive_id(id) {
                    recursive_id_sum += id;
                }
            }
        }
    }

    Ok((invalid_id_sum, recursive_id_sum))
}

/// Entry point for Day 02
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2025), Day(2))?;
    let groups = parse_str_lines(&content, RangeGroup::parse)?;
    calculate_solution(&groups)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_day_02_solution() -> Result<()> {
        let groups = parse_str_lines(TEST_INPUT, RangeGroup::parse)?;
        let (p1, p2) = calculate_solution(&groups)?;
        assert_eq!(p1, 1227775554);
        assert_eq!(p2, 4174379265);
        Ok(())
    }

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(222222));
        assert!(is_invalid_id(1188511885));
        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(123456));
        assert!(!is_invalid_id(5));
    }

    #[test]
    fn test_is_recursive_id() {
        assert!(is_recursive_id(11));
        assert!(is_recursive_id(22));
        assert!(is_recursive_id(99));
        assert!(is_recursive_id(1010));
        assert!(is_recursive_id(222222));
        assert!(is_recursive_id(111));
        assert!(is_recursive_id(12341234));
        assert!(is_recursive_id(123123123));
        assert!(is_recursive_id(1212121212));
        assert!(is_recursive_id(1111111));
        assert!(!is_recursive_id(12));
        assert!(!is_recursive_id(101));
        assert!(!is_recursive_id(123456));
        assert!(!is_recursive_id(5));
    }
}

