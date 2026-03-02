use crate::utils::read_input;
use anyhow::Result;

/// Core logic for Year 2015, Day 01
fn calculate_solution(_input: &str) -> Result<(u64, u64)> {
    // TODO: Implement solution
    Ok((0, 0))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2015, 1)?;
    calculate_solution(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_day_01_solution() -> Result<()> {
        let (p1, p2) = calculate_solution(TEST_INPUT)?;
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
        Ok(())
    }
}
