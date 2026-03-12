use anyhow::Result;
use std::collections::HashMap;
use crate::utils::{Year, Day, read_input};

/// # Errors
/// Returns error if the input cannot be read.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2022), Day(17))?;
    let jets = input.trim().as_bytes();
    
    let p1 = simulate(jets, 2022);
    let p2 = simulate(jets, 1_000_000_000_000);

    Ok((p1, p2))
}

fn simulate(jets: &[u8], total_rocks: u64) -> u64 {
    let rocks = [
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];

    let mut chamber: Vec<u8> = Vec::new();
    let mut max_y = -1i64;
    let mut jet_idx = 0usize;
    let mut top_heights = [-1i64; 7];
    
    // State for cycle detection: (rock_idx, jet_idx, profile) -> (rock_count, max_y)
    let mut seen_states: HashMap<(usize, usize, [i64; 7]), (u64, i64)> = HashMap::new();
    let mut added_by_cycles = 0u64;

    let mut r = 0u64;
    while r < total_rocks {
        let rock_idx = (r % 5) as usize;
        
        // Cycle detection profile: relative heights of each column
        let mut profile = [0i64; 7];
        for i in 0..7 {
            profile[i] = top_heights[i] - max_y;
        }

        let state = (rock_idx, jet_idx, profile);
        if let Some((prev_r, prev_max_y)) = seen_states.get(&state) {
            let cycle_len = r - prev_r;
            let cycle_height = max_y - prev_max_y;
            let remaining_rocks = total_rocks - r;
            let num_cycles = remaining_rocks / cycle_len;
            
            added_by_cycles += num_cycles * (cycle_height as u64);
            r += num_cycles * cycle_len;
            
            // After jumping, clear the map to prevent further jumps
            seen_states.clear();
        } else {
            seen_states.insert(state, (r, max_y));
        }

        if r >= total_rocks {
            break;
        }

        let rock = &rocks[rock_idx];
        let mut x = 2i64;
        let mut y = max_y + 4;

        loop {
            // Jet push
            let jet = jets[jet_idx];
            jet_idx = (jet_idx + 1) % jets.len();

            let dx = if jet == b'<' { -1 } else { 1 };
            if can_move(rock, x + dx, y, &chamber) {
                x += dx;
            }

            // Fall
            if can_move(rock, x, y - 1, &chamber) {
                y -= 1;
            } else {
                // Stop
                for &(rx, ry) in rock {
                    let rx_abs = x + rx;
                    let ry_abs = y + ry;
                    
                    if ry_abs >= chamber.len() as i64 {
                        chamber.resize((ry_abs + 1) as usize, 0);
                    }
                    chamber[ry_abs as usize] |= 1 << rx_abs;
                    
                    if ry_abs > max_y {
                        max_y = ry_abs;
                    }
                    if ry_abs > top_heights[rx_abs as usize] {
                        top_heights[rx_abs as usize] = ry_abs;
                    }
                }
                break;
            }
        }
        r += 1;
    }

    (max_y + 1) as u64 + added_by_cycles
}

fn can_move(rock: &[(i64, i64)], x: i64, y: i64, chamber: &[u8]) -> bool {
    for &(rx, ry) in rock {
        let nx = x + rx;
        let ny = y + ry;
        if !(0..7).contains(&nx) || ny < 0 {
            return false;
        }
        if ny < chamber.len() as i64 && (chamber[ny as usize] & (1 << nx)) != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() -> Result<()> {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = input.trim().as_bytes();
        assert_eq!(simulate(jets, 2022), 3068);
        assert_eq!(simulate(jets, 1_000_000_000_000), 1514285714288);
        Ok(())
    }
}
