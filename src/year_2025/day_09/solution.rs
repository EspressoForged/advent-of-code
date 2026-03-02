use advent_of_code::utils::parser::Parse;
use advent_of_code::utils::{format_output, read_input};
use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag, character::complete::i64 as parse_i64, sequence::separated_pair, IResult,
    Parser,
};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Edge {
    Horizontal { y: i64, x1: i64, x2: i64 },
    Vertical { x: i64, y1: i64, y2: i64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Parse for Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        separated_pair(parse_i64, tag(","), parse_i64)
            .map(|(x, y)| Point { x, y })
            .parse(input)
    }
}

/// Checks if a point (px, py) is inside or on the boundary of the rectilinear polygon.
fn is_in_or_on(px: f64, py: f64, edges: &[Edge]) -> bool {
    let mut on_edge = false;
    let mut count = 0;
    for &edge in edges {
        match edge {
            Edge::Horizontal { y, x1, x2 } => {
                let x_min = x1.min(x2);
                let x_max = x1.max(x2);
                if (py - y as f64).abs() < 1e-9 && px >= x_min as f64 && px <= x_max as f64 {
                    on_edge = true;
                    break;
                }
                if y as f64 > py && px >= x_min as f64 && px < x_max as f64 {
                    count += 1;
                }
            }
            Edge::Vertical { x, y1, y2 } => {
                let y_min = y1.min(y2);
                let y_max = y1.max(y2);
                if (px - x as f64).abs() < 1e-9 && py >= y_min as f64 && py <= y_max as f64 {
                    on_edge = true;
                    break;
                }
            }
        }
    }
    on_edge || (count % 2 != 0)
}

/// Checks if any polygon edge intersects the strict interior of the rectangle.
fn intersects_interior(xmin: i64, xmax: i64, ymin: i64, ymax: i64, edges: &[Edge]) -> bool {
    for &edge in edges {
        match edge {
            Edge::Vertical { x, y1, y2 } => {
                let ey_min = y1.min(y2);
                let ey_max = y1.max(y2);
                if x > xmin && x < xmax && ey_max > ymin && ey_min < ymax {
                    return true;
                }
            }
            Edge::Horizontal { y, x1, x2 } => {
                let ex_min = x1.min(x2);
                let ex_max = x1.max(x2);
                if y > ymin && y < ymax && ex_max > xmin && ex_min < xmax {
                    return true;
                }
            }
        }
    }
    false
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(points: &[Point]) -> Result<(u64, u64)> {
    let n = points.len();

    // Part 1: Largest rectangle between any two red tiles (no containment constraint)
    let part1 = (0..n)
        .into_par_iter()
        .map(|i| {
            let mut max_local = 0;
            for j in i..n {
                let p1 = points[i];
                let p2 = points[j];
                let area = (p1.x - p2.x).abs() + 1;
                let height = (p1.y - p2.y).abs() + 1;
                max_local = max_local.max(area * height);
            }
            max_local
        })
        .max()
        .unwrap_or(0);

    // Build the edges of the rectilinear polygon for Part 2
    let mut edges = Vec::with_capacity(n);
    for i in 0..n {
        let p1 = points[i];
        let p2 = points[(i + 1) % n];
        if p1.x == p2.x {
            edges.push(Edge::Vertical {
                x: p1.x,
                y1: p1.y,
                y2: p2.y,
            });
        } else {
            edges.push(Edge::Horizontal {
                y: p1.y,
                x1: p1.x,
                x2: p2.x,
            });
        }
    }

    // Part 2: Largest rectangle fully contained within the red/green tile polygon
    let part2 = (0..n)
        .into_par_iter()
        .map(|i| {
            let mut max_local = 0;
            for j in i..n {
                let p1 = points[i];
                let p2 = points[j];

                let xmin = p1.x.min(p2.x);
                let xmax = p1.x.max(p2.x);
                let ymin = p1.y.min(p2.y);
                let ymax = p1.y.max(p2.y);

                let area = (xmax - xmin + 1) * (ymax - ymin + 1);
                if area <= max_local {
                    continue;
                }

                if !is_in_or_on(xmin as f64, ymax as f64, &edges) {
                    continue;
                }
                if !is_in_or_on(xmax as f64, ymin as f64, &edges) {
                    continue;
                }

                let mid_x = (xmin as f64 + xmax as f64) / 2.0;
                let mid_y = (ymin as f64 + ymax as f64) / 2.0;
                if !is_in_or_on(mid_x, ymin as f64, &edges) {
                    continue;
                }
                if !is_in_or_on(mid_x, ymax as f64, &edges) {
                    continue;
                }
                if !is_in_or_on(xmin as f64, mid_y, &edges) {
                    continue;
                }
                if !is_in_or_on(xmax as f64, mid_y, &edges) {
                    continue;
                }

                if !intersects_interior(xmin, xmax, ymin, ymax, &edges) {
                    max_local = area;
                }
            }
            max_local
        })
        .max()
        .unwrap_or(0);

    Ok((part1 as u64, part2 as u64))
}

pub fn solve() -> Result<()> {
    let content = read_input(2025, 9)?;
    // Input is space-separated points, not line-separated.
    let points: Vec<Point> = content
        .split_whitespace()
        .map(|s| {
            Point::parse(s)
                .map(|(_, p)| p)
                .map_err(|e| anyhow!("Failed to parse point {}: {}", s, e))
        })
        .collect::<Result<Vec<Point>>>()?;

    let (p1, p2) = calculate_solution(&points)?;
    println!("{}", format_output("09", p1, p2));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "7,1 11,1 11,7 9,7 9,5 2,5 2,3 7,3";

    #[test]
    fn test_day_09_solution() -> Result<()> {
        let points: Vec<Point> = TEST_INPUT
            .split_whitespace()
            .map(|s| {
                Point::parse(s)
                    .map(|(_, p)| p)
                    .map_err(|e| anyhow!("Failed to parse point {}: {}", s, e))
            })
            .collect::<Result<Vec<Point>>>()?;

        let (p1, p2) = calculate_solution(&points)?;
        assert_eq!(p1, 50);
        assert_eq!(p2, 24);
        Ok(())
    }
}
