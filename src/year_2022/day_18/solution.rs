use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn neighbors(&self) -> [Point3D; 6] {
        [
            Point3D { x: self.x + 1, y: self.y, z: self.z },
            Point3D { x: self.x - 1, y: self.y, z: self.z },
            Point3D { x: self.x, y: self.y + 1, z: self.z },
            Point3D { x: self.x, y: self.y - 1, z: self.z },
            Point3D { x: self.x, y: self.y, z: self.z + 1 },
            Point3D { x: self.x, y: self.y, z: self.z - 1 },
        ]
    }
}

fn parse_input(input: &str) -> Result<HashSet<Point3D>> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let coords: Vec<i32> = line
            .split(',')
            .map(|s| s.parse::<i32>().map_err(|e| anyhow!("Failed to parse coordinate: {}", e)))
            .collect::<Result<Vec<i32>>>()?;
        
        if coords.len() != 3 {
            return Err(anyhow!("Invalid coordinate format: {}", line));
        }
        
        cubes.insert(Point3D {
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }
    Ok(cubes)
}

/// Solve for Year 2022, Day 18
pub fn solve() -> Result<(u64, u64)> {
    let input = crate::utils::read_input(crate::utils::Year(2022), crate::utils::Day(18))?;
    let cubes = parse_input(&input)?;
    
    let p1 = calculate_surface_area(&cubes);
    let p2 = calculate_exterior_surface_area(&cubes);
    
    Ok((p1 as u64, p2 as u64))
}

fn calculate_surface_area(cubes: &HashSet<Point3D>) -> usize {
    let mut surface_area = 0;
    for cube in cubes {
        for neighbor in cube.neighbors() {
            if !cubes.contains(&neighbor) {
                surface_area += 1;
            }
        }
    }
    surface_area
}

fn calculate_exterior_surface_area(cubes: &HashSet<Point3D>) -> usize {
    if cubes.is_empty() {
        return 0;
    }

    let min_x = cubes.iter().map(|p| p.x).min().unwrap() - 1;
    let max_x = cubes.iter().map(|p| p.x).max().unwrap() + 1;
    let min_y = cubes.iter().map(|p| p.y).min().unwrap() - 1;
    let max_y = cubes.iter().map(|p| p.y).max().unwrap() + 1;
    let min_z = cubes.iter().map(|p| p.z).min().unwrap() - 1;
    let max_z = cubes.iter().map(|p| p.z).max().unwrap() + 1;

    let mut exterior = HashSet::new();
    let mut queue = std::collections::VecDeque::new();
    let start = Point3D { x: min_x, y: min_y, z: min_z };
    
    exterior.insert(start);
    queue.push_back(start);

    while let Some(curr) = queue.pop_front() {
        for neighbor in curr.neighbors() {
            if neighbor.x >= min_x && neighbor.x <= max_x &&
               neighbor.y >= min_y && neighbor.y <= max_y &&
               neighbor.z >= min_z && neighbor.z <= max_z &&
               !cubes.contains(&neighbor) &&
               !exterior.contains(&neighbor) {
                exterior.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }

    let mut exterior_surface_area = 0;
    for cube in cubes {
        for neighbor in cube.neighbors() {
            if exterior.contains(&neighbor) {
                exterior_surface_area += 1;
            }
        }
    }
    exterior_surface_area
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_example() {
        let cubes = parse_input(EXAMPLE).unwrap();
        assert_eq!(calculate_surface_area(&cubes), 64);
        assert_eq!(calculate_exterior_surface_area(&cubes), 58);
    }

    #[test]
    fn test_regression() -> Result<()> {
        let (p1, p2) = solve()?;
        assert_eq!(p1, 4500);
        assert_eq!(p2, 2558);
        Ok(())
    }
}
