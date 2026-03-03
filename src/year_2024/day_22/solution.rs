use crate::utils::{read_input, Year, Day};
use anyhow::{Context, Result};
use std::collections::HashMap;

/// Solves Year 2024, Day 22: Monkey Market.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2024), Day(22))?;
    let initial_secrets: Vec<u64> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.parse::<u64>()
                .with_context(|| format!("Failed to parse secret: '{}'", l))
        })
        .collect::<Result<Vec<u64>>>()?;

    let mut part1 = 0;
    let mut total_bananas: HashMap<[i8; 4], u64> = HashMap::new();

    for &secret in &initial_secrets {
        let mut current = secret;
        let mut prices = Vec::with_capacity(2001);
        prices.push((current % 10) as i8);

        for _ in 0..2000 {
            current = next_secret(current);
            prices.push((current % 10) as i8);
        }
        part1 += current;

        let mut seen_in_this_buyer = HashSet::new();
        for i in 0..prices.len() - 4 {
            let p1 = prices[i];
            let p2 = prices[i + 1];
            let p3 = prices[i + 2];
            let p4 = prices[i + 3];
            let p5 = prices[i + 4];

            let diffs = [p2 - p1, p3 - p2, p4 - p3, p5 - p4];
            if seen_in_this_buyer.insert(diffs) {
                *total_bananas.entry(diffs).or_default() += p5 as u64;
            }
        }
    }

    let part2 = total_bananas.values().copied().max().unwrap_or(0);

    Ok((part1, part2))
}

use std::collections::HashSet;

fn next_secret(mut secret: u64) -> u64 {
    // Step 1: multiply by 64, mix, prune
    secret = ((secret * 64) ^ secret) % 16777216;
    // Step 2: divide by 32, mix, prune
    secret = ((secret / 32) ^ secret) % 16777216;
    // Step 3: multiply by 2048, mix, prune
    secret = ((secret * 2048) ^ secret) % 16777216;
    secret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        let mut s = 123;
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432,
            5908254,
        ];
        for &e in &expected {
            s = next_secret(s);
            assert_eq!(s, e);
        }
    }

    #[test]
    fn test_part1_example() {
        let input = [1, 10, 100, 2024];
        let mut sum = 0;
        for mut s in input {
            for _ in 0..2000 {
                s = next_secret(s);
            }
            sum += s;
        }
        assert_eq!(sum, 37327623);
    }

    #[test]
    fn test_part2_example() {
        let initial_secrets = vec![1, 2, 3, 2024];
        let mut total_bananas: HashMap<[i8; 4], u64> = HashMap::new();

        for &secret in &initial_secrets {
            let mut current = secret;
            let mut prices = Vec::with_capacity(2001);
            prices.push((current % 10) as i8);

            for _ in 0..2000 {
                current = next_secret(current);
                prices.push((current % 10) as i8);
            }

            let mut seen_in_this_buyer = HashSet::new();
            for i in 0..prices.len() - 4 {
                let p1 = prices[i];
                let p2 = prices[i + 1];
                let p3 = prices[i + 2];
                let p4 = prices[i + 3];
                let p5 = prices[i + 4];

                let diffs = [p2 - p1, p3 - p2, p4 - p3, p5 - p4];
                if seen_in_this_buyer.insert(diffs) {
                    *total_bananas.entry(diffs).or_default() += p5 as u64;
                }
            }
        }
        let part2 = total_bananas.values().copied().max().unwrap_or(0);
        assert_eq!(part2, 23);
    }
}

