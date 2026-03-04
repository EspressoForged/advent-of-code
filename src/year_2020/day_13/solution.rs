use crate::utils::{read_input, Day, Year};
use anyhow::Result;

/// Solves Year 2020, Day 13: Shuttle Search.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2020), Day(13))?;
    let (p1, p2) = calculate_solution(&content)?;
    Ok((p1, p2))
}

fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let mut lines = input.lines();
    let earliest_time: u64 = lines.next().unwrap_or("0").parse()?;
    let bus_ids_raw = lines.next().unwrap_or("");

    let part1 = solve_part1(earliest_time, bus_ids_raw);
    let part2 = solve_part2(bus_ids_raw);

    Ok((part1, part2))
}

fn solve_part1(earliest_time: u64, bus_ids_raw: &str) -> u64 {
    let bus_ids: Vec<u64> = bus_ids_raw
        .split(',')
        .filter(|&x| x != "x")
        .map(|x| x.parse::<u64>().unwrap())
        .collect();

    let mut min_wait = u64::MAX;
    let mut best_bus = 0;

    for &id in &bus_ids {
        let wait_time = if earliest_time.is_multiple_of(id) {
            0
        } else {
            id - (earliest_time % id)
        };

        if wait_time < min_wait {
            min_wait = wait_time;
            best_bus = id;
        }
    }

    best_bus * min_wait
}

fn solve_part2(bus_ids_raw: &str) -> u64 {
    let buses: Vec<(u64, u64)> = bus_ids_raw
        .split(',')
        .enumerate()
        .filter(|&(_, x)| x != "x")
        .map(|(i, x)| (i as u64, x.parse::<u64>().unwrap()))
        .collect();

    let mut t = 0;
    let mut step = 1;

    for (offset, id) in buses {
        // Find the first t' of the form t + n * step such that (t' + offset) % id == 0
        while (t + offset) % id != 0 {
            t += step;
        }
        // Update step to be the LCM of current step and the new bus ID.
        // Since bus IDs are prime, we just multiply.
        step *= id;
    }

    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = "939\n7,13,x,x,59,x,31,19";
        let (p1, _) = calculate_solution(input).unwrap();
        assert_eq!(p1, 295);
    }

    #[test]
    fn test_example_part2_1() {
        assert_eq!(solve_part2("7,13,x,x,59,x,31,19"), 1068781);
    }

    #[test]
    fn test_example_part2_2() {
        assert_eq!(solve_part2("17,x,13,19"), 3417);
    }

    #[test]
    fn test_example_part2_3() {
        assert_eq!(solve_part2("67,7,59,61"), 754018);
    }

    #[test]
    fn test_example_part2_4() {
        assert_eq!(solve_part2("67,x,7,59,61"), 779210);
    }

    #[test]
    fn test_example_part2_5() {
        assert_eq!(solve_part2("67,7,x,59,61"), 1261476);
    }

    #[test]
    fn test_example_part2_6() {
        assert_eq!(solve_part2("1789,37,47,1889"), 1202161486);
    }
}
