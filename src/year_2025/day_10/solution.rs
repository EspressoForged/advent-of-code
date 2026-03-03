use crate::utils::parser::{parse_str_lines, unsigned_number, Parse};
use crate::utils::{read_input, Year, Day};
use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::complete::{one_of, space0},
    multi::many1,
    multi::separated_list1,
    sequence::{delimited, preceded},
    IResult, Parser,
};

/// Represents a single machine's configuration.
#[derive(Debug)]
pub struct Machine {
    pub light_target: Vec<bool>,
    pub joltage_target: Vec<i64>,
    pub buttons: Vec<Vec<usize>>,
}

impl Parse for Machine {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, light_target) = delimited(tag("["), many1(one_of(".#")), tag("]"))
            .map(|chars: Vec<char>| chars.into_iter().map(|c| c == '#').collect::<Vec<bool>>())
            .parse(input)?;

        let (input, buttons) = many1(preceded(
            space0,
            delimited(
                tag("("),
                separated_list1(tag(","), unsigned_number),
                tag(")"),
            ),
        ))
        .parse(input)?;

        let (input, joltage_target) = preceded(
            space0,
            delimited(
                tag("{"),
                separated_list1(tag(","), unsigned_number),
                tag("}"),
            ),
        )
        .parse(input)?;

        Ok((
            input,
            Machine {
                light_target,
                buttons,
                joltage_target,
            },
        ))
    }
}

/// Solves Part 1 (Lights) using Gaussian Elimination over GF(2).
fn solve_part1(machine: &Machine) -> Option<usize> {
    let num_lights = machine.light_target.len();
    let num_buttons = machine.buttons.len();

    if num_buttons > 63 {
        return None;
    }

    let mut matrix = vec![0u64; num_lights];
    for (r, row) in matrix.iter_mut().enumerate().take(num_lights) {
        for (c, btn) in machine.buttons.iter().enumerate() {
            if btn.contains(&r) {
                *row |= 1 << c;
            }
        }
        if machine.light_target[r] {
            *row |= 1 << num_buttons;
        }
    }

    let mut pivot_row = 0;
    let mut pivot_cols = vec![None; num_buttons];

    for (col, pivot) in pivot_cols.iter_mut().enumerate().take(num_buttons) {
        if pivot_row >= num_lights {
            break;
        }
        let mut candidate = None;
        for (r, row) in matrix.iter().enumerate().take(num_lights).skip(pivot_row) {
            if (row >> col) & 1 == 1 {
                candidate = Some(r);
                break;
            }
        }
        if let Some(r) = candidate {
            matrix.swap(pivot_row, r);
            let pivot_val = matrix[pivot_row];
            for (i, row) in matrix.iter_mut().enumerate().take(num_lights) {
                if i != pivot_row && (*row >> col) & 1 == 1 {
                    *row ^= pivot_val;
                }
            }
            *pivot = Some(pivot_row);
            pivot_row += 1;
        }
    }

    for row in matrix.iter().take(num_lights).skip(pivot_row) {
        if (row >> num_buttons) & 1 == 1 {
            return None;
        }
    }

    let mut free_vars = Vec::new();
    for (c, pivot) in pivot_cols.iter().enumerate().take(num_buttons) {
        if pivot.is_none() {
            free_vars.push(c);
        }
    }

    let combinations = 1 << free_vars.len();
    let mut min_presses = usize::MAX;

    for i in 0..combinations {
        let mut current_presses = 0;
        let mut solution = vec![0; num_buttons];
        for (bit_idx, &var_idx) in free_vars.iter().enumerate() {
            let val = (i >> bit_idx) & 1;
            solution[var_idx] = val;
            current_presses += val;
        }
        for c in (0..num_buttons).rev() {
            if let Some(r) = pivot_cols[c] {
                let row_val = matrix[r];
                let target = (row_val >> num_buttons) & 1;
                let mut sum = 0;
                for (f, &sol_val) in solution.iter().enumerate().take(num_buttons).skip(c + 1) {
                    if (row_val >> f) & 1 == 1 {
                        sum ^= sol_val;
                    }
                }
                let val = target ^ sum;
                solution[c] = val;
                current_presses += val;
            }
        }
        if current_presses < min_presses as u64 {
            min_presses = current_presses as usize;
        }
    }

    if min_presses == usize::MAX {
        None
    } else {
        Some(min_presses)
    }
}

/// Parameters for the recursive search in Part 2.
struct SearchContext<'a> {
    free_vars: &'a [usize],
    bounds: &'a [i64],
    pivot_cols: &'a [Option<usize>],
    matrix: &'a [Vec<f64>],
    num_buttons: usize,
}

