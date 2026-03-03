//! Shared utilities for the Advent of Code runner.
//!
//! This module provides the central types and functions for running puzzles,
//! reading input files, and managing year/day registration.

use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

pub mod parser;

/// A wrapper around a year value to avoid primitive obsession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(pub u16);

/// A wrapper around a day value to avoid primitive obsession.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Day(pub u8);

/// Signature for a puzzle solver function.
///
/// Every solver returns a result containing a tuple of (Part 1, Part 2) answers as `u64`.
pub type SolveFn = fn() -> Result<(u64, u64)>;

/// Macro to register multiple days within a year module.
///
/// This macro automates the creation of module declarations and the `get_solver`
/// function for a specific year.
#[macro_export]
macro_rules! register_days {
    ($($day:ident),*) => {
        $(
            pub mod $day;
        )*

        /// Resolves a solver function for a specific day.
        pub(crate) fn get_solver(day: u8) -> Option<$crate::utils::SolveFn> {
            $(
                if day == $crate::utils::day_to_u8(stringify!($day)) {
                    return Some($day::solve);
                }
            )*
            None
        }
    };
}

/// Converts a "day_XX" string literal to its numeric value.
///
/// # Examples
/// ```
/// use advent_of_code::utils::day_to_u8;
/// assert_eq!(day_to_u8("day_01"), 1);
/// assert_eq!(day_to_u8("day_25"), 25);
/// ```
#[must_use]
pub const fn day_to_u8(day_str: &str) -> u8 {
    let bytes = day_str.as_bytes();
    if bytes.len() < 6 {
        return 0;
    }
    let ten = bytes[4] - b'0';
    let one = bytes[5] - b'0';
    ten * 10 + one
}

/// Reads the puzzle input for a specified year and day.
///
/// The input is expected to be at `inputs/year_XXXX/day_XX.txt`.
///
/// # Errors
/// Returns an `anyhow::Error` if the input file cannot be found or read.
pub fn read_input(year: Year, day: Day) -> Result<String> {
    let path = PathBuf::from("inputs")
        .join(format!("year_{}", year.0))
        .join(format!("day_{:02}.txt", day.0));

    fs::read_to_string(&path).with_context(|| {
        format!(
            "Failed to read input file for Year {}, Day {:02} at: {:?}",
            year.0, day.0, path
        )
    })
}

/// Standardized runner for a puzzle.
///
/// Executes the provided `solve_fn`, tracks its execution time, and prints the results.
///
/// # Errors
/// Returns an error if the puzzle solver function itself returns an `Err`.
pub fn run_day(year: Year, day: Day, solve_fn: SolveFn) -> Result<()> {
    let start = Instant::now();
    let (p1, p2) = solve_fn()?;
    let duration = start.elapsed();

    println!(
        "[{}] Day {:02}: p1={p1:<15} p2={p2:<15} ({:?})",
        year.0, day.0, duration
    );

    Ok(())
}
