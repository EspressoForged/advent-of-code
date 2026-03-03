use crate::utils::parser::{parse_str_lines, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, space0, space1},
    multi::separated_list1,
    IResult, Parser,
};
use std::collections::HashMap;

/// Type alias for the graph adjacency list.
type Graph = HashMap<String, Vec<String>>;

/// Represents a source node and its list of destination nodes.
#[derive(Debug, Clone)]
struct Connection {
    src: String,
    dests: Vec<String>,
}

impl Parse for Connection {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, src) = alpha1(input)?;
        let (input, _) = tag(":")(input)?;
        let (input, _) = space0(input)?;
        let (input, dests) = separated_list1(space1, alpha1).parse(input)?;

        Ok((
            input,
            Connection {
                src: src.to_string(),
                dests: dests.iter().map(|s| s.to_string()).collect::<Vec<String>>(),
            },
        ))
    }
}

/// Recursive helper to count paths from `current` to `target`.
fn count_paths_recursive(
    current: &str,
    target: &str,
    graph: &Graph,
    memo: &mut HashMap<String, u64>,
) -> u64 {
    // Base Case: Reached the target
    if current == target {
        return 1;
    }

    // Check Memoization
    if let Some(&count) = memo.get(current) {
        return count;
    }

    let mut total_paths = 0;

    // Iterate neighbors
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths_recursive(neighbor, target, graph, memo);
        }
    }

    // Cache and return
    memo.insert(current.to_string(), total_paths);
    total_paths
}

/// Wrapper to initialize memoization and count paths between two specific nodes.
fn count_paths(start: &str, target: &str, graph: &Graph) -> u64 {
    // We create a fresh memoization cache for each distinct query (start->target pair)
    let mut memo = HashMap::new();
    count_paths_recursive(start, target, graph, &mut memo)
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(connections: &[Connection]) -> Result<(u64, u64)> {
    let mut graph: Graph = HashMap::new();
    for conn in connections {
        graph.insert(conn.src.clone(), conn.dests.clone());
    }

    // --- Part 1: "you" -> "out" ---
    let p1_total = count_paths("you", "out", &graph);

    // --- Part 2: "svr" -> "out" visiting "dac" and "fft" ---
    let svr_to_dac = count_paths("svr", "dac", &graph);
    let dac_to_fft = count_paths("dac", "fft", &graph);
    let fft_to_out = count_paths("fft", "out", &graph);
    let paths_sequence_a = svr_to_dac * dac_to_fft * fft_to_out;

    let svr_to_fft = count_paths("svr", "fft", &graph);
    let fft_to_dac = count_paths("fft", "dac", &graph);
    let dac_to_out = count_paths("dac", "out", &graph);
    let paths_sequence_b = svr_to_fft * fft_to_dac * dac_to_out;

    let p2_total = paths_sequence_a + paths_sequence_b;

    Ok((p1_total, p2_total))
}

/// Entry point for Day 11
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2025), Day(11))?;
    let connections = parse_str_lines(&content, Connection::parse)?;
    calculate_solution(&connections)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "svr: aaa bbb\naaa: fft\nfft: ccc\nbbb: tty\ntty: ccc\nccc: ddd eee\nddd: hub\nhub: fff\neee: dac\ndac: fff\nfff: ggg hhh\nggg: out\nhhh: out";

    #[test]
    fn test_day_11_solution() -> Result<()> {
        let connections = parse_str_lines(TEST_INPUT, Connection::parse)?;
        let (p1, p2) = calculate_solution(&connections)?;
        assert_eq!(p1, 0);
        assert_eq!(p2, 2);
        Ok(())
    }
}

