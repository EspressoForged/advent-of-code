use crate::utils::parser::{parse_full, unsigned_number, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{
    character::complete::{line_ending, multispace0, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};
use std::cmp;

/// Represents an inclusive range of ingredient IDs.
#[derive(Debug, Clone, Copy)]
struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id <= self.end
    }

    /// Returns the number of IDs in this inclusive range.
    fn len(&self) -> u64 {
        if self.end < self.start {
            0
        } else {
            self.end - self.start + 1
        }
    }
}

impl Parse for FreshRange {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_pair(
                unsigned_number,
                nom::bytes::complete::tag("-"),
                unsigned_number,
            ),
            |(start, end)| FreshRange { start, end },
        )
        .parse(input)
    }
}

/// The entire database structure.
#[derive(Debug)]
struct InventoryDb {
    ranges: Vec<FreshRange>,
    available_ids: Vec<u64>,
}

impl Parse for InventoryDb {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, ranges) = separated_list1(line_ending, FreshRange::parse).parse(input)?;

        // Consume the blank line(s)
        let (input, _) = multispace1(input)?;

        let (input, available_ids) = separated_list1(line_ending, unsigned_number).parse(input)?;

        // Handle optional trailing newline
        let (input, _) = multispace0.parse(input)?;

        Ok((
            input,
            InventoryDb {
                ranges,
                available_ids,
            },
        ))
    }
}

/// Merges overlapping or adjacent ranges.
/// e.g. [1-5, 4-8] -> [1-8]
/// e.g. [1-5, 6-8] -> [1-8] (Adjacent)
fn merge_ranges(mut ranges: Vec<FreshRange>) -> Vec<FreshRange> {
    if ranges.is_empty() {
        return Vec::new();
    }

    // 1. Sort by start value
    ranges.sort_by_key(|r| r.start);

    let mut merged = Vec::new();
    let mut current = ranges[0];

    for next in ranges.iter().skip(1) {
        // If next range starts before (or immediately after) current ends, merge them.
        if next.start <= current.end.saturating_add(1) {
            current.end = cmp::max(current.end, next.end);
        } else {
            // No overlap, push current and start a new one
            merged.push(current);
            current = *next;
        }
    }
    // Push the final range
    merged.push(current);

    merged
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(db: &InventoryDb) -> Result<(u64, u64)> {
    // --- Part 1 ---
    let fresh_count_p1 = db
        .available_ids
        .iter()
        .filter(|&&id| db.ranges.iter().any(|range| range.contains(id)))
        .count();

    // --- Part 2 ---
    let merged = merge_ranges(db.ranges.clone());
    let total_fresh_ids: u64 = merged.iter().map(|r| r.len()).sum();

    Ok((fresh_count_p1 as u64, total_fresh_ids))
}

/// Entry point for Day 05
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2025), Day(5))?;
    let db = parse_full(&content, InventoryDb::parse)?;
    calculate_solution(&db)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";

    #[test]
    fn test_day_05_solution() -> Result<()> {
        let db = parse_full(TEST_INPUT, InventoryDb::parse)?;
        let (p1, p2) = calculate_solution(&db)?;
        assert_eq!(p1, 3);
        assert_eq!(p2, 14);
        Ok(())
    }
}

