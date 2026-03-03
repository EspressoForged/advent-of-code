use crate::utils::parser::{parse_str_lines, unsigned_number, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{bytes::complete::tag, IResult};
use std::collections::HashMap;

/// Represents a 3D coordinate.
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Parse for Point {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, x) = unsigned_number(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, y) = unsigned_number(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, z) = unsigned_number(input)?;

        Ok((input, Point { x, y, z }))
    }
}

/// Represents a connection between two junction boxes with a calculated distance.
#[derive(Debug, Clone, Copy)]
struct Connection {
    u: usize,
    v: usize,
    dist_sq: u64,
}

/// Disjoint Set Union (DSU) structure to manage connected components.
struct Dsu {
    parent: Vec<usize>,
    num_components: usize,
}

impl Dsu {
    fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            num_components: size,
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]); // Path compression
        }
        self.parent[i]
    }

    /// Unions sets containing i and j. Returns true if they were different sets.
    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i != root_j {
            self.parent[root_i] = root_j;
            self.num_components -= 1;
            true
        } else {
            false
        }
    }
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(points: &[Point]) -> Result<(u64, u64)> {
    let n = points.len();
    if n == 0 {
        return Ok((0, 0));
    }

    // 2. Generate all pairs and calculate squared distances
    let mut connections = Vec::with_capacity(n * (n - 1) / 2);
    for i in 0..n {
        for j in (i + 1)..n {
            let dx = points[i].x - points[j].x;
            let dy = points[i].y - points[j].y;
            let dz = points[i].z - points[j].z;
            let dist_sq = (dx * dx + dy * dy + dz * dz) as u64;

            connections.push(Connection {
                u: i,
                v: j,
                dist_sq,
            });
        }
    }

    // 3. Sort by distance (ascending)
    connections.sort_unstable_by(|a, b| a.dist_sq.cmp(&b.dist_sq));

    // --- Part 1 Logic: Simulate fixed number of connections (1000) ---
    let p1_total: u64 = {
        let limit = 1000.min(connections.len());
        let mut dsu_p1 = Dsu::new(n);

        for conn in connections.iter().take(limit) {
            dsu_p1.union(conn.u, conn.v);
        }

        let mut circuit_sizes: HashMap<usize, u64> = HashMap::new();
        for i in 0..n {
            let root = dsu_p1.find(i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }

        let mut sizes: Vec<u64> = circuit_sizes.values().cloned().collect();
        sizes.sort_unstable_by(|a, b| b.cmp(a)); // Descending order

        sizes.iter().take(3).product::<u64>()
    };

    // --- Part 2 Logic: Continue until fully connected ---
    let p2_total: u64 = {
        let mut dsu_p2 = Dsu::new(n);
        let mut last_connection: Option<&Connection> = None;

        for conn in &connections {
            if dsu_p2.union(conn.u, conn.v) && dsu_p2.num_components == 1 {
                last_connection = Some(conn);
                break;
            }
        }

        match last_connection {
            Some(conn) => {
                let p1 = points[conn.u];
                let p2 = points[conn.v];
                (p1.x * p2.x) as u64
            }
            None => 0,
        }
    };

    Ok((p1_total, p2_total))
}

/// Entry point for Day 08
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2025), Day(8))?;
    let points = parse_str_lines(&content, Point::parse)?;
    calculate_solution(&points)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "162,817,812\n57,618,57\n906,360,560\n592,479,940\n352,342,300\n466,668,158\n542,29,236\n431,825,988\n739,650,466\n52,470,668\n216,146,977\n819,987,18\n117,168,530\n805,96,715\n346,949,466\n970,615,88\n941,993,340\n862,61,35\n984,92,344\n425,690,689";

    #[test]
    fn test_day_08_solution() -> Result<()> {
        let points = parse_str_lines(TEST_INPUT, Point::parse)?;
        let (p1, p2) = calculate_solution(&points)?;
        assert_eq!(p1, 20);
        assert_eq!(p2, 25272);
        Ok(())
    }
}

