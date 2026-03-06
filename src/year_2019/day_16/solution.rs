use crate::utils::{read_input, Day, Year};
use anyhow::Result;

/// Solves Year 2019, Day 16: Flawed Frequency Transmission.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(16))?;
    calculate_solution(&input)
}

/// Core logic for Year 2019, Day 16.
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let input = input.trim();
    let digits: Vec<i32> = input
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect();

    let p1 = run_fft_part1(&digits, 100)?;
    let p2 = run_fft_part2(&digits, 100)?;

    Ok((p1, p2))
}

fn run_fft_part1(initial_digits: &[i32], phases: usize) -> Result<u64> {
    let mut current = initial_digits.to_vec();
    let n = current.len();

    for _ in 0..phases {
        // Build prefix sums: P[i] = sum(current[0..i])
        // P[0] = 0, P[1] = current[0], P[n] = sum(all)
        let mut prefix_sums = Vec::with_capacity(n + 1);
        prefix_sums.push(0);
        let mut running_sum = 0;
        for &d in &current {
            running_sum += d;
            prefix_sums.push(running_sum);
        }

        let mut next = Vec::with_capacity(n);
        for i in 1..=n {
            let mut sum: i32 = 0;
            let mut j = i - 1;
            while j < n {
                // Add range [j, j + i)
                let end = (j + i).min(n);
                sum += prefix_sums[end] - prefix_sums[j];

                j += 2 * i;
                if j >= n {
                    break;
                }

                // Subtract range [j, j + i)
                let end = (j + i).min(n);
                sum -= prefix_sums[end] - prefix_sums[j];
                j += 2 * i;
            }

            next.push(sum.abs() % 10);
        }
        current = next;
    }

    let mut res = 0;
    for &digit in current.iter().take(8) {
        res = res * 10 + digit as u64;
    }
    Ok(res)
}

fn run_fft_part2(initial_digits: &[i32], phases: usize) -> Result<u64> {
    let offset_str: String = initial_digits
        .iter()
        .take(7)
        .map(|d| d.to_string())
        .collect();
    let offset: usize = offset_str.parse()?;

    let n = initial_digits.len();
    let total_len = n * 10000;

    // In all puzzle inputs, the offset is in the second half.
    if offset <= total_len / 2 {
        return Err(anyhow::anyhow!(
            "Offset is not in the second half of the signal; optimization not applicable."
        ));
    }

    let mut signal = Vec::with_capacity(total_len - offset);
    for i in offset..total_len {
        signal.push(initial_digits[i % n]);
    }

    for _ in 0..phases {
        let mut suffix_sum = 0;
        for i in (0..signal.len()).rev() {
            suffix_sum = (suffix_sum + signal[i]) % 10;
            signal[i] = suffix_sum;
        }
    }

    let mut res = 0;
    for &digit in signal.iter().take(8) {
        res = res * 10 + digit as u64;
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() -> Result<()> {
        let input = "12345678";
        let digits: Vec<i32> = input
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part1(&digits, 1)?, 48226158);
        assert_eq!(run_fft_part1(&digits, 2)?, 34040438);
        assert_eq!(run_fft_part1(&digits, 3)?, 03415518);
        assert_eq!(run_fft_part1(&digits, 4)?, 01029498);
        Ok(())
    }

    #[test]
    fn test_larger_examples() -> Result<()> {
        let input1 = "80871224585914546619083218645595";
        let digits1: Vec<i32> = input1
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part1(&digits1, 100)?, 24176176);

        let input2 = "19617804207202209144916044189917";
        let digits2: Vec<i32> = input2
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part1(&digits2, 100)?, 73745418);

        let input3 = "69317163492948606335995924319873";
        let digits3: Vec<i32> = input3
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part1(&digits3, 100)?, 52432133);
        Ok(())
    }

    #[test]
    fn test_part2_examples() -> Result<()> {
        let input1 = "03036732577212944063491565474664";
        let digits1: Vec<i32> = input1
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part2(&digits1, 100)?, 84462026);

        let input2 = "02935109699940807407585447034323";
        let digits2: Vec<i32> = input2
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part2(&digits2, 100)?, 78725270);

        let input3 = "03081770884921959731165446850517";
        let digits3: Vec<i32> = input3
            .chars()
            .filter_map(|c| c.to_digit(10).map(|d| d as i32))
            .collect();
        assert_eq!(run_fft_part2(&digits3, 100)?, 53553731);
        Ok(())
    }
}
