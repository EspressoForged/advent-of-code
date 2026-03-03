use crate::utils::parser::{parse_full, unsigned_number, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, multispace0, one_of, space1},
    multi::{count, many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    IResult, Parser,
};

// --- Data Structures ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Piece {
    pub id: usize,
    pub grid: [[bool; 3]; 3],
    pub area: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub width: usize,
    pub height: usize,
    /// index i corresponds to Piece ID i. Value is the count of that piece.
    pub constraints: Vec<usize>,
}

#[derive(Debug, Clone)]
pub struct PuzzleData {
    pub pieces: Vec<Piece>,
    pub regions: Vec<Region>,
}

// --- Parsing Implementations ---

impl Parse for Piece {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, id) = terminated(unsigned_number, pair(tag(":"), line_ending)).parse(input)?;

        let (input, rows) =
            count(terminated(count(one_of(".#"), 3), line_ending), 3).parse(input)?;

        let mut grid = [[false; 3]; 3];
        let mut area = 0;
        for (r, row) in rows.iter().enumerate() {
            for (c, &ch) in row.iter().enumerate() {
                if ch == '#' {
                    grid[r][c] = true;
                    area += 1;
                }
            }
        }

        Ok((input, Piece { id, grid, area }))
    }
}

impl Parse for Region {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, (width, height)) =
            separated_pair(unsigned_number, tag("x"), unsigned_number).parse(input)?;

        let (input, _) = tag(": ").parse(input)?;

        let (input, constraints) = separated_list1(space1, unsigned_number).parse(input)?;

        Ok((
            input,
            Region {
                width,
                height,
                constraints,
            },
        ))
    }
}

impl Parse for PuzzleData {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, pieces) = many1(terminated(Piece::parse, multispace0)).parse(input)?;
        let (input, regions) = separated_list1(line_ending, Region::parse).parse(input)?;
        let (input, _) = multispace0.parse(input)?;

        Ok((input, PuzzleData { pieces, regions }))
    }
}

// --- Logic ---

/// Contains the core logic for the day's puzzle.
fn calculate_solution(data: &PuzzleData) -> Result<(u64, u64)> {
    let valid_regions = data
        .regions
        .iter()
        .filter(|region| {
            let total_piece_area: usize = region
                .constraints
                .iter()
                .enumerate()
                .map(|(id, &count)| {
                    let piece_area = data.pieces.get(id).map(|p| p.area).unwrap_or(0);
                    count * piece_area
                })
                .sum();
            total_piece_area <= (region.width * region.height)
        })
        .count();

    // Part 2 is known to be 0 for this day.
    Ok((valid_regions as u64, 0))
}

pub fn solve() -> Result<(u64, u64)> {
    let input_str = read_input(Year(2025), Day(12))?;
    let data = parse_full(&input_str, PuzzleData::parse)?;
    calculate_solution(&data)
}

// --- Unit Tests ---

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2"#;

    #[test]
    fn test_day_12_solution() -> Result<()> {
        let data = parse_full(TEST_INPUT, PuzzleData::parse)?;
        let (p1, p2) = calculate_solution(&data)?;
        assert_eq!(p1, 3);
        assert_eq!(p2, 0);
        Ok(())
    }
}

