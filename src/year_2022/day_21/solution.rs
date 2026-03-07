use std::collections::HashMap;
use anyhow::{Result, anyhow};
use crate::utils::{read_input, Year, Day};

#[derive(Debug, Clone)]
enum Job {
    Literal(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

fn parse(input: &str) -> HashMap<String, Job> {
    input.lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let name = parts[0].to_string();
            let job_str = parts[1];

            let job = if let Ok(num) = job_str.parse::<i64>() {
                Job::Literal(num)
            } else {
                let job_parts: Vec<&str> = job_str.split_whitespace().collect();
                let left = job_parts[0].to_string();
                let op = job_parts[1];
                let right = job_parts[2].to_string();
                match op {
                    "+" => Job::Add(left, right),
                    "-" => Job::Sub(left, right),
                    "*" => Job::Mul(left, right),
                    "/" => Job::Div(left, right),
                    _ => panic!("Unknown operator: {}", op),
                }
            };
            (name, job)
        })
        .collect()
}

fn evaluate(name: &str, monkeys: &HashMap<String, Job>) -> i64 {
    match &monkeys[name] {
        Job::Literal(n) => *n,
        Job::Add(l, r) => evaluate(l, monkeys) + evaluate(r, monkeys),
        Job::Sub(l, r) => evaluate(l, monkeys) - evaluate(r, monkeys),
        Job::Mul(l, r) => evaluate(l, monkeys) * evaluate(r, monkeys),
        Job::Div(l, r) => evaluate(l, monkeys) / evaluate(r, monkeys),
    }
}

fn depends_on_humn(name: &str, monkeys: &HashMap<String, Job>) -> bool {
    if name == "humn" {
        return true;
    }
    match &monkeys[name] {
        Job::Literal(_) => false,
        Job::Add(l, r) | Job::Sub(l, r) | Job::Mul(l, r) | Job::Div(l, r) => {
            depends_on_humn(l, monkeys) || depends_on_humn(r, monkeys)
        }
    }
}

fn solve_for(name: &str, target: i64, monkeys: &HashMap<String, Job>) -> i64 {
    if name == "humn" {
        return target;
    }

    match &monkeys[name] {
        Job::Literal(_) => panic!("Reached literal while solving for humn"),
        Job::Add(l, r) => {
            if depends_on_humn(l, monkeys) {
                // target = L + R => L = target - R
                solve_for(l, target - evaluate(r, monkeys), monkeys)
            } else {
                // target = L + R => R = target - L
                solve_for(r, target - evaluate(l, monkeys), monkeys)
            }
        }
        Job::Sub(l, r) => {
            if depends_on_humn(l, monkeys) {
                // target = L - R => L = target + R
                solve_for(l, target + evaluate(r, monkeys), monkeys)
            } else {
                // target = L - R => R = L - target
                solve_for(r, evaluate(l, monkeys) - target, monkeys)
            }
        }
        Job::Mul(l, r) => {
            if depends_on_humn(l, monkeys) {
                // target = L * R => L = target / R
                solve_for(l, target / evaluate(r, monkeys), monkeys)
            } else {
                // target = L * R => R = target / L
                solve_for(r, target / evaluate(l, monkeys), monkeys)
            }
        }
        Job::Div(l, r) => {
            if depends_on_humn(l, monkeys) {
                // target = L / R => L = target * R
                solve_for(l, target * evaluate(r, monkeys), monkeys)
            } else {
                // target = L / R => R = L / target
                solve_for(r, evaluate(l, monkeys) / target, monkeys)
            }
        }
    }
}

pub fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let monkeys = parse(input);

    let part1 = evaluate("root", &monkeys);

    let root_job = monkeys.get("root").ok_or_else(|| anyhow!("Missing root monkey"))?;
    let (l, r) = match root_job {
        Job::Add(l, r) | Job::Sub(l, r) | Job::Mul(l, r) | Job::Div(l, r) => (l, r),
        Job::Literal(_) => return Err(anyhow!("Root monkey has literal job")),
    };

    let part2 = if depends_on_humn(l, &monkeys) {
        solve_for(l, evaluate(r, &monkeys), &monkeys)
    } else {
        solve_for(r, evaluate(l, &monkeys), &monkeys)
    };

    Ok((part1 as u64, part2 as u64))
}

/// Solve year 2022 day 21.
/// # Errors
/// Returns an error if root monkey is missing or if input cannot be read.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2022), Day(21))?;
    calculate_solution(&input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_example() {
        let (p1, p2) = calculate_solution(EXAMPLE).unwrap();
        assert_eq!(p1, 152);
        assert_eq!(p2, 301);
    }
}
