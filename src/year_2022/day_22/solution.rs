use anyhow::{anyhow, Result};
use crate::utils::{read_input, Year, Day};

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Move(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Void,
}

struct Map {
    grid: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
    face_size: usize,
}

impl Map {
    fn get(&self, r: i32, c: i32) -> Tile {
        if r < 0 || r >= self.rows as i32 || c < 0 || c >= self.cols as i32 {
            Tile::Void
        } else {
            self.grid[r as usize][c as usize]
        }
    }
}

/// Calculate the solution for Day 22.
/// # Errors
/// Returns an error if input is malformed.
pub fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let split_idx = input.find("\n\n").or_else(|| input.find("\r\n\r\n"));
    let (map_str, path_str) = if let Some(idx) = split_idx {
        let (m, p) = input.split_at(idx);
        (m, p.trim())
    } else {
        return Err(anyhow!("Invalid input format"));
    };

    let map_lines: Vec<&str> = map_str.lines().collect();
    let rows = map_lines.len();
    let cols = map_lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut grid = vec![vec![Tile::Void; cols]; rows];
    for (r, line) in map_lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            grid[r][c] = match ch {
                '.' => Tile::Open,
                '#' => Tile::Wall,
                _ => Tile::Void,
            };
        }
    }

    let face_size = if rows > 50 { 50 } else { 4 };
    let map = Map { grid, rows, cols, face_size };

    let mut instructions = Vec::new();
    let mut num_str = String::new();
    for ch in path_str.chars() {
        if ch.is_ascii_digit() {
            num_str.push(ch);
        } else {
            if !num_str.is_empty() {
                instructions.push(Instruction::Move(num_str.parse()?));
                num_str.clear();
            }
            if ch == 'L' {
                instructions.push(Instruction::TurnLeft);
            } else if ch == 'R' {
                instructions.push(Instruction::TurnRight);
            }
        }
    }
    if !num_str.is_empty() {
        instructions.push(Instruction::Move(num_str.parse()?));
    }

    let p1 = solve_part(&map, &instructions, false);
    let p2 = solve_part(&map, &instructions, true);

    Ok((p1, p2))
}

fn solve_part(map: &Map, instructions: &[Instruction], is_cube: bool) -> u64 {
    let mut r = 0;
    let mut c = 0;
    for (idx, &tile) in map.grid[0].iter().enumerate() {
        if tile == Tile::Open {
            c = idx;
            break;
        }
    }

    let mut facing = 0; // 0: R, 1: D, 2: L, 3: U

    for &instr in instructions {
        match instr {
            Instruction::TurnLeft => facing = (facing + 3) % 4,
            Instruction::TurnRight => facing = (facing + 1) % 4,
            Instruction::Move(steps) => {
                for _ in 0..steps {
                    let (dr, dc) = match facing {
                        0 => (0, 1),
                        1 => (1, 0),
                        2 => (0, -1),
                        3 => (-1, 0),
                        _ => unreachable!(),
                    };

                    let next_r = r as i32 + dr;
                    let next_c = c as i32 + dc;

                    let mut nr = next_r;
                    let mut nc = next_c;
                    let mut nf = facing;

                    if map.get(nr, nc) == Tile::Void {
                        if !is_cube {
                            // Part 1 wrapping
                            if dr == 1 {
                                for rr in 0..map.rows {
                                    if map.get(rr as i32, c as i32) != Tile::Void { nr = rr as i32; break; }
                                }
                            } else if dr == -1 {
                                for rr in (0..map.rows).rev() {
                                    if map.get(rr as i32, c as i32) != Tile::Void { nr = rr as i32; break; }
                                }
                            } else if dc == 1 {
                                for cc in 0..map.cols {
                                    if map.get(r as i32, cc as i32) != Tile::Void { nc = cc as i32; break; }
                                }
                            } else if dc == -1 {
                                for cc in (0..map.cols).rev() {
                                    if map.get(r as i32, cc as i32) != Tile::Void { nc = cc as i32; break; }
                                }
                            }
                        } else {
                            // Part 2 wrapping (Cube)
                            let (target_r, target_c, target_f) = if map.face_size == 4 {
                                wrap_cube_example(r as i32, c as i32, facing, map.face_size)
                            } else {
                                wrap_cube_real(r as i32, c as i32, facing, map.face_size)
                            };
                            nr = target_r;
                            nc = target_c;
                            nf = target_f;
                        }
                    }

                    if map.get(nr, nc) == Tile::Wall {
                        break;
                    }
                    r = nr as usize;
                    c = nc as usize;
                    facing = nf;
                }
            }
        }
    }

    1000 * (r as u64 + 1) + 4 * (c as u64 + 1) + facing as u64
}

