use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1},
    multi::separated_list1,
    Parser,
    IResult,
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u32,
    tunnels: Vec<String>,
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let (input, _) = tag("Valve ").parse(input)?;
    let (input, name) = alpha1.parse(input)?;
    let (input, _) = tag(" has flow rate=").parse(input)?;
    let (input, flow_rate_str) = digit1.parse(input)?;
    let flow_rate = flow_rate_str.parse::<u32>().unwrap();
    let (input, _) = alt((
        tag("; tunnels lead to valves "),
        tag("; tunnel leads to valve "),
    )).parse(input)?;
    let (input, tunnels): (&str, Vec<&str>) = separated_list1(tag(", "), alpha1).parse(input)?;

    Ok((
        input,
        Valve {
            name: name.to_string(),
            flow_rate,
            tunnels: tunnels.into_iter().map(|s| s.to_string()).collect(),
        },
    ))
}

fn parse_input(input: &str) -> Result<Vec<Valve>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (_, valve) = parse_valve(line).map_err(|e| anyhow::anyhow!("Parse error: {}", e))?;
            Ok(valve)
        })
        .collect()
}

struct Context {
    flow_rates: Vec<u32>,
    dists: Vec<Vec<u32>>,
    start_idx: usize,
    n_rel: usize,
}

fn prepare_context(valves: &[Valve]) -> Context {
    let mut valve_map = HashMap::new();
    for (i, v) in valves.iter().enumerate() {
        valve_map.insert(v.name.clone(), i);
    }

    let n = valves.len();
    let mut dists = vec![vec![u32::MAX / 2; n]; n];
    for i in 0..n {
        dists[i][i] = 0;
        for neighbor in &valves[i].tunnels {
            let j = *valve_map.get(neighbor).unwrap();
            dists[i][j] = 1;
        }
    }

    // Floyd-Warshall
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dists[i][j] > dists[i][k] + dists[k][j] {
                    dists[i][j] = dists[i][k] + dists[k][j];
                }
            }
        }
    }

    // Only keep valves with flow_rate > 0 and the starting valve "AA"
    let relevant_valves: Vec<usize> = valves
        .iter()
        .enumerate()
        .filter(|(_, v)| v.flow_rate > 0 || v.name == "AA")
        .map(|(i, _)| i)
        .collect();

    let start_node = *valve_map.get("AA").expect("Valve AA not found");
    let start_idx = relevant_valves.iter().position(|&i| i == start_node).unwrap();

    let n_rel = relevant_valves.len();
    let flow_rates: Vec<u32> = relevant_valves.iter().map(|&i| valves[i].flow_rate).collect();
    let mut rel_dists = vec![vec![0u32; n_rel]; n_rel];
    for i in 0..n_rel {
        for j in 0..n_rel {
            rel_dists[i][j] = dists[relevant_valves[i]][relevant_valves[j]];
        }
    }

    Context {
        flow_rates,
        dists: rel_dists,
        start_idx,
        n_rel,
    }
}

fn get_max_pressures_for_masks(ctx: &Context, time: u32) -> HashMap<u64, u32> {
    let mut max_pressures = HashMap::new();
    dfs_masks(
        ctx.start_idx,
        time,
        0,
        0,
        ctx,
        &mut max_pressures,
    );
    max_pressures
}

fn dfs_masks(
    curr_idx: usize,
    time_left: u32,
    curr_pressure: u32,
    opened_mask: u64,
    ctx: &Context,
    max_pressures: &mut HashMap<u64, u32>,
) {
    let entry = max_pressures.entry(opened_mask).or_insert(0);
    if curr_pressure > *entry {
        *entry = curr_pressure;
    }

    for next_idx in 0..ctx.n_rel {
        if ctx.flow_rates[next_idx] > 0 && (opened_mask & (1 << next_idx)) == 0 {
            let dist = ctx.dists[curr_idx][next_idx];
            if time_left > dist + 1 {
                let remaining = time_left - dist - 1;
                dfs_masks(
                    next_idx,
                    remaining,
                    curr_pressure + remaining * ctx.flow_rates[next_idx],
                    opened_mask | (1 << next_idx),
                    ctx,
                    max_pressures,
                );
            }
        }
    }
}

/// Solve for Year 2022, Day 16
pub fn solve() -> Result<(u64, u64)> {
    let input = crate::utils::read_input(crate::utils::Year(2022), crate::utils::Day(16))?;
    let valves = parse_input(&input)?;
    let ctx = prepare_context(&valves);

    // Part 1: 30 minutes, 1 person
    let masks_p1 = get_max_pressures_for_masks(&ctx, 30);
    let p1 = *masks_p1.values().max().unwrap_or(&0);

    // Part 2: 26 minutes, 2 people
    let masks_p2 = get_max_pressures_for_masks(&ctx, 26);
    let mut p2 = 0;
    
    // Convert to Vec for faster iteration
    let mask_vec: Vec<(u64, u32)> = masks_p2.into_iter().collect();
    
    for i in 0..mask_vec.len() {
        for j in i + 1..mask_vec.len() {
            let (m1, v1) = mask_vec[i];
            let (m2, v2) = mask_vec[j];
            if (m1 & m2) == 0 {
                if v1 + v2 > p2 {
                    p2 = v1 + v2;
                }
            }
        }
    }

    Ok((p1 as u64, p2 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_example() {
        let valves = parse_input(EXAMPLE).unwrap();
        let ctx = prepare_context(&valves);
        
        let masks_p1 = get_max_pressures_for_masks(&ctx, 30);
        let p1 = *masks_p1.values().max().unwrap_or(&0);
        assert_eq!(p1, 1651);

        let masks_p2 = get_max_pressures_for_masks(&ctx, 26);
        let mut p2 = 0;
        let mask_vec: Vec<(u64, u32)> = masks_p2.into_iter().collect();
        for i in 0..mask_vec.len() {
            for j in i + 1..mask_vec.len() {
                let (m1, v1) = mask_vec[i];
                let (m2, v2) = mask_vec[j];
                if (m1 & m2) == 0 {
                    if v1 + v2 > p2 {
                        p2 = v1 + v2;
                    }
                }
            }
        }
        assert_eq!(p2, 1707);
    }

    #[test]
    fn test_regression() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 1862);
        assert_eq!(p2, 2422);
        Ok(())
    }
}
