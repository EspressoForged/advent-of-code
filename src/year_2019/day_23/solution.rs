use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};
use std::collections::VecDeque;

/// Solves Year 2019, Day 23: Category Six.
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(23))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let part1 = solve_part1(&program)?;
    let part2 = solve_part2(&program)?;

    Ok((part1, part2))
}

fn solve_part1(program: &[i64]) -> Result<u64> {
    let mut vms: Vec<Intcode> = (0..50)
        .map(|i| {
            let mut vm = Intcode::new(program.to_vec());
            vm.add_input(i as i64);
            vm
        })
        .collect();

    let mut queues: Vec<VecDeque<(i64, i64)>> = vec![VecDeque::new(); 50];

    loop {
        for i in 0..50 {
            // Process any pending inputs
            if let Some((x, y)) = queues[i].pop_front() {
                vms[i].add_input(x);
                vms[i].add_input(y);
            } else {
                vms[i].add_input(-1);
            }

            // Run until it needs input or produces output
            loop {
                match vms[i].step()? {
                    Status::Output(_) => {
                        if vms[i].output.len() >= 3 {
                            let dest = vms[i].output.pop_front().unwrap();
                            let x = vms[i].output.pop_front().unwrap();
                            let y = vms[i].output.pop_front().unwrap();

                            if dest == 255 {
                                return Ok(y as u64);
                            }

                            if (0..50).contains(&dest) {
                                queues[dest as usize].push_back((x, y));
                            }
                        }
                    }
                    Status::NeedsInput => break,
                    Status::Halted => break,
                }
            }
        }
    }
}

fn solve_part2(program: &[i64]) -> Result<u64> {
    let mut vms: Vec<Intcode> = (0..50)
        .map(|i| {
            let mut vm = Intcode::new(program.to_vec());
            vm.add_input(i as i64);
            vm
        })
        .collect();

    let mut queues: Vec<VecDeque<(i64, i64)>> = vec![VecDeque::new(); 50];
    let mut nat_packet: Option<(i64, i64)> = None;
    let mut last_nat_y: Option<i64> = None;

    loop {
        let mut idle = true;
        for i in 0..50 {
            // Process any pending inputs
            if let Some((x, y)) = queues[i].pop_front() {
                idle = false;
                vms[i].add_input(x);
                vms[i].add_input(y);
            } else {
                vms[i].add_input(-1);
            }

            // Run until it needs input
            loop {
                match vms[i].step()? {
                    Status::Output(_) => {
                        if vms[i].output.len() >= 3 {
                            idle = false;
                            let dest = vms[i].output.pop_front().unwrap();
                            let x = vms[i].output.pop_front().unwrap();
                            let y = vms[i].output.pop_front().unwrap();

                            if dest == 255 {
                                nat_packet = Some((x, y));
                            } else if (0..50).contains(&dest) {
                                queues[dest as usize].push_back((x, y));
                            }
                        }
                    }
                    Status::NeedsInput => break,
                    Status::Halted => break,
                }
            }
        }

        if idle && queues.iter().all(VecDeque::is_empty) {
            if let Some((x, y)) = nat_packet {
                if last_nat_y == Some(y) {
                    return Ok(y as u64);
                }
                last_nat_y = Some(y);
                queues[0].push_back((x, y));
            }
        }
    }
}
