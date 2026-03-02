use crate::utils::parser::{parse_str_lines, Parse};
use crate::utils::read_input;
use anyhow::Result;
use nom::{character::complete::one_of, multi::many1, IResult, Parser};

/// Represents a single row of the grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GridRow {
    pub cells: Vec<char>,
}

impl Parse for GridRow {
    fn parse(input: &str) -> IResult<&str, Self> {
        many1(one_of(".@"))
            .map(|cells| GridRow { cells })
            .parse(input)
    }
}

/// Checks the 8 neighbors of a specific cell (row, col).
fn count_neighbors(grid: &[Vec<char>], r: usize, c: usize) -> usize {
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    let r = r as isize;
    let c = c as isize;

    let mut count = 0;
    for dr in -1..=1 {
        for dc in -1..=1 {
            if dr == 0 && dc == 0 {
                continue;
            }
            let nr = r + dr;
            let nc = c + dc;
            if nr >= 0 && nr < rows && nc >= 0 && nc < cols && grid[nr as usize][nc as usize] == '@'
            {
                count += 1;
            }
        }
    }
    count
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(mut grid: Vec<Vec<char>>) -> Result<(u64, u64)> {
    if grid.is_empty() {
        return Ok((0, 0));
    }

    let rows = grid.len();
    let cols = grid[0].len();
    let mut first_round = 0;
    let mut total_removed = 0;
    let mut round = 0;

    loop {
        let mut to_remove = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    let neighbors = count_neighbors(&grid, r, c);
                    if neighbors < 4 {
                        to_remove.push((r, c));
                    }
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        let count = to_remove.len();
        if round == 0 {
            first_round = count;
        }
        total_removed += count;

        for (r, c) in to_remove {
            grid[r][c] = '.';
        }
        round += 1;
    }

    Ok((first_round as u64, total_removed as u64))
}

/// Entry point for Day 04
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2025, 4)?;
    let rows = parse_str_lines(&content, GridRow::parse)?;
    let grid: Vec<Vec<char>> = rows.into_iter().map(|r| r.cells).collect();
    calculate_solution(grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

    #[test]
    fn test_day_04_solution() -> Result<()> {
        let rows = parse_str_lines(TEST_INPUT, GridRow::parse)?;
        let grid: Vec<Vec<char>> = rows.into_iter().map(|r| r.cells).collect();
        let (_p1, p2) = calculate_solution(grid)?;
        assert_eq!(p2, 43);
        Ok(())
    }
}
