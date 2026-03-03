use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Oxygen,
}

type Grid = HashMap<(i32, i32), Tile>;

/// Solves Year 2019, Day 15: Oxygen System.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(15))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let (map, oxygen_pos) = explore_map(&program)?;

    // Part 1: BFS distance to oxygen
    let part1 = find_shortest_path((0, 0), oxygen_pos, &map)
        .context("Could not find path to oxygen")?;

    // Part 2: Max distance from oxygen to any empty tile
    let part2 = find_max_distance(oxygen_pos, &map);

    Ok((part1 as u64, part2 as u64))
}

fn explore_map(program: &[i64]) -> Result<(Grid, (i32, i32))> {
    let mut map = HashMap::new();
    map.insert((0, 0), Tile::Empty);
    let mut oxygen_pos = (0, 0);

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), Intcode::new(program.to_vec())));

    let mut visited = HashMap::new();
    visited.insert((0, 0), 0);

    while let Some((pos, vm)) = queue.pop_front() {
        // Try all 4 directions
        for dir in 1..=4 {
            let next_pos = match dir {
                1 => (pos.0, pos.1 - 1), // North
                2 => (pos.0, pos.1 + 1), // South
                3 => (pos.0 - 1, pos.1), // West
                4 => (pos.0 + 1, pos.1), // East
                _ => unreachable!(),
            };

            if map.contains_key(&next_pos) {
                continue;
            }

            let mut next_vm = vm.clone();
            next_vm.add_input(dir);
            
            match next_vm.step()? {
                Status::Output(0) => {
                    map.insert(next_pos, Tile::Wall);
                }
                Status::Output(1) => {
                    map.insert(next_pos, Tile::Empty);
                    queue.push_back((next_pos, next_vm));
                }
                Status::Output(2) => {
                    map.insert(next_pos, Tile::Oxygen);
                    oxygen_pos = next_pos;
                    queue.push_back((next_pos, next_vm));
                }
                _ => return Err(anyhow::anyhow!("Unexpected VM status during exploration")),
            }
        }
    }

    Ok((map, oxygen_pos))
}

fn find_shortest_path(start: (i32, i32), target: (i32, i32), map: &Grid) -> Option<u32> {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashMap::new();
    visited.insert(start, 0);

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == target {
            return Some(dist);
        }

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let next_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(&tile) = map.get(&next_pos) {
                if tile != Tile::Wall && !visited.contains_key(&next_pos) {
                    visited.insert(next_pos, dist + 1);
                    queue.push_back((next_pos, dist + 1));
                }
            }
        }
    }
    None
}

fn find_max_distance(start: (i32, i32), map: &Grid) -> u32 {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    let mut max_dist = 0;

    while let Some((pos, dist)) = queue.pop_front() {
        max_dist = max_dist.max(dist);

        for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            let next_pos = (pos.0 + dx, pos.1 + dy);
            if let Some(&tile) = map.get(&next_pos) {
                if tile != Tile::Wall && !visited.contains_key(&next_pos) {
                    visited.insert(next_pos, dist + 1);
                    queue.push_back((next_pos, dist + 1));
                }
            }
        }
    }
    max_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shortest_path_example() {
        let mut map = Grid::new();
        // Tracing the example from the problem description:
        // Start at (0,0)
        map.insert((0,0), Tile::Empty);
        // North (1) -> 0 (Wall)
        map.insert((0,-1), Tile::Wall);
        // East (4) -> 1 (Empty)
        map.insert((1,0), Tile::Empty);
        // North (1) -> 0 (Wall)
        map.insert((1,-1), Tile::Wall);
        // South (2) -> 0 (Wall)
        map.insert((1,1), Tile::Wall);
        // East (4) -> 0 (Wall)
        map.insert((2,0), Tile::Wall);
        // West (3) -> 1 (Move back to 0,0 - already Empty)
        // West (3) -> 0 (Wall)
        map.insert((-1,0), Tile::Wall);
        // South (2) -> 1 (Empty)
        map.insert((0,1), Tile::Empty);
        // South (2) -> 0 (Wall)
        map.insert((0,2), Tile::Wall);
        // West (3) -> 2 (Oxygen)
        map.insert((-1,1), Tile::Oxygen);

        assert_eq!(find_shortest_path((0, 0), (-1, 1), &map), Some(2));
    }

    #[test]
    fn test_oxygen_spread() {
        let example = " ##   
#..## 
#.#..#
#.O.# 
 ###  ";
        let mut map = Grid::new();
        let mut oxygen_pos = (0, 0);
        for (y, line) in example.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = (x as i32, y as i32);
                match c {
                    '#' => { map.insert(pos, Tile::Wall); }
                    '.' => { map.insert(pos, Tile::Empty); }
                    'O' => {
                        map.insert(pos, Tile::Oxygen);
                        oxygen_pos = pos;
                    }
                    _ => {}
                }
            }
        }
        assert_eq!(find_max_distance(oxygen_pos, &map), 4);
    }
}
