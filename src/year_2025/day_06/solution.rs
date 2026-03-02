use advent_of_code::utils::{format_output, read_input};
use anyhow::{Context, Result};

/// Represents a math problem.
#[derive(Debug)]
struct MathProblem {
    numbers: Vec<u64>,
    operation: char, // '+' or '*'
}

impl MathProblem {
    fn solve(&self) -> u64 {
        match self.operation {
            '+' => self.numbers.iter().sum(),
            '*' => self.numbers.iter().product(),
            _ => 0,
        }
    }
}

/// Helper to determine if a specific column index is empty (all spaces) across all lines.
fn is_column_empty(lines: &[Vec<char>], col_idx: usize) -> bool {
    lines
        .iter()
        .all(|line| *line.get(col_idx).unwrap_or(&' ') == ' ')
}

/// Part 1 Parser: Rows are numbers, bottom row is operator.
fn parse_block_part1(lines: &[Vec<char>], col_start: usize, col_end: usize) -> Result<MathProblem> {
    let mut numbers = Vec::new();
    let mut op = None;
    let last_row_idx = lines.len().saturating_sub(1);

    for (row_idx, line) in lines.iter().enumerate() {
        let chunk: String = (col_start..col_end)
            .map(|c| *line.get(c).unwrap_or(&' '))
            .collect();
        let trimmed = chunk.trim();

        if row_idx == last_row_idx {
            if let Some(c) = trimmed.chars().next() {
                op = Some(c);
            }
        } else if !trimmed.is_empty() {
            let num = trimmed
                .parse::<u64>()
                .with_context(|| format!("Part 1: Failed to parse number from '{}'", trimmed))?;
            numbers.push(num);
        }
    }

    Ok(MathProblem {
        numbers,
        operation: op.context("Part 1: No operator found in block")?,
    })
}

/// Part 2 Parser: Columns are numbers (read top-to-bottom), bottom row is operator.
fn parse_block_part2(lines: &[Vec<char>], col_start: usize, col_end: usize) -> Result<MathProblem> {
    let mut numbers = Vec::new();
    let mut op = None;
    let last_row_idx = lines.len().saturating_sub(1);

    for c in col_start..col_end {
        let char_at_bottom = *lines[last_row_idx].get(c).unwrap_or(&' ');
        if !char_at_bottom.is_whitespace() {
            op = Some(char_at_bottom);
            break;
        }
    }

    for c in col_start..col_end {
        let mut digit_str = String::new();
        for line in lines.iter().take(last_row_idx) {
            let char_at_cell = *line.get(c).unwrap_or(&' ');
            if char_at_cell.is_ascii_digit() {
                digit_str.push(char_at_cell);
            }
        }

        if !digit_str.is_empty() {
            let num = digit_str
                .parse::<u64>()
                .with_context(|| format!("Part 2: Failed to parse number from col {}", c))?;
            numbers.push(num);
        }
    }

    Ok(MathProblem {
        numbers,
        operation: op.context("Part 2: No operator found in block")?,
    })
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(content: &str) -> Result<(u64, u64)> {
    let lines: Vec<Vec<char>> = content.lines().map(|l| l.chars().collect()).collect();
    if lines.is_empty() {
        return Ok((0, 0));
    }

    let max_width = lines.iter().map(|l| l.len()).max().unwrap_or(0);
    let mut p1_total: u64 = 0;
    let mut p2_total: u64 = 0;
    let mut block_start = None;

    for col in 0..=max_width {
        let empty = is_column_empty(&lines, col);
        if !empty {
            if block_start.is_none() {
                block_start = Some(col);
            }
        } else if let Some(start) = block_start {
            if let Ok(p) = parse_block_part1(&lines, start, col) {
                p1_total += p.solve();
            }
            if let Ok(p) = parse_block_part2(&lines, start, col) {
                p2_total += p.solve();
            }
            block_start = None;
        }
    }

    Ok((p1_total, p2_total))
}

/// Entry point for Day 06
pub fn solve() -> Result<()> {
    let content = read_input(2025, 6)?;
    let (p1, p2) = calculate_solution(&content)?;
    println!("{}", format_output("06", p1, p2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_day_06_solution() -> Result<()> {
        let (p1, p2) = calculate_solution(TEST_INPUT)?;
        assert_eq!(p1, 4277556);
        assert_eq!(p2, 3263827);
        Ok(())
    }
}
