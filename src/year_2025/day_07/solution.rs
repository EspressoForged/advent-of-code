use advent_of_code::utils::parser::{parse_str_lines, Parse};
use advent_of_code::utils::{format_output, read_input};
use anyhow::{anyhow, Result};
use nom::{character::complete::one_of, multi::many1, IResult, Parser};
use std::collections::{HashMap, HashSet, VecDeque};

/// Represents a single row of the manifold diagram.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifoldRow {
    pub cells: Vec<char>,
}

impl Parse for ManifoldRow {
    fn parse(input: &str) -> IResult<&str, Self> {
        many1(one_of(".S^"))
            .map(|cells| ManifoldRow { cells })
            .parse(input)
    }
}

type CacheKey = (usize, usize);

fn count_timelines(
    grid: &[Vec<char>],
    r: usize,
    c: usize,
    memo: &mut HashMap<CacheKey, u64>,
) -> u64 {
    let rows = grid.len();
    let cols = grid[0].len();

    if r >= rows {
        return 1;
    }

    if let Some(&count) = memo.get(&(r, c)) {
        return count;
    }

    let char_at = grid[r][c];
    let result = match char_at {
        '.' | 'S' => count_timelines(grid, r + 1, c, memo),
        '^' => {
            let left_count = if c > 0 {
                count_timelines(grid, r + 1, c - 1, memo)
            } else {
                1
            };
            let right_count = if c + 1 < cols {
                count_timelines(grid, r + 1, c + 1, memo)
            } else {
                1
            };
            left_count + right_count
        }
        _ => 0,
    };

    memo.insert((r, c), result);
    result
}

fn solve_part_one(grid: &[Vec<char>], start: (usize, usize)) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut activated_splitters: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some((r, c)) = queue.pop_front() {
        let current_char = grid[r][c];
        match current_char {
            '^' => {
                activated_splitters.insert((r, c));
                if c > 0 {
                    let next_pos = (r, c - 1);
                    if visited.insert(next_pos) {
                        queue.push_back(next_pos);
                    }
                }
                if c + 1 < cols {
                    let next_pos = (r, c + 1);
                    if visited.insert(next_pos) {
                        queue.push_back(next_pos);
                    }
                }
            }
            '.' | 'S' => {
                if r + 1 < rows {
                    let next_pos = (r + 1, c);
                    if visited.insert(next_pos) {
                        queue.push_back(next_pos);
                    }
                }
            }
            _ => {}
        }
    }
    activated_splitters.len()
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(grid: &[Vec<char>]) -> Result<(u64, u64)> {
    let mut start_pos = None;
    for (r, row) in grid.iter().enumerate() {
        for (c, &val) in row.iter().enumerate() {
            if val == 'S' {
                start_pos = Some((r, c));
                break;
            }
        }
    }

    let (start_r, start_c) =
        start_pos.ok_or_else(|| anyhow!("No starting point 'S' found in grid"))?;

    let part_one = solve_part_one(grid, (start_r, start_c));
    let mut memo = HashMap::new();
    let part_two = count_timelines(grid, start_r, start_c, &mut memo);

    Ok((part_one as u64, part_two))
}

/// Entry point for Day 07
pub fn solve() -> Result<()> {
    let content = read_input(2025, 7)?;
    let rows = parse_str_lines(&content, ManifoldRow::parse)?;
    let grid: Vec<Vec<char>> = rows.into_iter().map(|r| r.cells).collect();
    let (p1, p2) = calculate_solution(&grid)?;
    println!("{}", format_output("07", p1, p2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_day_07_solution() -> Result<()> {
        let rows = parse_str_lines(TEST_INPUT, ManifoldRow::parse)?;
        let grid: Vec<Vec<char>> = rows.into_iter().map(|r| r.cells).collect();
        let (p1, p2) = calculate_solution(&grid)?;
        assert_eq!(p1, 21);
        assert_eq!(p2, 40);
        Ok(())
    }
}
