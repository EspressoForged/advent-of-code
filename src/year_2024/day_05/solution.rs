use crate::utils::read_input;
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};

/// Solves Year 2024, Day 5: Print Queue.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(2024, 5)?;
    let (rules_str, updates_str) = input
        .split_once("\r\n\r\n")
        .or_else(|| input.split_once("\n\n"))
        .context("Invalid input format: missing double newline")?;

    // Parse rules: X must be before Y
    // Map of X -> Set of all Y that must be after X
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for line in rules_str.lines() {
        if let Some((x_str, y_str)) = line.split_once('|') {
            let x = x_str.trim().parse::<u32>()?;
            let y = y_str.trim().parse::<u32>()?;
            rules.entry(x).or_default().insert(y);
        }
    }

    // Parse updates
    let updates: Vec<Vec<u32>> = updates_str
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(',')
                .map(|s| {
                    s.trim()
                        .parse::<u32>()
                        .with_context(|| format!("Failed to parse page number: '{}'", s))
                })
                .collect::<Result<Vec<u32>>>()
        })
        .collect::<Result<Vec<Vec<u32>>>>()?;

    let mut part1_sum = 0;
    let mut part2_sum = 0;

    for mut update in updates {
        if is_ordered(&update, &rules) {
            if let Some(&mid) = update.get(update.len() / 2) {
                part1_sum += mid as u64;
            }
        } else {
            // Part 2: Re-order correctly
            reorder(&mut update, &rules);
            if let Some(&mid) = update.get(update.len() / 2) {
                part2_sum += mid as u64;
            }
        }
    }

    Ok((part1_sum, part2_sum))
}

fn is_ordered(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    for i in 0..update.len() {
        for j in i + 1..update.len() {
            let x = update[i];
            let y = update[j];
            // If there's a rule that says y must be before x, then it's wrong
            if let Some(after_y) = rules.get(&y) {
                if after_y.contains(&x) {
                    return false;
                }
            }
        }
    }
    true
}

fn reorder(update: &mut [u32], rules: &HashMap<u32, HashSet<u32>>) {
    update.sort_by(|&a, &b| {
        if let Some(after_a) = rules.get(&a) {
            if after_a.contains(&b) {
                return std::cmp::Ordering::Less;
            }
        }
        if let Some(after_b) = rules.get(&b) {
            if after_b.contains(&a) {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_05_example() {
        let example_input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

        let (rules_str, updates_str) = example_input.split_once("\n\n").unwrap();
        let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
        for line in rules_str.lines() {
            if let Some((x_str, y_str)) = line.split_once('|') {
                let x = x_str.trim().parse::<u32>().unwrap();
                let y = y_str.trim().parse::<u32>().unwrap();
                rules.entry(x).or_default().insert(y);
            }
        }

        let updates: Vec<Vec<u32>> = updates_str
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|s| s.trim().parse::<u32>().unwrap())
                    .collect()
            })
            .collect();

        let mut part1_sum = 0;
        let mut part2_sum = 0;
        for mut update in updates {
            if is_ordered(&update, &rules) {
                part1_sum += update[update.len() / 2] as u64;
            } else {
                reorder(&mut update, &rules);
                part2_sum += update[update.len() / 2] as u64;
            }
        }

        assert_eq!(part1_sum, 143);
        assert_eq!(part2_sum, 123);
    }
}
