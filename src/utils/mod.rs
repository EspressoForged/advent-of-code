//! Expert-level common utilities for Advent of Code.
//!
//! This module provides standardized input loading and output formatting
//! helpers, ensuring a consistent experience across all multi-year solutions.

pub mod parser;

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

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

/// Formats the result of a puzzle into a standardized, readable string.
///
/// Returns a string displaying the day number, the answer for Part 1,
/// and the answer for Part 2 (or 0 if not applicable).
pub fn format_output(day: &str, part_one: u64, part_two: u64) -> String {
    format!("Day {}: {:>20}\t{:>20}", day, part_one, part_two)
}
