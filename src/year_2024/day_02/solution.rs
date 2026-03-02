use crate::utils::parser::{parse_str_lines, unsigned_number};
use crate::utils::read_input;
use anyhow::Result;
use nom::{character::complete::space1, multi::separated_list1, IResult, Parser};

/// Checks if a report is safe according to Part 1 rules.
fn is_safe(levels: &[i64]) -> bool {
    if levels.len() < 2 {
        return true;
    }

    let diffs: Vec<i64> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    let all_increasing = diffs.iter().all(|&d| (1..=3).contains(&d));
    let all_decreasing = diffs.iter().all(|&d| (-3..=-1).contains(&d));

    all_increasing || all_decreasing
}

/// Checks if a report is safe according to Part 2 rules (with Problem Dampener).
fn is_safe_with_dampener(levels: &[i64]) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified = levels.to_vec();
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }

    false
}

/// Parser for a single report line.
fn parse_report(input: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(space1, unsigned_number::<i64>).parse(input)
}

/// Core logic for solving Day 02
fn calculate_solution(reports: &[Vec<i64>]) -> Result<(u64, u64)> {
    let p1_count = reports.iter().filter(|r| is_safe(r)).count() as u64;
    let p2_count = reports.iter().filter(|r| is_safe_with_dampener(r)).count() as u64;

    Ok((p1_count, p2_count))
}

/// Entry point for Day 02
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2024, 2)?;
    let reports = parse_str_lines(&content, parse_report)?;
    calculate_solution(&reports)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn test_day_02_solution() -> Result<()> {
        let reports = parse_str_lines(TEST_INPUT, parse_report)?;
        let (p1, p2) = calculate_solution(&reports)?;
        assert_eq!(p1, 2);
        assert_eq!(p2, 4);
        Ok(())
    }

    #[test]
    fn test_day_02_full_input() -> Result<()> {
        let content = read_input(2024, 2)?;
        let reports = parse_str_lines(&content, parse_report)?;
        let (p1, p2) = calculate_solution(&reports)?;
        assert_eq!(p1, 402);
        assert_eq!(p2, 455);
        Ok(())
    }
}