fn wrap_cube_example(r: i32, c: i32, f: i32, s: usize) -> (i32, i32, i32) {
    let s = s as i32;
    let face_r = r / s;
    let face_c = c / s;
    let local_r = r % s;
    let local_c = c % s;

    match (face_r, face_c, f) {
        // Face 1 (0, 2)
        (0, 2, 0) => (2 * s + (s - 1 - local_r), 4 * s - 1, 2), // 1R -> 6R (flip)
        (0, 2, 2) => (s, s + local_r, 1),                      // 1L -> 3D
        (0, 2, 3) => (s, s - 1 - local_c, 1),                  // 1U -> 2D (flip)
        // Face 2 (1, 0)
        (1, 0, 1) => (3 * s - 1, 3 * s - 1 - local_c, 3),      // 2D -> 5U (flip)
        (1, 0, 2) => (3 * s - 1, 4 * s - 1 - local_r, 3),      // 2L -> 6U (flip)
        (1, 0, 3) => (0, 3 * s - 1 - local_c, 1),              // 2U -> 1D (flip)
        // Face 3 (1, 1)
        (1, 1, 1) => (3 * s - 1 - local_c, 2 * s, 0),          // 3D -> 5L
        (1, 1, 3) => (local_c, 2 * s, 0),                      // 3U -> 1L
        // Face 4 (1, 2)
        (1, 2, 0) => (2 * s, 4 * s - 1 - local_r, 1),          // 4R -> 6D (flip)
        // Face 5 (2, 2)
        (2, 2, 1) => (2 * s - 1, s - 1 - local_c, 3),          // 5D -> 2U (flip)
        (2, 2, 2) => (2 * s - 1, 2 * s - 1 - local_r, 3),      // 5L -> 3U (flip)
        // Face 6 (2, 3)
        (2, 3, 0) => (s - 1 - local_r, 3 * s - 1, 2),          // 6R -> 1R (flip)
        (2, 3, 1) => (2 * s - 1 - local_c, 0, 0),              // 6D -> 2R (flip)
        (2, 3, 3) => (2 * s - 1 - local_c, 3 * s - 1, 2),      // 6U -> 4R
        _ => unreachable!("Invalid wrap in example: ({}, {}, {})", face_r, face_c, f),
    }
}

fn wrap_cube_real(r: i32, c: i32, f: i32, s: usize) -> (i32, i32, i32) {
    let s = s as i32;
    let face_r = r / s;
    let face_c = c / s;
    let local_r = r % s;
    let local_c = c % s;

    match (face_r, face_c, f) {
        // Face 1 (0, 1)
        (0, 1, 2) => (3 * s - 1 - local_r, 0, 0),              // 1L -> 4L (flip)
        (0, 1, 3) => (3 * s + local_c, 0, 0),                  // 1U -> 6L
        // Face 2 (0, 2)
        (0, 2, 0) => (3 * s - 1 - local_r, 2 * s - 1, 2),      // 2R -> 5R (flip)
        (0, 2, 1) => (s + local_c, 2 * s - 1, 2),              // 2D -> 3R
        (0, 2, 3) => (4 * s - 1, local_c, 3),                  // 2U -> 6D
        // Face 3 (1, 1)
        (1, 1, 0) => (s - 1, 2 * s + local_r, 3),              // 3R -> 2U
        (1, 1, 2) => (2 * s, local_r, 1),                      // 3L -> 4D
        // Face 4 (2, 0)
        (2, 0, 2) => (s - 1 - local_r, s, 0),                  // 4L -> 1L (flip)
        (2, 0, 3) => (s + local_c, s, 0),                      // 4U -> 3L
        // Face 5 (2, 1)
        (2, 1, 0) => (s - 1 - local_r, 3 * s - 1, 2),          // 5R -> 2R (flip)
        (2, 1, 1) => (3 * s + local_c, s - 1, 2),              // 5D -> 6R
        // Face 6 (3, 0)
        (3, 0, 0) => (3 * s - 1, s + local_r, 3),              // 6R -> 5U
        (3, 0, 1) => (0, 2 * s + local_c, 1),                  // 6D -> 2D
        (3, 0, 2) => (0, s + local_r, 1),                      // 6L -> 1D
        _ => unreachable!("Invalid wrap in real: ({}, {}, {})", face_r, face_c, f),
    }
}

/// Solve year 2022 day 22.
/// # Errors
/// Returns an error if input cannot be read.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2022), Day(22))?;
    calculate_solution(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_example_p1() {
        let (p1, _) = calculate_solution(EXAMPLE).unwrap();
        assert_eq!(p1, 6032);
    }

    #[test]
    fn test_example_p2() {
        let (_, p2) = calculate_solution(EXAMPLE).unwrap();
        assert_eq!(p2, 5031);
    }
}
