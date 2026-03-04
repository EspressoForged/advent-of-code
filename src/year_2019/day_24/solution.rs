use crate::utils::{read_input, Day, Year};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Solves Year 2019, Day 24: Planet of Discord.
///
/// # Errors
/// Returns an error if the input cannot be read.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(24))?;
    let initial_grid = parse_grid(&input);

    let part1 = solve_part1(initial_grid);
    let part2 = solve_part2(initial_grid, 200);

    Ok((part1 as u64, part2 as u64))
}

fn parse_grid(input: &str) -> u32 {
    let mut grid = 0;
    for (i, c) in input.chars().filter(|&c| c == '#' || c == '.').enumerate() {
        if c == '#' {
            grid |= 1 << i;
        }
    }
    grid
}

fn solve_part1(initial_grid: u32) -> u32 {
    let mut grid = initial_grid;
    let mut seen = HashSet::new();
    seen.insert(grid);

    loop {
        let mut next_grid = 0;
        for i in 0..25 {
            let neighbors = count_neighbors_p1(grid, i);
            let bug = (grid >> i) & 1 == 1;
            if bug {
                if neighbors == 1 {
                    next_grid |= 1 << i;
                }
            } else if neighbors == 1 || neighbors == 2 {
                next_grid |= 1 << i;
            }
        }
        grid = next_grid;
        if !seen.insert(grid) {
            return grid;
        }
    }
}

fn count_neighbors_p1(grid: u32, i: usize) -> u32 {
    let mut count = 0;
    let x = i % 5;
    let y = i / 5;

    if y > 0 && (grid >> (i - 5)) & 1 == 1 {
        count += 1;
    }
    if y < 4 && (grid >> (i + 5)) & 1 == 1 {
        count += 1;
    }
    if x > 0 && (grid >> (i - 1)) & 1 == 1 {
        count += 1;
    }
    if x < 4 && (grid >> (i + 1)) & 1 == 1 {
        count += 1;
    }
    count
}

fn solve_part2(initial_grid: u32, minutes: i32) -> u32 {
    let mut grids = HashMap::new();
    // Clear center tile for part 2
    grids.insert(0, initial_grid & !(1 << 12));

    for _ in 0..minutes {
        let mut next_grids = HashMap::new();
        let &min_level = grids.keys().min().unwrap_or(&0);
        let &max_level = grids.keys().max().unwrap_or(&0);

        for level in (min_level - 1)..=(max_level + 1) {
            let mut next_grid = 0;
            for i in 0..25 {
                if i == 12 {
                    continue;
                }
                let neighbors = count_neighbors_p2(&grids, level, i);
                let current_grid = grids.get(&level).copied().unwrap_or(0);
                let bug = (current_grid >> i) & 1 == 1;
                if bug {
                    if neighbors == 1 {
                        next_grid |= 1 << i;
                    }
                } else if neighbors == 1 || neighbors == 2 {
                    next_grid |= 1 << i;
                }
            }
            if next_grid != 0 {
                next_grids.insert(level, next_grid);
            }
        }
        grids = next_grids;
    }

    grids.values().map(|&g| g.count_ones()).sum()
}

fn count_neighbors_p2(grids: &HashMap<i32, u32>, level: i32, i: usize) -> u32 {
    let mut count = 0;
    let x = i % 5;
    let y = i / 5;

    // Up
    if y == 0 {
        // level - 1, bit 7
        if let Some(upper) = grids.get(&(level - 1)) {
            if (upper >> 7) & 1 == 1 {
                count += 1;
            }
        }
    } else if y == 3 && x == 2 {
        // level + 1, bottom row (20-24)
        if let Some(lower) = grids.get(&(level + 1)) {
            for b in 20..25 {
                if (lower >> b) & 1 == 1 {
                    count += 1;
                }
            }
        }
    } else if let Some(grid) = grids.get(&level) {
        if (grid >> (i - 5)) & 1 == 1 {
            count += 1;
        }
    }

    // Down
    if y == 4 {
        // level - 1, bit 17
        if let Some(upper) = grids.get(&(level - 1)) {
            if (upper >> 17) & 1 == 1 {
                count += 1;
            }
        }
    } else if y == 1 && x == 2 {
        // level + 1, top row (0-4)
        if let Some(lower) = grids.get(&(level + 1)) {
            for b in 0..5 {
                if (lower >> b) & 1 == 1 {
                    count += 1;
                }
            }
        }
    } else if let Some(grid) = grids.get(&level) {
        if (grid >> (i + 5)) & 1 == 1 {
            count += 1;
        }
    }

    // Left
    if x == 0 {
        // level - 1, bit 11
        if let Some(upper) = grids.get(&(level - 1)) {
            if (upper >> 11) & 1 == 1 {
                count += 1;
            }
        }
    } else if x == 3 && y == 2 {
        // level + 1, right column (4,9,14,19,24)
        if let Some(lower) = grids.get(&(level + 1)) {
            for b in [4, 9, 14, 19, 24] {
                if (lower >> b) & 1 == 1 {
                    count += 1;
                }
            }
        }
    } else if let Some(grid) = grids.get(&level) {
        if (grid >> (i - 1)) & 1 == 1 {
            count += 1;
        }
    }

    // Right
    if x == 4 {
        // level - 1, bit 13
        if let Some(upper) = grids.get(&(level - 1)) {
            if (upper >> 13) & 1 == 1 {
                count += 1;
            }
        }
    } else if x == 1 && y == 2 {
        // level + 1, left column (0,5,10,15,20)
        if let Some(lower) = grids.get(&(level + 1)) {
            for b in [0, 5, 10, 15, 20] {
                if (lower >> b) & 1 == 1 {
                    count += 1;
                }
            }
        }
    } else if let Some(grid) = grids.get(&level) {
        if (grid >> (i + 1)) & 1 == 1 {
            count += 1;
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {
        let input = "....#
#..#.
#..##
..#..
#....";
        let initial_grid = parse_grid(input);
        assert_eq!(solve_part1(initial_grid), 2129920);
    }

    #[test]
    fn test_part2_example() {
        let input = "....#
#..#.
#..##
..#..
#....";
        let initial_grid = parse_grid(input);
        assert_eq!(solve_part2(initial_grid, 10), 99);
    }
}
