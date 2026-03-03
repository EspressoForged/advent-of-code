//! Parsing utilities for Advent of Code puzzle inputs.
//!
//! This module provides a set of common parsers based on the `nom` crate,
//! facilitating consistent and type-safe data extraction from puzzle strings.

use anyhow::{anyhow, Result};
use nom::{
    character::complete::{char, digit1},
    combinator::{map_res, opt, recognize},
    sequence::pair,
    IResult, Parser,
};

/// Trait for types that can be parsed from a string using a `nom` parser.
pub trait Parse: Sized {
    /// Parses the input string into an instance of the type.
    ///
    /// # Errors
    /// Returns a `nom::Err` if the parsing logic fails.
    fn parse(input: &str) -> IResult<&str, Self>;
}

/// Parses the full input string using the provided parser and ensures no trailing data.
///
/// # Errors
/// Returns an error if the parser fails or if there is non-whitespace trailing data.
///
/// # Examples
/// ```
/// use advent_of_code::utils::parser::{parse_full, unsigned_number};
/// let result: u32 = parse_full("123", unsigned_number).unwrap();
/// assert_eq!(result, 123);
/// ```
#[allow(dead_code)]
pub fn parse_full<T, P>(input: &str, mut parser: P) -> Result<T>
where
    P: FnMut(&str) -> IResult<&str, T>,
{
    let (rest, result) = parser(input).map_err(|e| anyhow!("Parsing error: {}", e))?;
    if !rest.trim().is_empty() {
        return Err(anyhow!("Trailing input after parsing: '{}'", rest));
    }
    Ok(result)
}

/// Parses an unsigned number of type T.
///
/// # Examples
/// ```
/// use advent_of_code::utils::parser::unsigned_number;
/// let (_, n): (&str, u32) = unsigned_number("123").unwrap();
/// assert_eq!(n, 123);
/// ```
pub fn unsigned_number<T>(input: &str) -> IResult<&str, T>
where
    T: std::str::FromStr,
{
    map_res(digit1, |s: &str| s.parse::<T>()).parse(input)
}

/// Parses a signed number of type T (including optional '-' sign).
///
/// # Examples
/// ```
/// use advent_of_code::utils::parser::signed_number;
/// let (_, n): (&str, i32) = signed_number("-42").unwrap();
/// assert_eq!(n, -42);
/// ```
#[allow(dead_code)]
pub fn signed_number<T>(input: &str) -> IResult<&str, T>
where
    T: std::str::FromStr,
{
    map_res(recognize(pair(opt(char('-')), digit1)), |s: &str| {
        s.parse::<T>()
    })
    .parse(input)
}

/// Parses multiple lines of text into a vector of items using the provided parser.
///
/// This function filters out empty lines automatically.
///
/// # Errors
/// Returns an error if any non-empty line fails to be fully consumed by the parser.
pub fn parse_str_lines<T, P>(content: &str, mut parser: P) -> Result<Vec<T>>
where
    P: FnMut(&str) -> IResult<&str, T>,
{
    content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, item) = parser(line).map_err(|e| anyhow!("Line parsing error: {}", e))?;
            Ok(item)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unsigned_number() {
        let (_, n): (&str, u32) = unsigned_number("123").unwrap();
        assert_eq!(n, 123);
    }

    #[test]
    fn test_signed_number() {
        let (_, n): (&str, i32) = signed_number("-42").unwrap();
        assert_eq!(n, -42);
    }

    struct Dummy {
        val: u32,
    }
    impl Dummy {
        fn parse(input: &str) -> IResult<&str, Self> {
            let (rest, val) = unsigned_number(input)?;
            Ok((rest, Dummy { val }))
        }
    }

    impl Parse for Dummy {
        fn parse(input: &str) -> IResult<&str, Self> {
            Self::parse(input)
        }
    }

    #[test]
    fn test_parse_trait() {
        let (_, d) = Dummy::parse("123").unwrap();
        assert_eq!(d.val, 123);
    }

    #[test]
    fn test_parse_full() {
        let res = parse_full("123, 456", |i| {
            nom::multi::separated_list1(nom::bytes::complete::tag(", "), unsigned_number::<u32>)
                .parse(i)
        });
        assert_eq!(res.unwrap(), vec![123, 456]);

        let res = parse_full("789", Dummy::parse).unwrap();
        assert_eq!(res.val, 789);
    }

    #[test]
    fn test_parse_str_lines() {
        let input = "123\n456\n789";
        let res = parse_str_lines(input, unsigned_number::<u32>).unwrap();
        assert_eq!(res, vec![123, 456, 789]);
    }
}
