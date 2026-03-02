use crate::utils::read_input;
use anyhow::Result;
use rayon::prelude::*;
use std::sync::atomic::{AtomicU32, Ordering};

/// Evolves the secret number one step.
fn next_secret(mut s: u64) -> u64 {
    s = (s ^ (s << 6)) & 0xFFFFFF;
    s = (s ^ (s >> 5)) & 0xFFFFFF;
    s = (s ^ (s << 11)) & 0xFFFFFF;
    s
}

/// Core logic for Year 2024, Day 22
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let initials: Vec<u64> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u64>().unwrap())
        .collect();

    // Use AtomicU32 for thread-safe global updates
    let num_sequences = 19usize.pow(4);
    let global_bananas: Vec<AtomicU32> = (0..num_sequences)
        .map(|_| AtomicU32::new(0))
        .collect();

    let part1_sum: u64 = initials
        .par_iter()
        .map(|&initial| {
            let mut s = initial;
            let mut prices = Vec::with_capacity(2001);
            prices.push((s % 10) as i8);

            for _ in 0..2000 {
                s = next_secret(s);
                prices.push((s % 10) as i8);
            }

            // seen_at[hash] stores the buyer index to identify first-seen sequences.
            // However, since we process buyers in parallel, it's easier to use a local
            // bitset or seen array for each buyer.
            // A bitset for 130,321 bits is ~16KB.
            let mut seen = vec![false; 19usize.pow(4)];
            
            for i in 1..=1997 {
                let d1 = prices[i] - prices[i-1];
                let d2 = prices[i+1] - prices[i];
                let d3 = prices[i+2] - prices[i+1];
                let d4 = prices[i+3] - prices[i+2];

                let h1 = (d1 + 9) as usize;
                let h2 = (d2 + 9) as usize;
                let h3 = (d3 + 9) as usize;
                let h4 = (d4 + 9) as usize;

                let hash = h1 * 6859 + h2 * 361 + h3 * 19 + h4;
                
                if !seen[hash] {
                    seen[hash] = true;
                    global_bananas[hash].fetch_add(prices[i+3] as u32, Ordering::Relaxed);
                }
            }
            s
        })
        .sum();

    let part2_max = global_bananas
        .iter()
        .map(|a| a.load(Ordering::Relaxed))
        .max()
        .unwrap_or(0);

    Ok((part1_sum, part2_max as u64))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2024, 22)?;
    calculate_solution(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_secret() {
        let mut s = 123;
        let expected = [
            15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484, 7753432, 5908254,
        ];
        for &e in expected.iter() {
            s = next_secret(s);
            assert_eq!(s, e);
        }
    }

    #[test]
    fn test_part1_example() -> Result<()> {
        let input = "1\n10\n100\n2024";
        let (p1, _) = calculate_solution(input)?;
        assert_eq!(p1, 37327623);
        Ok(())
    }

    #[test]
    fn test_part2_example() -> Result<()> {
        let input = "1\n2\n3\n2024";
        let (_, p2) = calculate_solution(input)?;
        assert_eq!(p2, 23);
        Ok(())
    }
}
