use crate::utils::{read_input, Day, Year};
use anyhow::{anyhow, Result};
use std::collections::{HashSet, VecDeque};

/// Solves Year 2023, Day 10: Pipe Maze.
///
/// # Errors
/// Returns an error if the input cannot be read or the loop cannot be found.
///
/// # Examples
/// ```
/// let result = solve();
/// ```
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2023), Day(10))?;
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let (part1, loop_coords, start_pos, actual_start_char) = find_loop(&grid)?;
    let part2 = count_enclosed_tiles(&grid, &loop_coords, start_pos, actual_start_char);

    Ok((part1 as u64, part2 as u64))
}

type LoopInfo = (usize, HashSet<(usize, usize)>, (usize, usize), char);

fn find_loop(grid: &[Vec<char>]) -> Result<LoopInfo> {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut start_pos = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char == 'S' {
                start_pos = (r, c);
                break;
            }
        }
    }

    let (r, c) = start_pos;
    let mut connected = Vec::new();

    // Check neighbors
    // North
    if r > 0 && matches!(grid[r - 1][c], '|' | '7' | 'F') {
        connected.push((r - 1, c));
    }
    // South
    if r + 1 < rows && matches!(grid[r + 1][c], '|' | 'L' | 'J') {
        connected.push((r + 1, c));
    }
    // West
    if c > 0 && matches!(grid[r][c - 1], '-' | 'L' | 'F') {
        connected.push((r, c - 1));
    }
    // East
    if c + 1 < cols && matches!(grid[r][c + 1], '-' | 'J' | '7') {
        connected.push((r, c + 1));
    }

    if connected.len() != 2 {
        return Err(anyhow!("Start position S does not have exactly 2 connected pipes"));
    }

    // Determine actual shape of S
    let n1 = connected[0];
    let n2 = connected[1];
    let actual_start_char = match ((n1.0 as i32 - r as i32, n1.1 as i32 - c as i32), (n2.0 as i32 - r as i32, n2.1 as i32 - c as i32)) {
        ((-1, 0), (1, 0)) | ((1, 0), (-1, 0)) => '|',
        ((0, -1), (0, 1)) | ((0, 1), (0, -1)) => '-',
        ((-1, 0), (0, 1)) | ((0, 1), (-1, 0)) => 'L',
        ((-1, 0), (0, -1)) | ((0, -1), (-1, 0)) => 'J',
        ((1, 0), (0, -1)) | ((0, -1), (1, 0)) => '7',
        ((1, 0), (0, 1)) | ((0, 1), (1, 0)) => 'F',
        _ => return Err(anyhow!("Could not determine shape of S")),
    };

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    visited.insert(start_pos);
    queue.push_back((start_pos, 0));

    let mut max_dist = 0;

    while let Some((pos, dist)) = queue.pop_front() {
        max_dist = max_dist.max(dist);
        let (curr_r, curr_c) = pos;
        let curr_char = if pos == start_pos { actual_start_char } else { grid[curr_r][curr_c] };

        let neighbors = get_neighbors(curr_r, curr_c, curr_char, rows, cols);
        for next in neighbors {
            if !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }
    }

    Ok((max_dist, visited, start_pos, actual_start_char))
}

fn get_neighbors(r: usize, c: usize, ch: char, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    match ch {
        '|' => {
            if r > 0 { res.push((r - 1, c)); }
            if r + 1 < rows { res.push((r + 1, c)); }
        }
        '-' => {
            if c > 0 { res.push((r, c - 1)); }
            if c + 1 < cols { res.push((r, c + 1)); }
        }
        'L' => {
            if r > 0 { res.push((r - 1, c)); }
            if c + 1 < cols { res.push((r, c + 1)); }
        }
        'J' => {
            if r > 0 { res.push((r - 1, c)); }
            if c > 0 { res.push((r, c - 1)); }
        }
        '7' => {
            if r + 1 < rows { res.push((r + 1, c)); }
            if c > 0 { res.push((r, c - 1)); }
        }
        'F' => {
            if r + 1 < rows { res.push((r + 1, c)); }
            if c + 1 < cols { res.push((r, c + 1)); }
        }
        _ => {}
    }
    res
}

fn count_enclosed_tiles(grid: &[Vec<char>], loop_coords: &HashSet<(usize, usize)>, start_pos: (usize, usize), actual_start_char: char) -> usize {
    let mut count = 0;
    for (r, row) in grid.iter().enumerate() {
        let mut inside = false;
        for (c, &ch) in row.iter().enumerate() {
            if loop_coords.contains(&(r, c)) {
                let actual_ch = if (r, c) == start_pos { actual_start_char } else { ch };
                // Rule: Toggle inside if the pipe has a "North" component
                if matches!(actual_ch, '|' | 'L' | 'J') {
                    inside = !inside;
                }
            } else if inside {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (dist, _, _, _) = find_loop(&grid)?;
        assert_eq!(dist, 4);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (dist, _, _, _) = find_loop(&grid)?;
        assert_eq!(dist, 8);
        Ok(())
    }

    #[test]
    fn test_part2_example_1() -> Result<()> {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (_, loop_coords, start_pos, actual_start_char) = find_loop(&grid)?;
        let count = count_enclosed_tiles(&grid, &loop_coords, start_pos, actual_start_char);
        assert_eq!(count, 4);
        Ok(())
    }

    #[test]
    fn test_part2_example_2() -> Result<()> {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (_, loop_coords, start_pos, actual_start_char) = find_loop(&grid)?;
        let count = count_enclosed_tiles(&grid, &loop_coords, start_pos, actual_start_char);
        assert_eq!(count, 8);
        Ok(())
    }

    #[test]
    fn test_part2_example_3() -> Result<()> {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let (_, loop_coords, start_pos, actual_start_char) = find_loop(&grid)?;
        let count = count_enclosed_tiles(&grid, &loop_coords, start_pos, actual_start_char);
        assert_eq!(count, 10);
        Ok(())
    }
}
