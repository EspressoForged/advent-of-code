//! Expert-level parsing utilities for Advent of Code.
//!
//! This module provides robust, `nom`-based parsing primitives and a standardized
//! `Parse` trait to ensure consistent and safe data ingestion across all
//! multi-year solutions.

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{digit1, one_of},
    combinator::{all_consuming, map_res, opt, recognize},
    sequence::pair,
    IResult, Parser,
};
use std::str::FromStr;

/// A standardized interface for parsing puzzle data structures.
pub trait Parse: Sized {
    /// Parses the input string into the implementing type.
    ///
    /// # Errors
    ///
    /// Returns a `nom::Err` if the input does not match the expected format.
    fn parse(input: &str) -> IResult<&str, Self>;
}

/// Parses the entire input string using the provided parser.
///
/// This helper ensures that the entire input is consumed, returning an error
/// if any trailing characters remain.
///
/// # Errors
///
/// Returns an `anyhow::Error` if the parser fails or if the input is not fully consumed.
pub fn parse_full<T, P>(input: &str, mut parser: P) -> Result<T>
where
    P: FnMut(&str) -> IResult<&str, T>,
{
    all_consuming(&mut parser)
        .parse(input)
        .map(|(_, res)| res)
        .map_err(|e| anyhow!("Full input parsing failed: {}", e))
}

/// Parses a non-negative integer from a string of digits into a specified type.
///
/// # Examples
///
/// ```
/// use advent_of_code::utils::parser::unsigned_number;
/// let (_, n): (&str, u32) = unsigned_number("123").unwrap();
/// assert_eq!(n, 123);
/// ```
pub fn unsigned_number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    map_res(digit1, str::parse).parse(input)
}

/// Parses a signed integer (with optional '+' or '-' prefix) into a specified type.
///
/// # Examples
///
/// ```
/// use advent_of_code::utils::parser::signed_number;
/// let (_, n): (&str, i32) = signed_number("-42").unwrap();
/// assert_eq!(n, -42);
/// ```
pub fn signed_number<T>(input: &str) -> IResult<&str, T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
{
    map_res(recognize(pair(opt(one_of("+-")), digit1)), str::parse).parse(input)
}

/// Parses lines from a string slice into a `Vec` of results.
///
/// This is the core parsing logic, decoupled from file I/O for testability.
///
/// # Errors
///
/// Returns an `anyhow::Error` if any line fails to parse.
pub fn parse_str_lines<T, P>(content: &str, mut parser: P) -> Result<Vec<T>>
where
    P: FnMut(&str) -> IResult<&str, T>,
{
    content
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(line_num, line)| {
            all_consuming(&mut parser)
                .parse(line)
                .map(|(_, record)| record)
                .map_err(|e| {
                    anyhow!(
                        "Parsing failed on line {}: '{}'. Error: {}",
                        line_num + 1,
                        line,
                        e
                    )
                })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_number() {
        let res: IResult<&str, u32> = unsigned_number("123");
        assert_eq!(res, Ok(("", 123)));

        let res: IResult<&str, usize> = unsigned_number("0");
        assert_eq!(res, Ok(("", 0)));

        let res: IResult<&str, u32> = unsigned_number("abc");
        assert!(res.is_err());
    }

    #[test]
    fn test_signed_number() {
        let res: IResult<&str, i32> = signed_number("-42");
        assert_eq!(res, Ok(("", -42)));

        let res: IResult<&str, i32> = signed_number("+42");
        assert_eq!(res, Ok(("", 42)));

        let res: IResult<&str, i32> = signed_number("42");
        assert_eq!(res, Ok(("", 42)));

        let res: IResult<&str, i32> = signed_number("abc");
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_full() {
        let mut parser = unsigned_number::<u32>;
        assert!(parse_full("123", &mut parser).is_ok());
        assert!(parse_full("123 ", &mut parser).is_err()); // Trailing space
        assert!(parse_full("123abc", &mut parser).is_err()); // Trailing garbage
    }

    #[test]
    fn test_parse_str_lines() {
        let input = "123\n\n456\n";
        let res = parse_str_lines(input, unsigned_number::<u32>);
        assert_eq!(res.unwrap(), vec![123, 456]);

        let input = "123\nabc\n456";
        let res = parse_str_lines(input, unsigned_number::<u32>);
        assert!(res.is_err());
    }

    struct Dummy {
        val: u32,
    }

    impl Parse for Dummy {
        fn parse(input: &str) -> IResult<&str, Self> {
            unsigned_number(input).map(|(i, val)| (i, Dummy { val }))
        }
    }

    #[test]
    fn test_parse_trait() {
        let res = parse_full("789", Dummy::parse).unwrap();
        assert_eq!(res.val, 789);
    }
}
