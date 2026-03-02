use crate::utils::read_input;
use anyhow::Result;

/// Core logic for Year 2024, Day 04
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();

    if grid.is_empty() {
        return Ok((0, 0));
    }

    let rows = grid.len();
    let cols = grid[0].len();
    
    // --- Part 1 ---
    let target = "XMAS".as_bytes();
    let mut count_p1 = 0;

    let directions = [
        (0, 1),   // E
        (0, -1),  // W
        (1, 0),   // S
        (-1, 0),  // N
        (1, 1),   // SE
        (1, -1),  // SW
        (-1, 1),  // NE
        (-1, -1), // NW
    ];

    for r in 0..rows {
        for c in 0..cols {
            for &(dr, dc) in &directions {
                let mut found = true;
                for i in 0..4 {
                    let nr = r as isize + dr * i as isize;
                    let nc = c as isize + dc * i as isize;
                    if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
                        found = false;
                        break;
                    }
                    if grid[nr as usize][nc as usize] != target[i] as char {
                        found = false;
                        break;
                    }
                }
                if found {
                    count_p1 += 1;
                }
            }
        }
    }

    // --- Part 2 ---
    let mut count_p2 = 0;
    for r in 1..rows.saturating_sub(1) {
        for c in 1..cols.saturating_sub(1) {
            if grid[r][c] == 'A' {
                let tl = grid[r - 1][c - 1];
                let br = grid[r + 1][c + 1];
                let tr = grid[r - 1][c + 1];
                let bl = grid[r + 1][c - 1];

                let diag1 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
                let diag2 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');

                if diag1 && diag2 {
                    count_p2 += 1;
                }
            }
        }
    }

    Ok((count_p1, count_p2))
}

pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(2024, 04)?;
    calculate_solution(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_day_04_solution() -> Result<()> {
        let (p1, p2) = calculate_solution(TEST_INPUT)?;
        assert_eq!(p1, 18);
        assert_eq!(p2, 9);
        Ok(())
    }
}
