use anyhow::{anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    IResult,
    Parser,
};
use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    let (input, _) = tag("Blueprint ").parse(input)?;
    let (input, id_str) = digit1.parse(input)?;
    let id = id_str.parse::<u32>().unwrap();
    let (input, _) = tag(": Each ore robot costs ").parse(input)?;
    let (input, ore_robot_ore_cost_str) = digit1.parse(input)?;
    let ore_robot_ore_cost = ore_robot_ore_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" ore. Each clay robot costs ").parse(input)?;
    let (input, clay_robot_ore_cost_str) = digit1.parse(input)?;
    let clay_robot_ore_cost = clay_robot_ore_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" ore. Each obsidian robot costs ").parse(input)?;
    let (input, obsidian_robot_ore_cost_str) = digit1.parse(input)?;
    let obsidian_robot_ore_cost = obsidian_robot_ore_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" ore and ").parse(input)?;
    let (input, obsidian_robot_clay_cost_str) = digit1.parse(input)?;
    let obsidian_robot_clay_cost = obsidian_robot_clay_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" clay. Each geode robot costs ").parse(input)?;
    let (input, geode_robot_ore_cost_str) = digit1.parse(input)?;
    let geode_robot_ore_cost = geode_robot_ore_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" ore and ").parse(input)?;
    let (input, geode_robot_obsidian_cost_str) = digit1.parse(input)?;
    let geode_robot_obsidian_cost = geode_robot_obsidian_cost_str.parse::<u32>().unwrap();
    let (input, _) = tag(" obsidian.").parse(input)?;

    Ok((
        input,
        Blueprint {
            id,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        },
    ))
}

#[derive(Debug, Clone, Copy)]
struct State {
    time_left: u32,
    ore: u32,
    clay: u32,
    obsidian: u32,
    geodes: u32,
    ore_robots: u32,
    clay_robots: u32,
    obsidian_robots: u32,
    geode_robots: u32,
}

fn solve_blueprint(blueprint: &Blueprint, time: u32) -> u32 {
    let mut max_geodes = 0;
    let max_ore_cost = [
        blueprint.ore_robot_ore_cost,
        blueprint.clay_robot_ore_cost,
        blueprint.obsidian_robot_ore_cost,
        blueprint.geode_robot_ore_cost,
    ]
    .iter()
    .copied()
    .max()
    .unwrap();

    let initial_state = State {
        time_left: time,
        ore: 0,
        clay: 0,
        obsidian: 0,
        geodes: 0,
        ore_robots: 1,
        clay_robots: 0,
        obsidian_robots: 0,
        geode_robots: 0,
    };

    dfs(initial_state, blueprint, max_ore_cost, &mut max_geodes);
    max_geodes
}

