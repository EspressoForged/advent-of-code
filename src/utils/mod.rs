//! Expert-level common utilities for Advent of Code.
//!
//! This module provides standardized input loading and output formatting
//! helpers, ensuring a consistent experience across all multi-year solutions.

pub mod parser;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/// Type alias for a puzzle solver function.
pub type SolveFn = fn() -> Result<(u64, u64)>;

/// Macro to register days in a year's mod.rs.
/// It declares the modules and implements a `get_solver` function.
#[macro_export]
macro_rules! register_days {
    ($($day:ident),* $(,)?) => {
        $(
            pub mod $day;
        )*

        pub fn get_solver(day: u8) -> Option<$crate::utils::SolveFn> {
            match day {
                $(
                    d if d == $crate::utils::day_to_u8(stringify!($day)) => Some($day::solution::solve),
                )*
                _ => None,
            }
        }
    };
}

/// Helper function for the `register_days!` macro.
pub const fn day_to_u8(day_str: &str) -> u8 {
    // Expects "day_XX"
    let bytes = day_str.as_bytes();
    if bytes.len() != 6 {
        return 0;
    }
    let ten = bytes[4] - b'0';
    let one = bytes[5] - b'0';
    ten * 10 + one
}

/// Reads the puzzle input for a specified year and day.
///
/// This helper resolves the input file path relative to the project root
/// and returns its content as a single `String`.
///
/// # Errors
///
/// Returns an `anyhow::Error` if the input file cannot be read from the
/// Standardized directory structure (`inputs/year_XXXX/day_XX.txt`).
pub fn read_input(year: u16, day: u8) -> Result<String> {
    let path = PathBuf::from("inputs")
        .join(format!("year_{}", year))
        .join(format!("day_{:02}.txt", day));

    fs::read_to_string(&path).with_context(|| {
        format!(
            "Failed to read input file for Year {}, Day {:02} at: {:?}",
            year, day, path
        )
    })
}

/// Standardized runner for a puzzle.
///
/// It executes the provided `solve_fn`, tracks its execution time,
/// and prints the formatted results to stdout.
pub fn run_day(year: u16, day: u8, solve_fn: SolveFn) -> Result<()> {
    let start = Instant::now();
    let (p1, p2) =
        solve_fn().with_context(|| format!("Failed to solve Year {}, Day {:02}", year, day))?;
    let duration = start.elapsed();

    if p2 == 0 {
        println!("[{}] Day {:02}: p1={:<15} ({:?})", year, day, p1, duration);
    } else {
        println!(
            "[{}] Day {:02}: p1={:<15} p2={:<15} ({:?})",
            year, day, p1, p2, duration
        );
    }
    Ok(())
}
