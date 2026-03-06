use crate::utils::parser::unsigned_number;
use crate::utils::{read_input, Day, Year};
use anyhow::{anyhow, Context, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace0},
    multi::separated_list1,
    sequence::{separated_pair, terminated},
    IResult, Parser,
};
use std::collections::{HashMap, VecDeque};

/// A chemical with a specific quantity.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ChemicalQty {
    amount: u64,
    name: String,
}

/// A chemical reaction.
#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<ChemicalQty>,
    output: ChemicalQty,
}

/// Parses a single chemical and its quantity (e.g., "10 ORE").
fn parse_chemical_qty(input: &str) -> IResult<&str, ChemicalQty> {
    separated_pair(unsigned_number, tag(" "), alpha1)
        .map(|(amount, name): (u64, &str)| ChemicalQty {
            amount,
            name: name.to_string(),
        })
        .parse(input)
}

/// Parses a single reaction (e.g., "10 ORE => 10 A").
fn parse_reaction(input: &str) -> IResult<&str, Reaction> {
    separated_pair(
        separated_list1(tag(", "), parse_chemical_qty),
        tag(" => "),
        parse_chemical_qty,
    )
    .map(|(inputs, output)| Reaction { inputs, output })
    .parse(input)
}

/// Parses all reactions from the input string.
fn parse_reactions(input: &str) -> IResult<&str, Vec<Reaction>> {
    separated_list1(terminated(line_ending, multispace0), parse_reaction).parse(input)
}

/// Solves Year 2019, Day 14: Space Stoichiometry.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(14))?;
    calculate_solution(&input)
}

/// Core logic for Year 2019, Day 14.
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let (_, reactions) = parse_reactions(input.trim())
        .map_err(|e| anyhow!("Failed to parse reactions: {}", e))?;

    let mut reaction_map = HashMap::new();
    for r in reactions {
        reaction_map.insert(r.output.name.clone(), r);
    }

    let p1 = calculate_ore_needed(&reaction_map, 1)?;

    let total_ore = 1_000_000_000_000u64;
    let mut low = total_ore / p1;
    let mut high = low * 2;
    
    // Ensure high is actually high enough
    while calculate_ore_needed(&reaction_map, high)? <= total_ore {
        low = high;
        high *= 2;
    }

    let mut p2 = low;
    while low <= high {
        let mid = low + (high - low) / 2;
        if mid == 0 {
            low = 1;
            continue;
        }
        let ore = calculate_ore_needed(&reaction_map, mid)?;
        if ore <= total_ore {
            p2 = mid;
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    Ok((p1, p2))
}

/// Calculates the ORE needed to produce a specific amount of FUEL.
fn calculate_ore_needed(reaction_map: &HashMap<String, Reaction>, fuel_amount: u64) -> Result<u64> {
    let mut in_degree = HashMap::new();
    // Initialize degrees for all chemicals present in the reaction map
    for name in reaction_map.keys() {
        in_degree.entry(name.clone()).or_insert(0);
    }
    in_degree.entry("ORE".to_string()).or_insert(0);

    for r in reaction_map.values() {
        for input in &r.inputs {
            *in_degree.entry(input.name.clone()).or_insert(0) += 1;
        }
    }
    
    let mut requirements = HashMap::new();
    requirements.insert("FUEL".to_string(), fuel_amount);
    
    let mut available_to_process: VecDeque<String> = in_degree.iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(name, _)| name.clone())
        .collect();

    let mut ore_needed = 0;

    while let Some(current) = available_to_process.pop_front() {
        if current == "ORE" {
            continue;
        }
        
        let req_amount = requirements.get(&current).copied().unwrap_or(0);
        let reaction = reaction_map.get(&current).ok_or_else(|| anyhow!("Reaction not found for chemical: {}", current))?;
        
        let batch_size = reaction.output.amount;
        let num_batches = req_amount.div_ceil(batch_size);
        
        for input in &reaction.inputs {
            let total_input_needed = num_batches * input.amount;
            if input.name == "ORE" {
                ore_needed += total_input_needed;
            } else {
                *requirements.entry(input.name.clone()).or_insert(0) += total_input_needed;
                
                let degree = in_degree.get_mut(&input.name).context("Degree not found")?;
                *degree -= 1;
                if *degree == 0 {
                    available_to_process.push_back(input.name.clone());
                }
            }
        }
    }

    Ok(ore_needed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let (p1, _) = calculate_solution(input)?;
        assert_eq!(p1, 31);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let (p1, _) = calculate_solution(input)?;
        assert_eq!(p1, 165);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 13312);
        assert_eq!(p2, 82892753);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let input = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 180697);
        assert_eq!(p2, 5586022);
        Ok(())
    }

    #[test]
    fn test_example_5() -> Result<()> {
        let input = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        let (p1, p2) = calculate_solution(input)?;
        assert_eq!(p1, 2210736);
        assert_eq!(p2, 460664);
        Ok(())
    }
}