/// Solves Part 2 (Joltage) using Integer Linear Programming.
fn solve_part2(machine: &Machine) -> Option<i64> {
    let num_counters = machine.joltage_target.len();
    let num_buttons = machine.buttons.len();

    let mut matrix = vec![vec![0.0; num_buttons + 1]; num_counters];

    for (c, btn) in machine.buttons.iter().enumerate() {
        for &r in btn {
            if r < num_counters {
                matrix[r][c] = 1.0;
            }
        }
    }
    for (r, row) in matrix.iter_mut().enumerate().take(num_counters) {
        row[num_buttons] = machine.joltage_target[r] as f64;
    }

    let mut pivot_row = 0;
    let mut pivot_cols = vec![None; num_buttons];

    for (col, pivot) in pivot_cols.iter_mut().enumerate().take(num_buttons) {
        if pivot_row >= num_counters {
            break;
        }
        let mut candidate = None;
        for (r, row) in matrix.iter().enumerate().take(num_counters).skip(pivot_row) {
            if row[col].abs() > 1e-9 {
                candidate = Some(r);
                break;
            }
        }

        if let Some(r) = candidate {
            matrix.swap(pivot_row, r);
            let pivot_val = matrix[pivot_row][col];
            for val in matrix[pivot_row].iter_mut().take(num_buttons + 1).skip(col) {
                *val /= pivot_val;
            }

            // Clone pivot row to avoid double-borrow of matrix
            let pivot_row_data = matrix[pivot_row].clone();

            for (i, row) in matrix.iter_mut().enumerate().take(num_counters) {
                if i != pivot_row {
                    let factor = row[col];
                    if factor.abs() > 1e-9 {
                        for (j, val) in row.iter_mut().enumerate().take(num_buttons + 1).skip(col) {
                            *val -= factor * pivot_row_data[j];
                        }
                    }
                }
            }
            *pivot = Some(pivot_row);
            pivot_row += 1;
        }
    }

    for row in matrix.iter().take(num_counters).skip(pivot_row) {
        if row[num_buttons].abs() > 1e-9 {
            return None;
        }
    }

    let mut free_vars = Vec::new();
    for (c, pivot) in pivot_cols.iter().enumerate().take(num_buttons) {
        if pivot.is_none() {
            free_vars.push(c);
        }
    }

    let mut bounds = vec![i64::MAX; num_buttons];
    for (c, btn) in machine.buttons.iter().enumerate() {
        for &r in btn {
            if r < num_counters {
                bounds[c] = bounds[c].min(machine.joltage_target[r]);
            }
        }
    }
    for val in bounds.iter_mut().take(num_buttons) {
        if *val == i64::MAX {
            *val = 0;
        }
    }

    let mut best_total = i64::MAX;

    fn search(free_idx: usize, solution: &mut [i64], ctx: &SearchContext, best_total: &mut i64) {
        if free_idx == ctx.free_vars.len() {
            let mut current_sum = 0;
            for &fv in ctx.free_vars {
                current_sum += solution[fv];
            }
            for (c, pivot) in ctx
                .pivot_cols
                .iter()
                .enumerate()
                .take(ctx.num_buttons)
                .rev()
            {
                if let Some(r) = pivot {
                    let mut val = ctx.matrix[*r][ctx.num_buttons];
                    for (f, &sol_val) in solution
                        .iter()
                        .enumerate()
                        .take(ctx.num_buttons)
                        .skip(c + 1)
                    {
                        val -= ctx.matrix[*r][f] * (sol_val as f64);
                    }
                    let int_val = val.round() as i64;
                    if (val - int_val as f64).abs() > 1e-5 || int_val < 0 {
                        return;
                    }
                    solution[c] = int_val;
                    current_sum += int_val;
                }
            }
            if current_sum < *best_total {
                *best_total = current_sum;
            }
            return;
        }
        let var_idx = ctx.free_vars[free_idx];
        let bound = ctx.bounds[var_idx];
        for val in 0..=bound {
            solution[var_idx] = val;
            search(free_idx + 1, solution, ctx, best_total);
        }
    }

    let mut solution = vec![0; num_buttons];
    let ctx = SearchContext {
        free_vars: &free_vars,
        bounds: &bounds,
        pivot_cols: &pivot_cols,
        matrix: &matrix,
        num_buttons,
    };
    search(0, &mut solution, &ctx, &mut best_total);

    if best_total == i64::MAX {
        None
    } else {
        Some(best_total)
    }
}

/// Contains the core logic for the day's puzzle.
fn calculate_solution(machines: &[Machine]) -> Result<(u64, u64)> {
    let mut p1_total = 0;
    let mut p2_total = 0;

    for machine in machines {
        if let Some(p) = solve_part1(machine) {
            p1_total += p;
        }
        if let Some(p) = solve_part2(machine) {
            p2_total += p;
        }
    }
    Ok((p1_total as u64, p2_total as u64))
}

/// Entry point for Day 10
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2025), Day(10))?;
    let machines = parse_str_lines(&content, Machine::parse)?;
    calculate_solution(&machines)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}";

    #[test]
    fn test_day_10_solution() -> Result<()> {
        let machines = parse_str_lines(TEST_INPUT, Machine::parse)?;
        let (p1, p2) = calculate_solution(&machines)?;
        assert_eq!(p1, 2);
        assert_eq!(p2, 10);
        Ok(())
    }
}

