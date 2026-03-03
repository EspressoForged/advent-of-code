use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Directions for the robot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turn(self, turn_val: i64) -> Self {
        let current = self as i32;
        let next = if turn_val == 0 {
            // Left 90 degrees
            (current + 3) % 4
        } else {
            // Right 90 degrees
            (current + 1) % 4
        };
        match next {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            3 => Direction::Left,
            _ => unreachable!(),
        }
    }

    fn delta(self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

/// Solves Year 2019, Day 11: Space Police.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(11))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    // Part 1: Start on a black panel (0)
    let painted_p1 = run_robot(&program, 0)?;
    let part1 = painted_p1.len() as u64;

    // Part 2: Start on a white panel (1)
    let painted_p2 = run_robot(&program, 1)?;
    render_panels(&painted_p2);

    // Part 2 answer is a string, so we return 0 and expect the user to see the output.
    Ok((part1, 0))
}

fn run_robot(program: &[i64], start_color: i64) -> Result<HashMap<(i32, i32), i64>> {
    let mut vm = Intcode::new(program.to_vec());
    let mut panels = HashMap::new();
    let mut pos = (0, 0);
    let mut dir = Direction::Up;

    if start_color == 1 {
        panels.insert(pos, 1);
    }

    loop {
        let current_color = *panels.get(&pos).unwrap_or(&0);
        vm.add_input(current_color);

        // First output: color to paint
        let color = match vm.run()? {
            Status::Output(c) => c,
            Status::Halted => break,
            Status::NeedsInput => return Err(anyhow::anyhow!("VM requested input unexpectedly")),
        };

        // Second output: direction to turn
        let turn = match vm.run()? {
            Status::Output(t) => t,
            Status::Halted => return Err(anyhow::anyhow!("VM halted unexpectedly after first output")),
            Status::NeedsInput => return Err(anyhow::anyhow!("VM requested input unexpectedly")),
        };

        panels.insert(pos, color);
        dir = dir.turn(turn);
        let (dx, dy) = dir.delta();
        pos = (pos.0 + dx, pos.1 + dy);
    }

    Ok(panels)
}

fn render_panels(panels: &HashMap<(i32, i32), i64>) {
    if panels.is_empty() {
        return;
    }

    let min_x = panels.keys().map(|p| p.0).min().unwrap_or(0);
    let max_x = panels.keys().map(|p| p.0).max().unwrap_or(0);
    let min_y = panels.keys().map(|p| p.1).min().unwrap_or(0);
    let max_y = panels.keys().map(|p| p.1).max().unwrap_or(0);

    println!("\nRegistration Identifier:");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let color = *panels.get(&(x, y)).unwrap_or(&0);
            if color == 1 {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
