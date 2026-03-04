use crate::utils::{read_input, Day, Year};
use anyhow::Result;
use std::collections::{BTreeMap, BTreeSet};

/// Solves Year 2018, Day 07: The Sum of Its Parts.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2018), Day(7))?;
    let part1 = solve_part1(&content);
    println!("[2018] Day 07 Part 1 Order: {}", part1);
    let part2 = solve_part2(&content, 5, 60);
    Ok((0, part2))
}

fn parse_dependencies(input: &str) -> (BTreeSet<char>, BTreeMap<char, BTreeSet<char>>) {
    let mut all_steps = BTreeSet::new();
    let mut deps: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        // Step C must be finished before step A can begin.
        let parts: Vec<&str> = line.split_whitespace().collect();
        let before = parts[1].chars().next().unwrap();
        let after = parts[7].chars().next().unwrap();
        all_steps.insert(before);
        all_steps.insert(after);
        deps.entry(after).or_default().insert(before);
    }
    (all_steps, deps)
}

fn solve_part1(input: &str) -> String {
    let (mut all_steps, mut deps) = parse_dependencies(input);
    let mut result = String::new();

    while !all_steps.is_empty() {
        let next_step = *all_steps
            .iter()
            .find(|&&step| {
                deps.get(&step).is_none_or(|d| d.is_empty())
            })
            .unwrap();

        result.push(next_step);
        all_steps.remove(&next_step);

        for d in deps.values_mut() {
            d.remove(&next_step);
        }
    }

    result
}

fn solve_part2(input: &str, num_workers: usize, base_duration: u32) -> u64 {
    let (mut all_steps, mut deps) = parse_dependencies(input);
    let mut time = 0;
    let mut workers: Vec<Option<(char, u32)>> = vec![None; num_workers];

    while !all_steps.is_empty() || workers.iter().any(|w| w.is_some()) {
        // Check for finished workers
        for worker in &mut workers {
            if let Some((step, finish_time)) = *worker {
                if finish_time == time {
                    *worker = None;
                    // Remove from dependencies
                    for d in deps.values_mut() {
                        d.remove(&step);
                    }
                }
            }
        }

        // Assign available steps to idle workers
        let mut available_steps: Vec<char> = all_steps
            .iter()
            .filter(|&&step| {
                deps.get(&step).is_none_or(|d| d.is_empty())
            })
            .copied()
            .collect();
        
        available_steps.sort_unstable();

        for worker in &mut workers {
            if worker.is_none() && !available_steps.is_empty() {
                let step = available_steps.remove(0);
                all_steps.remove(&step);
                let duration = base_duration + (step as u32 - 'A' as u32 + 1);
                *worker = Some((step, time + duration));
            }
        }

        if all_steps.is_empty() && workers.iter().all(|w| w.is_none()) {
            break;
        }

        time += 1;
    }

    time as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1_example() {
        assert_eq!(solve_part1(EXAMPLE), "CABDFE");
    }

    #[test]
    fn test_part2_example() {
        assert_eq!(solve_part2(EXAMPLE, 2, 0), 15);
    }
}
