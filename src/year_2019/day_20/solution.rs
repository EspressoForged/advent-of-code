use crate::utils::{read_input, Day, Year};
use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet, VecDeque};

/// Solves Year 2019, Day 20: Donut Maze.
///
/// # Errors
/// Returns an error if the input cannot be read or BFS fails to find a path.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(20))?;
    let p1 = solve_part1(&input)?;
    let p2 = solve_part2(&input)?;
    Ok((p1, p2))
}

struct Maze {
    grid: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
    portals: HashMap<(usize, usize), PortalInfo>,
    start_pos: (usize, usize),
    end_pos: (usize, usize),
}

struct PortalInfo {
    target: Option<(usize, usize)>,
    is_outer: bool,
}

impl Maze {
    #[allow(clippy::needless_range_loop)]
    fn parse(input: &str) -> Result<Self> {
        let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let rows = grid.len();
        if rows == 0 {
            return Err(anyhow!("Empty grid"));
        }
        let cols = grid.iter().map(|r| r.len()).max().unwrap_or(0);

        // Standardize grid width
        let grid: Vec<Vec<char>> = grid
            .into_iter()
            .map(|mut r| {
                r.resize(cols, ' ');
                r
            })
            .collect();

        // Find maze boundaries (the '#' and '.' area)
        let mut min_r = rows;
        let mut max_r = 0;
        let mut min_c = cols;
        let mut max_c = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '#' || grid[r][c] == '.' {
                    min_r = min_r.min(r);
                    max_r = max_r.max(r);
                    min_c = min_c.min(c);
                    max_c = max_c.max(c);
                }
            }
        }

        let mut portal_labels: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c].is_ascii_uppercase() {
                    // Horizontal label
                    if c + 1 < cols && grid[r][c + 1].is_ascii_uppercase() {
                        let label = format!("{}{}", grid[r][c], grid[r][c + 1]);
                        if c > 0 && grid[r][c - 1] == '.' {
                            portal_labels.entry(label).or_default().push((r, c - 1));
                        } else if c + 2 < cols && grid[r][c + 2] == '.' {
                            portal_labels.entry(label).or_default().push((r, c + 2));
                        }
                    }
                    // Vertical label
                    if r + 1 < rows && grid[r + 1][c].is_ascii_uppercase() {
                        let label = format!("{}{}", grid[r][c], grid[r + 1][c]);
                        if r > 0 && grid[r - 1][c] == '.' {
                            portal_labels.entry(label).or_default().push((r - 1, c));
                        } else if r + 2 < rows && grid[r + 2][c] == '.' {
                            portal_labels.entry(label).or_default().push((r + 2, c));
                        }
                    }
                }
            }
        }

        let start_pos = portal_labels
            .get("AA")
            .and_then(|v| v.first())
            .copied()
            .ok_or_else(|| anyhow!("AA not found"))?;
        let end_pos = portal_labels
            .get("ZZ")
            .and_then(|v| v.first())
            .copied()
            .ok_or_else(|| anyhow!("ZZ not found"))?;

        let mut portals = HashMap::new();
        for (label, pos_list) in portal_labels {
            for &pos in &pos_list {
                let is_outer = pos.0 == min_r || pos.0 == max_r || pos.1 == min_c || pos.1 == max_c;
                let target = if label == "AA" || label == "ZZ" {
                    None
                } else {
                    pos_list.iter().find(|&&p| p != pos).copied()
                };
                portals.insert(
                    pos,
                    PortalInfo {
                        target,
                        is_outer,
                    },
                );
            }
        }

        Ok(Maze {
            grid,
            rows,
            cols,
            portals,
            start_pos,
            end_pos,
        })
    }
}

fn solve_part1(input: &str) -> Result<u64> {
    let maze = Maze::parse(input)?;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((maze.start_pos, 0u64));
    visited.insert(maze.start_pos);

    while let Some((pos, dist)) = queue.pop_front() {
        if pos == maze.end_pos {
            return Ok(dist);
        }

        let (r, c) = pos;
        let neighbors = [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)];

        for next in neighbors {
            if next.0 < maze.rows && next.1 < maze.cols && maze.grid[next.0][next.1] == '.' && !visited.contains(&next) {
                visited.insert(next);
                queue.push_back((next, dist + 1));
            }
        }

        if let Some(portal) = maze.portals.get(&pos) {
            if let Some(target) = portal.target {
                if !visited.contains(&target) {
                    visited.insert(target);
                    queue.push_back((target, dist + 1));
                }
            }
        }
    }

    Err(anyhow!("No path found for Part 1"))
}

fn solve_part2(input: &str) -> Result<u64> {
    let maze = Maze::parse(input)?;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // State: (row, col, level)
    queue.push_back((maze.start_pos.0, maze.start_pos.1, 0, 0u64));
    visited.insert((maze.start_pos.0, maze.start_pos.1, 0));

    while let Some((r, c, level, dist)) = queue.pop_front() {
        if r == maze.end_pos.0 && c == maze.end_pos.1 && level == 0 {
            return Ok(dist);
        }

        let neighbors = [(r.wrapping_sub(1), c), (r + 1, c), (r, c.wrapping_sub(1)), (r, c + 1)];

        for next in neighbors {
            if next.0 < maze.rows && next.1 < maze.cols && maze.grid[next.0][next.1] == '.' && !visited.contains(&(next.0, next.1, level)) {
                visited.insert((next.0, next.1, level));
                queue.push_back((next.0, next.1, level, dist + 1));
            }
        }

        if let Some(portal) = maze.portals.get(&(r, c)) {
            if let Some(target) = portal.target {
                let next_level = if portal.is_outer {
                    if level == 0 { None } else { Some(level - 1) }
                } else {
                    Some(level + 1)
                };

                if let Some(nl) = next_level {
                    if !visited.contains(&(target.0, target.1, nl)) {
                        visited.insert((target.0, target.1, nl));
                        queue.push_back((target.0, target.1, nl, dist + 1));
                    }
                }
            }
        }
    }

    Err(anyhow!("No path found for Part 2"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
        assert_eq!(solve_part1(input)?, 23);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";
        assert_eq!(solve_part1(input)?, 58);
        Ok(())
    }

    #[test]
    fn test_example_part2() -> Result<()> {
        let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        assert_eq!(solve_part2(input)?, 396);
        Ok(())
    }
}
