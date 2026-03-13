use anyhow::{anyhow, Result};
use std::collections::HashSet;
use crate::utils::{read_input, Year, Day};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Blizzard {
    r: i32,
    c: i32,
    dir: Direction,
}

struct Map {
    width: i32,
    height: i32,
    blizzards: Vec<Blizzard>,
}

impl Map {
    // Optimization: pre-calculate which positions are safe at a given time
    fn get_safe_positions(&self, time: i32) -> HashSet<(i32, i32)> {
        let mut blizzards = HashSet::new();
        for b in &self.blizzards {
            let (br, bc) = match b.dir {
                Direction::Right => (b.r, (b.c + time).rem_euclid(self.width)),
                Direction::Left => (b.r, (b.c - time).rem_euclid(self.width)),
                Direction::Down => ((b.r + time).rem_euclid(self.height), b.c),
                Direction::Up => ((b.r - time).rem_euclid(self.height), b.c),
            };
            blizzards.insert((br, bc));
        }

        let mut safe = HashSet::new();
        for r in 0..self.height {
            for c in 0..self.width {
                if !blizzards.contains(&(r, c)) {
                    safe.insert((r, c));
                }
            }
        }
        // Start and end are always safe
        safe.insert((-1, 0));
        safe.insert((self.height, self.width - 1));
        safe
    }
}

fn find_path(map: &Map, start: (i32, i32), end: (i32, i32), start_time: u64) -> Result<u64> {
    let mut current_positions = HashSet::new();
    current_positions.insert(start);

    let mut time = start_time;
    while !current_positions.contains(&end) {
        time += 1;
        let safe_positions = map.get_safe_positions(time as i32);
        let mut next_positions = HashSet::new();

        for &(r, c) in &current_positions {
            // Options: Wait, Up, Down, Left, Right
            let moves = [(r, c), (r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)];
            for m in moves {
                if safe_positions.contains(&m) {
                    next_positions.insert(m);
                }
            }
        }
        current_positions = next_positions;
        if current_positions.is_empty() {
            return Err(anyhow!("No path found from {:?} to {:?} at time {}", start, end, time));
        }
    }
    Ok(time)
}

/// Core logic for Year 2022, Day 24
/// # Errors
/// Returns an error if input is malformed.
pub fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let total_height = lines.len() as i32;
    let total_width = lines[0].len() as i32;
    let height = total_height - 2;
    let width = total_width - 2;

    let mut blizzards = Vec::new();
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let dir = match ch {
                '>' => Some(Direction::Right),
                '<' => Some(Direction::Left),
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                _ => None,
            };
            if let Some(d) = dir {
                blizzards.push(Blizzard {
                    r: r as i32 - 1,
                    c: c as i32 - 1,
                    dir: d,
                });
            }
        }
    }

    let map = Map { width, height, blizzards };

    let start = (-1, 0);
    let end = (height, width - 1);

    // Trip 1: Start to End
    let time1 = find_path(&map, start, end, 0)?;
    
    // Trip 2: End back to Start
    let time2 = find_path(&map, end, start, time1)?;

    // Trip 3: Start to End again
    let time3 = find_path(&map, start, end, time2)?;

    Ok((time1, time3))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2022), Day(24))?;
    calculate_solution(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_example() -> Result<()> {
        let (p1, p2) = calculate_solution(EXAMPLE)?;
        assert_eq!(p1, 18);
        assert_eq!(p2, 54);
        Ok(())
    }
}
