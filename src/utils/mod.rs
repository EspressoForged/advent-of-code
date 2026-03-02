//! Expert-level common utilities for Advent of Code.
//!
//! This module provides standardized input loading and output formatting
//! helpers, ensuring a consistent experience across all multi-year solutions.

pub mod parser;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

/// Reads the puzzle input for a specified year and day.
///
/// This helper resolves the input file path relative to the project root
/// and returns its content as a single `String`.
///
/// # Errors
///
/// Returns an `anyhow::Error` if the input file cannot be read from the
/// standard directory structure (`inputs/year_XXXX/day_XX/input.txt`).
pub fn read_input(year: u16, day: u8) -> Result<String> {
    let path = PathBuf::from("inputs")
        .join(format!("year_{}", year))
        .join(format!("day_{:02}", day))
        .join("input.txt");

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
pub fn run_day(day: u8, solve_fn: fn() -> Result<(u64, u64)>) -> Result<()> {
    let start = Instant::now();
    let (p1, p2) = solve_fn().with_context(|| format!("Failed to solve day {:02}", day))?;
    let duration = start.elapsed();

    println!(
        "Day {:02}: p1={:<15} p2={:<15} ({:?})",
        day, p1, p2, duration
    );
    Ok(())
}