fn dfs(state: State, b: &Blueprint, max_ore: u32, max_geodes: &mut u32) {
    // Heuristic pruning: if even building a geode robot every minute won't beat best, stop.
    let potential = state.geodes + state.geode_robots * state.time_left + state.time_left * (state.time_left.saturating_sub(1)) / 2;
    if potential <= *max_geodes {
        return;
    }

    // Update global maximum
    let end_geodes = state.geodes + state.geode_robots * state.time_left;
    if end_geodes > *max_geodes {
        *max_geodes = end_geodes;
    }

    if state.time_left == 0 {
        return;
    }

    // Try building geode robot
    if let Some(t) = time_to_build(b.geode_robot_ore_cost, 0, b.geode_robot_obsidian_cost, &state) {
        if state.time_left > t {
            let mut next = state;
            next.time_left -= t;
            next.ore = next.ore + next.ore_robots * t - b.geode_robot_ore_cost;
            next.clay += next.clay_robots * t;
            next.obsidian = next.obsidian + next.obsidian_robots * t - b.geode_robot_obsidian_cost;
            next.geodes += next.geode_robots * t;
            next.geode_robots += 1;
            dfs(next, b, max_ore, max_geodes);
        }
    }

    // Try building obsidian robot
    if state.obsidian_robots < b.geode_robot_obsidian_cost {
        if let Some(t) = time_to_build(b.obsidian_robot_ore_cost, b.obsidian_robot_clay_cost, 0, &state) {
            if state.time_left > t {
                let mut next = state;
                next.time_left -= t;
                next.ore = next.ore + next.ore_robots * t - b.obsidian_robot_ore_cost;
                next.clay = next.clay + next.clay_robots * t - b.obsidian_robot_clay_cost;
                next.obsidian += next.obsidian_robots * t;
                next.geodes += next.geode_robots * t;
                next.obsidian_robots += 1;
                dfs(next, b, max_ore, max_geodes);
            }
        }
    }

    // Try building clay robot
    if state.clay_robots < b.obsidian_robot_clay_cost {
        if let Some(t) = time_to_build(b.clay_robot_ore_cost, 0, 0, &state) {
            if state.time_left > t {
                let mut next = state;
                next.time_left -= t;
                next.ore = next.ore + next.ore_robots * t - b.clay_robot_ore_cost;
                next.clay += next.clay_robots * t;
                next.obsidian += next.obsidian_robots * t;
                next.geodes += next.geode_robots * t;
                next.clay_robots += 1;
                dfs(next, b, max_ore, max_geodes);
            }
        }
    }

    // Try building ore robot
    if state.ore_robots < max_ore {
        if let Some(t) = time_to_build(b.ore_robot_ore_cost, 0, 0, &state) {
            if state.time_left > t {
                let mut next = state;
                next.time_left -= t;
                next.ore = next.ore + next.ore_robots * t - b.ore_robot_ore_cost;
                next.clay += next.clay_robots * t;
                next.obsidian += next.obsidian_robots * t;
                next.geodes += next.geode_robots * t;
                next.ore_robots += 1;
                dfs(next, b, max_ore, max_geodes);
            }
        }
    }
}

fn time_to_build(ore_cost: u32, clay_cost: u32, obsidian_cost: u32, state: &State) -> Option<u32> {
    let mut t = 0;

    // Ore requirement
    if ore_cost > state.ore {
        if state.ore_robots == 0 { return None; }
        t = t.max((ore_cost - state.ore).div_ceil(state.ore_robots));
    }

    // Clay requirement
    if clay_cost > state.clay {
        if state.clay_robots == 0 { return None; }
        t = t.max((clay_cost - state.clay).div_ceil(state.clay_robots));
    }

    // Obsidian requirement
    if obsidian_cost > state.obsidian {
        if state.obsidian_robots == 0 { return None; }
        t = t.max((obsidian_cost - state.obsidian).div_ceil(state.obsidian_robots));
    }

    Some(t + 1)
}

pub fn solve() -> Result<(u64, u64)> {
    let content = crate::utils::read_input(crate::utils::Year(2022), crate::utils::Day(19))?;
    let blueprints: Vec<Blueprint> = content
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| parse_blueprint(l).map(|(_, b)| b).map_err(|e| anyhow!("Parse error: {}", e)))
        .collect::<Result<Vec<Blueprint>>>()?;

    // Part 1: All blueprints, 24 minutes
    let p1: u32 = blueprints
        .par_iter()
        .map(|b| b.id * solve_blueprint(b, 24))
        .sum();

    // Part 2: First 3 blueprints, 32 minutes
    let p2: u32 = blueprints
        .iter()
        .take(3)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|b| solve_blueprint(b, 32))
        .product();

    Ok((p1 as u64, p2 as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
";

    #[test]
    fn test_example_p1() {
        let blueprints: Vec<Blueprint> = EXAMPLE
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| parse_blueprint(l).map(|(_, b)| b).unwrap())
            .collect();
        
        let g1 = solve_blueprint(&blueprints[0], 24);
        assert_eq!(g1, 9);
        
        let g2 = solve_blueprint(&blueprints[1], 24);
        assert_eq!(g2, 12);
    }

    #[test]
    fn test_example_p2() {
        let blueprints: Vec<Blueprint> = EXAMPLE
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| parse_blueprint(l).map(|(_, b)| b).unwrap())
            .collect();
        
        let g1 = solve_blueprint(&blueprints[0], 32);
        assert_eq!(g1, 56);
        
        let g2 = solve_blueprint(&blueprints[1], 32);
        assert_eq!(g2, 62);
    }

    #[test]
    fn test_regression() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 1466);
        assert_eq!(p2, 8250);
        Ok(())
    }
}
