use crate::utils::{read_input, Day, Year};
use anyhow::Result;
use num::integer::gcd;
use std::collections::{BTreeMap, HashSet};

/// A coordinate in the asteroid belt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    #[must_use]
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

/// Solves Year 2019, Day 10: Monitoring Station.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(10))?;
    let (p1, p2) = calculate_solution(&input);
    Ok((p1, p2))
}

#[must_use]
fn calculate_solution(input: &str) -> (u64, u64) {
    let asteroids = parse_asteroids(input);
    let (best_pos, max_visible) = find_best_location(&asteroids);
    let p2_result = vaporize_asteroids(&asteroids, best_pos, 200);

    (max_visible as u64, p2_result as u64)
}

#[must_use]
fn parse_asteroids(input: &str) -> HashSet<Coord> {
    let mut asteroids = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                asteroids.insert(Coord::new(x as i32, y as i32));
            }
        }
    }
    asteroids
}

#[must_use]
fn get_visible_asteroids_count(origin: Coord, asteroids: &HashSet<Coord>) -> usize {
    let mut angles = HashSet::new();
    for &asteroid in asteroids {
        if asteroid == origin {
            continue;
        }
        let dx = asteroid.x - origin.x;
        let dy = asteroid.y - origin.y;
        let common = gcd(dx, dy);
        angles.insert((dx / common, dy / common));
    }
    angles.len()
}

#[must_use]
fn find_best_location(asteroids: &HashSet<Coord>) -> (Coord, usize) {
    let mut best_pos = Coord::new(0, 0);
    let mut max_visible = 0;

    for &origin in asteroids {
        let count = get_visible_asteroids_count(origin, asteroids);
        if count > max_visible {
            max_visible = count;
            best_pos = origin;
        }
    }

    (best_pos, max_visible)
}

type AngleMap = BTreeMap<(i32, i32), Vec<(i32, Coord)>>;

#[must_use]
fn vaporize_asteroids(asteroids: &HashSet<Coord>, origin: Coord, target_count: usize) -> i32 {
    if asteroids.len() <= 1 {
        return 0;
    }

    // Map by angle (reduced vector), each containing a list of (distance, coordinate)
    let mut map: AngleMap = BTreeMap::new();
    for &asteroid in asteroids {
        if asteroid == origin {
            continue;
        }
        let dx = asteroid.x - origin.x;
        let dy = asteroid.y - origin.y;
        let common = gcd(dx, dy);
        let key = (dx / common, dy / common);
        let dist_sq = dx * dx + dy * dy;
        map.entry(key).or_default().push((dist_sq, asteroid));
    }

    // Sort asteroids on the same line by distance
    for list in map.values_mut() {
        list.sort_by_key(|&(d, _)| d);
        list.reverse(); // So we can pop from the end
    }

    // Sort angles by laser rotation (starting up, clockwise)
    let mut sorted_keys: Vec<(i32, i32)> = map.keys().copied().collect();
    sorted_keys.sort_by(|a, b| {
        let angle_a = (a.0 as f64).atan2(-a.1 as f64);
        let angle_b = (b.0 as f64).atan2(-b.1 as f64);

        // Normalize atan2 result to [0, 2PI)
        let mut norm_a = angle_a;
        if norm_a < 0.0 {
            norm_a += 2.0 * std::f64::consts::PI;
        }
        let mut norm_b = angle_b;
        if norm_b < 0.0 {
            norm_b += 2.0 * std::f64::consts::PI;
        }

        norm_a.partial_cmp(&norm_b).unwrap()
    });

    let mut vaporized_count = 0;
    let mut last_vaporized = Coord::new(0, 0);

    while vaporized_count < target_count && !map.is_empty() {
        let mut keys_to_remove = Vec::new();
        for &key in &sorted_keys {
            if let Some(list) = map.get_mut(&key) {
                if let Some((_, pos)) = list.pop() {
                    vaporized_count += 1;
                    last_vaporized = pos;
                    if vaporized_count == target_count {
                        return last_vaporized.x * 100 + last_vaporized.y;
                    }
                }
                if list.is_empty() {
                    keys_to_remove.push(key);
                }
            }
        }
        for key in keys_to_remove {
            map.remove(&key);
        }
    }

    last_vaporized.x * 100 + last_vaporized.y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_small_example() {
        let input = ".#..#\n.....\n#####\n....#\n...##";
        let asteroids = parse_asteroids(input);
        let (pos, count) = find_best_location(&asteroids);
        assert_eq!(pos, Coord::new(3, 4));
        assert_eq!(count, 8);
    }

    #[test]
    fn test_large_example_210() {
        let input = ".#..##.###...#######\n##.############..##.\n.#.######.########.#\n.###.#######.####.#.\n#####.##.#.##.###.##\n..#####..#.#########\n####################\n#.####....###.#.#.##\n##.#################\n#####.##.###..####..\n..######..##.#######\n####.##.####...##..#\n.#####..#.######.###\n##...#.##########...\n#.##########.#######\n.####.#.###.###.#.##\n....##.##.###..#####\n.#.#.###########.###\n#.#.#.#####.####.###\n###.##.####.##.#..##";
        let asteroids = parse_asteroids(input);
        let (pos, count) = find_best_location(&asteroids);
        assert_eq!(pos, Coord::new(11, 13));
        assert_eq!(count, 210);

        let p2 = vaporize_asteroids(&asteroids, pos, 200);
        assert_eq!(p2, 802);
    }
}

