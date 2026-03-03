use crate::utils::{read_input, Year, Day};
use anyhow::{Context, Result};
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn bit(self) -> u8 {
        1 << (self as u8)
    }
}

/// Solves Year 2024, Day 6: Guard Gallivant.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2024), Day(6))?;
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    let (start_pos, start_dir) = find_start(&grid).context("Start position not found in grid")?;
    let original_visited = simulate_part1(&grid, start_pos, start_dir);

    // Part 1: Distinct positions visited
    let part1 = original_visited.len() as u64;

    // Part 2: Obstacles causing loops
    // We only need to check positions the guard would have visited (excluding start).
    let width = grid.first().map_or(0, |r| r.len()) as i32;
    let height = grid.len() as i32;

    let part2 = original_visited
        .par_iter()
        .filter(|&&pos| pos != start_pos)
        .filter(|&&pos| is_looping(&grid, start_pos, start_dir, pos, width, height))
        .count() as u64;

    Ok((part1, part2))
}

fn find_start(grid: &[Vec<char>]) -> Option<((i32, i32), Direction)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let pos = (x as i32, y as i32);
            match c {
                '^' => return Some((pos, Direction::Up)),
                'v' => return Some((pos, Direction::Down)),
                '<' => return Some((pos, Direction::Left)),
                '>' => return Some((pos, Direction::Right)),
                _ => {}
            }
        }
    }
    None
}

fn simulate_part1(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    start_dir: Direction,
) -> HashSet<(i32, i32)> {
    let mut visited = HashSet::new();
    let mut pos = start_pos;
    let mut dir = start_dir;
    let height = grid.len() as i32;
    let width = grid.first().map_or(0, |r| r.len()) as i32;

    loop {
        visited.insert(pos);
        let (dx, dy) = dir.delta();
        let next_pos = (pos.0 + dx, pos.1 + dy);

        if next_pos.0 < 0 || next_pos.0 >= width || next_pos.1 < 0 || next_pos.1 >= height {
            break;
        }

        if grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }
    }

    visited
}

fn is_looping(
    grid: &[Vec<char>],
    start_pos: (i32, i32),
    start_dir: Direction,
    extra_obstacle: (i32, i32),
    width: i32,
    height: i32,
) -> bool {
    // 2D bitmask array for tracking (pos, dir)
    let mut visited_states = vec![0u8; (width * height) as usize];

    let mut pos = start_pos;
    let mut dir = start_dir;

    loop {
        let idx = (pos.1 * width + pos.0) as usize;
        let bit = dir.bit();

        if (visited_states[idx] & bit) != 0 {
            return true; // Loop detected
        }
        visited_states[idx] |= bit;

        let (dx, dy) = dir.delta();
        let next_pos = (pos.0 + dx, pos.1 + dy);

        if next_pos.0 < 0 || next_pos.0 >= width || next_pos.1 < 0 || next_pos.1 >= height {
            return false; // Exit area
        }

        if next_pos == extra_obstacle || grid[next_pos.1 as usize][next_pos.0 as usize] == '#' {
            dir = dir.turn_right();
        } else {
            pos = next_pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_06_example() {
        let example = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let grid: Vec<Vec<char>> = example.lines().map(|l| l.chars().collect()).collect();

        let (start_pos, start_dir) = find_start(&grid).unwrap();
        let original_visited = simulate_part1(&grid, start_pos, start_dir);

        let width = grid[0].len() as i32;
        let height = grid.len() as i32;

        let part2 = original_visited
            .iter()
            .filter(|&&pos| pos != start_pos)
            .filter(|&&pos| is_looping(&grid, start_pos, start_dir, pos, width, height))
            .count();

        assert_eq!(original_visited.len(), 41);
        assert_eq!(part2, 6);
    }
}

