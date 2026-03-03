use advent_of_code::{Day, Year};
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Year to run (e.g. 2024)
    #[arg(short, long)]
    year: Option<u16>,

    /// Day to run (1-25)
    #[arg(short, long)]
    day: Option<u8>,

    /// Run all implemented puzzles
    #[arg(short, long)]
    all: bool,

    /// Run in parallel
    #[arg(short, long)]
    parallel: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a new day's scaffolding
    Generate {
        /// Year (2015-2025)
        #[arg(short, long)]
        year: u16,
        /// Day (1-25)
        #[arg(short, long)]
        day: u8,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Some(command) = cli.command {
        match command {
            Commands::Generate { year, day } => return generate_day(year, day),
        }
    }

    if cli.all {
        run_all(cli.parallel)?;
    } else {
        match (cli.year, cli.day) {
            (Some(year), Some(day)) => {
                let solver = advent_of_code::get_year_solver(year, day)
                    .ok_or_else(|| anyhow!("Year {}, Day {} not implemented", year, day))?;
                advent_of_code::run_day(Year(year), Day(day), solver)?;
            }
            (Some(year), None) => {
                run_year(year, cli.parallel)?;
            }
            (None, _) => {
                return Err(anyhow!(
                    "Please specify a year (-y), run all (-a), or use a subcommand"
                ));
            }
        }
    }

    Ok(())
}

fn run_year(year: u16, parallel: bool) -> Result<()> {
    let mut solvers = Vec::new();
    for day in 1..=25 {
        if let Some(solver) = advent_of_code::get_year_solver(year, day) {
            solvers.push((day, solver));
        }
    }

    if solvers.is_empty() {
        println!("No implemented days found for year {}", year);
        return Ok(());
    }

    if parallel {
        use rayon::prelude::*;
        solvers
            .into_par_iter()
            .try_for_each(|(day, solver)| advent_of_code::run_day(Year(year), Day(day), solver))?;
    } else {
        for (day, solver) in solvers {
            advent_of_code::run_day(Year(year), Day(day), solver)?;
        }
    }
    Ok(())
}

fn run_all(parallel: bool) -> Result<()> {
    for year in 2015..=2025 {
        run_year(year, parallel)?;
    }
    Ok(())
}

fn generate_day(year: u16, day: u8) -> Result<()> {
    let year_dir = format!("src/year_{}", year);
    let day_dir = format!("{}/day_{:02}", year_dir, day);
    let input_dir = format!("inputs/year_{}", year);

    // Create directories
    fs::create_dir_all(&day_dir)?;
    fs::create_dir_all(&input_dir)?;

    // Create day_XX.txt
    let input_path = format!("{}/day_{:02}.txt", input_dir, day);
    if !Path::new(&input_path).exists() {
        fs::write(&input_path, "")?;
        println!("Created {}", input_path);
    }

    // Create day_XX/mod.rs
    let mod_path = format!("{}/mod.rs", day_dir);
    if !Path::new(&mod_path).exists() {
        fs::write(&mod_path, "pub mod solution;\npub use solution::solve;\n")?;
        println!("Created {}", mod_path);
    }

    // Create day_XX/solution.rs
    let sol_path = format!("{}/solution.rs", day_dir);
    if !Path::new(&sol_path).exists() {
        let template = format!(
            r#"use crate::utils::{{read_input, Year, Day}};
use anyhow::Result;

/// Core logic for Year {year}, Day {day:02}
fn calculate_solution(input: &str) -> Result<(u64, u64)> {{
    // TODO: Implement solution
    Ok((0, 0))
}}

pub fn solve() -> Result<(u64, u64)> {{
    let content = read_input(Year({year}), Day({day}))?;
    calculate_solution(&content)
}}

#[cfg(test)]
mod tests {{
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_day_{day:02}_solution() -> Result<()> {{
        let (p1, p2) = calculate_solution(TEST_INPUT)?;
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
        Ok(())
    }}
}}
"#
        );
        fs::write(&sol_path, template)?;
        println!("Created {}", sol_path);
    }

    // Check if year_XXXX/mod.rs exists, if not create it
    let year_mod_path = format!("{}/mod.rs", year_dir);
    if !Path::new(&year_mod_path).exists() {
        let year_mod_template = format!(
            "use crate::register_days;\n\nregister_days!(day_{:02});\n",
            day
        );
        fs::write(&year_mod_path, year_mod_template)?;
        println!("Created {}", year_mod_path);
    } else {
        println!("\nSUCCESS: Scaffolding created.");
        println!(
            "ACTION REQUIRED: Add `day_{:02}` to the `register_days!` macro in `{}`",
            day, year_mod_path
        );
    }

    Ok(())
}
