use crate::utils::{read_input, Day, Year};
use anyhow::Result;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

/// Solves Year 2019, Day 18: Many-Worlds Interpretation.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(18))?;
    let part1 = solve_part1(&input)?;
    let part2 = solve_part2(&input)?;
    Ok((part1, part2))
}

fn solve_part1(input: &str) -> Result<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    solve_generic(&grid)
}

fn solve_part2(input: &str) -> Result<u64> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut center = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                center = (x, y);
                break;
            }
        }
    }

    // Modify map for part 2
    let (cx, cy) = center;
    grid[cy - 1][cx - 1] = '@';
    grid[cy - 1][cx] = '#';
    grid[cy - 1][cx + 1] = '@';
    grid[cy][cx - 1] = '#';
    grid[cy][cx] = '#';
    grid[cy][cx + 1] = '#';
    grid[cy + 1][cx - 1] = '@';
    grid[cy + 1][cx] = '#';
    grid[cy + 1][cx + 1] = '@';

    solve_generic(&grid)
}

fn solve_generic(grid: &[Vec<char>]) -> Result<u64> {
    let mut pois = HashMap::new();
    let mut start_chars = Vec::new();
    let mut next_start_idx = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                let start_char = (b'0' + next_start_idx) as char;
                pois.insert(start_char, (x, y));
                start_chars.push(start_char);
                next_start_idx += 1;
            } else if c.is_ascii_lowercase() {
                pois.insert(c, (x, y));
            }
        }
    }

    let num_keys = pois.len() - start_chars.len();
    let target_mask = (1 << num_keys) - 1;

    // Precompute distances and requirements between all POIs
    let mut adj = HashMap::new();
    for (&start_char, &start_pos) in &pois {
        let mut distances = HashMap::new();
        let mut queue = VecDeque::new();
        queue.push_back((start_pos, 0u32, 0u32));
        let mut visited = HashSet::new();
        visited.insert(start_pos);

        while let Some(((x, y), dist, req)) = queue.pop_front() {
            let c = grid[y][x];
            if c != start_char && (c.is_ascii_lowercase()) {
                distances.insert(c, (dist, req));
            }

            for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || ny < 0 || ny as usize >= grid.len() || nx as usize >= grid[0].len() {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                let nc = grid[ny][nx];

                if nc == '#' || visited.contains(&(nx, ny)) {
                    continue;
                }

                let mut nreq = req;
                if nc.is_ascii_uppercase() {
                    nreq |= 1 << (nc.to_ascii_lowercase() as u8 - b'a');
                }

                visited.insert((nx, ny));
                queue.push_back(((nx, ny), dist + 1, nreq));
            }
        }
        adj.insert(start_char, distances);
    }

    // Dijkstra
    #[derive(Clone, Eq, PartialEq, Hash)]
    struct State {
        cost: u32,
        positions: Vec<char>,
        mask: u32,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut dists = HashMap::new();
    let mut pq = BinaryHeap::new();

    let initial_positions = start_chars;
    pq.push(State {
        cost: 0,
        positions: initial_positions.clone(),
        mask: 0,
    });
    dists.insert((initial_positions, 0u32), 0u32);

    while let Some(State {
        cost,
        positions,
        mask,
    }) = pq.pop()
    {
        if mask == target_mask {
            return Ok(cost as u64);
        }

        if cost > *dists.get(&(positions.clone(), mask)).unwrap_or(&u32::MAX) {
            continue;
        }

        for (i, &p) in positions.iter().enumerate() {
            if let Some(neighbors) = adj.get(&p) {
                for (&next_char, &(d, req)) in neighbors {
                    if (mask & (1 << (next_char as u8 - b'a'))) == 0 && (mask & req) == req {
                        let next_mask = mask | (1 << (next_char as u8 - b'a'));
                        let next_dist = cost + d;
                        let mut next_positions = positions.clone();
                        next_positions[i] = next_char;

                        if next_dist
                            < *dists
                                .get(&(next_positions.clone(), next_mask))
                                .unwrap_or(&u32::MAX)
                        {
                            dists.insert((next_positions.clone(), next_mask), next_dist);
                            pq.push(State {
                                cost: next_dist,
                                positions: next_positions,
                                mask: next_mask,
                            });
                        }
                    }
                }
            }
        }
    }

    Err(anyhow::anyhow!("No path found to collect all keys"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "#########
#b.A.@.a#
#########";
        assert_eq!(solve_part1(input)?, 8);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(solve_part1(input)?, 86);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(solve_part1(input)?, 132);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(solve_part1(input)?, 136);
        Ok(())
    }

    #[test]
    fn test_example_5() -> Result<()> {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(solve_part1(input)?, 81);
        Ok(())
    }

    #[test]
    fn test_part2_example_1() -> Result<()> {
        let input = "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######";
        assert_eq!(solve_generic(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>())?, 8);
        Ok(())
    }

    #[test]
    fn test_part2_example_2() -> Result<()> {
        let input = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
        assert_eq!(solve_generic(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>())?, 24);
        Ok(())
    }

    #[test]
    fn test_part2_example_3() -> Result<()> {
        let input = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";
        assert_eq!(solve_generic(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>())?, 32);
        Ok(())
    }

    #[test]
    fn test_part2_example_4() -> Result<()> {
        let input = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";
        assert_eq!(solve_generic(&input.lines().map(|l| l.chars().collect()).collect::<Vec<Vec<char>>>())?, 72);
        Ok(())
    }
}
