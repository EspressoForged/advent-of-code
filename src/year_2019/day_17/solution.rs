use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::Intcode;
use anyhow::{Context, Result};
use std::collections::HashSet;

/// Solves Year 2019, Day 17: Set and Forget.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(17))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let mut vm = Intcode::new(program.clone());
    let outputs = vm.run_to_end()?;

    let grid_str: String = outputs.iter().map(|&x| x as u8 as char).collect();
    let grid: Vec<Vec<char>> = grid_str
        .trim()
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let part1 = calculate_alignment_parameters(&grid);
    let part2 = solve_part2(&program, &grid)?;

    Ok((part1, part2))
}

fn calculate_alignment_parameters(grid: &[Vec<char>]) -> u64 {
    let mut sum = 0;
    let height = grid.len();
    if height == 0 {
        return 0;
    }
    let width = grid[0].len();

    let scaffold_chars: HashSet<char> = ['#', '^', 'v', '<', '>'].iter().cloned().collect();

    for y in 1..height.saturating_sub(1) {
        for x in 1..width.saturating_sub(1) {
            if scaffold_chars.contains(&grid[y][x])
                && scaffold_chars.contains(&grid[y - 1][x])
                && scaffold_chars.contains(&grid[y + 1][x])
                && scaffold_chars.contains(&grid[y][x - 1])
                && scaffold_chars.contains(&grid[y][x + 1])
            {
                sum += (y * x) as u64;
            }
        }
    }

    sum
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vector(self) -> (i32, i32) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn solve_part2(program: &[i64], grid: &[Vec<char>]) -> Result<u64> {
    let mut robot_pos = (0, 0);
    let mut robot_dir = Direction::Up;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            match c {
                '^' => {
                    robot_pos = (x as i32, y as i32);
                    robot_dir = Direction::Up;
                }
                'v' => {
                    robot_pos = (x as i32, y as i32);
                    robot_dir = Direction::Down;
                }
                '<' => {
                    robot_pos = (x as i32, y as i32);
                    robot_dir = Direction::Left;
                }
                '>' => {
                    robot_pos = (x as i32, y as i32);
                    robot_dir = Direction::Right;
                }
                _ => {}
            }
        }
    }

    let mut path = Vec::new();
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    loop {
        // 1. Move forward as far as possible
        let mut steps = 0;
        let (dx, dy) = robot_dir.to_vector();
        while robot_pos.0 + dx >= 0
            && robot_pos.0 + dx < width
            && robot_pos.1 + dy >= 0
            && robot_pos.1 + dy < height
            && grid[(robot_pos.1 + dy) as usize][(robot_pos.0 + dx) as usize] == '#'
        {
            robot_pos.0 += dx;
            robot_pos.1 += dy;
            steps += 1;
        }
        if steps > 0 {
            path.push(steps.to_string());
        }

        // 2. Try to turn
        let left_dir = robot_dir.turn_left();
        let (ldx, ldy) = left_dir.to_vector();
        if robot_pos.0 + ldx >= 0
            && robot_pos.0 + ldx < width
            && robot_pos.1 + ldy >= 0
            && robot_pos.1 + ldy < height
            && grid[(robot_pos.1 + ldy) as usize][(robot_pos.0 + ldx) as usize] == '#'
        {
            robot_dir = left_dir;
            path.push("L".to_string());
        } else {
            let right_dir = robot_dir.turn_right();
            let (rdx, rdy) = right_dir.to_vector();
            if robot_pos.0 + rdx >= 0
                && robot_pos.0 + rdx < width
                && robot_pos.1 + rdy >= 0
                && robot_pos.1 + rdy < height
                && grid[(robot_pos.1 + rdy) as usize][(robot_pos.0 + rdx) as usize] == '#'
            {
                robot_dir = right_dir;
                path.push("R".to_string());
            } else {
                break; // No more moves or turns possible
            }
        }
    }

    let compressed = compress_path(&path).context("Failed to compress path")?;

    let mut mem = program.to_vec();
    mem[0] = 2;
    let mut vm = Intcode::new(mem);

    let input_str = format!(
        "{}\n{}\n{}\n{}\nn\n",
        compressed.main, compressed.a, compressed.b, compressed.c
    );

    let mut input_iter = input_str.chars();
    let mut last_output = 0;

    loop {
        match vm.step()? {
            crate::year_2019::intcode::Status::Halted => break,
            crate::year_2019::intcode::Status::Output(val) => {
                last_output = val;
            }
            crate::year_2019::intcode::Status::NeedsInput => {
                if let Some(c) = input_iter.next() {
                    vm.add_input(c as i64);
                } else {
                    return Err(anyhow::anyhow!("VM needs input but we have none left"));
                }
            }
        }
    }

    Ok(last_output as u64)
}

struct Compressed {
    main: String,
    a: String,
    b: String,
    c: String,
}

fn compress_path(path: &[String]) -> Option<Compressed> {
    fn find_functions(path: &[String], functions: &mut Vec<Vec<String>>) -> Option<Vec<usize>> {
        if path.is_empty() {
            return Some(Vec::new());
        }

        // Try using existing functions
        for i in 0..functions.len() {
            let func = &functions[i];
            if path.starts_with(func) {
                if let Some(mut rest) = find_functions(&path[func.len()..], functions) {
                    rest.insert(0, i);
                    return Some(rest);
                }
            }
        }

        // Try creating a new function if we have space
        if functions.len() < 3 {
            for len in (1..=path.len()).rev() {
                let candidate = &path[..len];
                let candidate_str = candidate.join(",");
                if candidate_str.len() > 20 {
                    continue;
                }

                let current_index = functions.len();
                functions.push(candidate.to_vec());
                if let Some(mut rest) = find_functions(&path[len..], functions) {
                    rest.insert(0, current_index);
                    return Some(rest);
                }
                functions.pop();
            }
        }

        None
    }

    let mut functions = Vec::new();
    if let Some(main_indices) = find_functions(path, &mut functions) {
        let main = main_indices
            .iter()
            .map(|&i| match i {
                0 => "A",
                1 => "B",
                2 => "C",
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join(",");

        if main.len() <= 20 {
            return Some(Compressed {
                main,
                a: functions.first().map(|f| f.join(",")).unwrap_or_default(),
                b: functions.get(1).map(|f| f.join(",")).unwrap_or_default(),
                c: functions.get(2).map(|f| f.join(",")).unwrap_or_default(),
            });
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let example = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";
        let grid: Vec<Vec<char>> = example.lines().map(|l| l.chars().collect()).collect();
        assert_eq!(calculate_alignment_parameters(&grid), 76);
    }
}
