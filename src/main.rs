use anyhow::{anyhow, Result};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Year to run (e.g. 2024)
    #[arg(short, long)]
    year: Option<u16>,

    /// Day to run (1-25)
    #[arg(short, long)]
    day: Option<u8>,

    /// Run all implemented puzzles
    #[arg(short, long)]
    all: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.all {
        run_all()?;
    } else {
        match (cli.year, cli.day) {
            (Some(year), Some(day)) => {
                let solver = advent_of_code::get_year_solver(year, day)
                    .ok_or_else(|| anyhow!("Year {}, Day {} not implemented", year, day))?;
                advent_of_code::utils::run_day(year, day, solver)?;
            }
            (Some(year), None) => {
                run_year(year)?;
            }
            (None, _) => {
                return Err(anyhow!("Please specify a year (-y) or run all (-a)"));
            }
        }
    }

    Ok(())
}

fn run_year(year: u16) -> Result<()> {
    let mut found = false;
    for day in 1..=25 {
        if let Some(solver) = advent_of_code::get_year_solver(year, day) {
            advent_of_code::utils::run_day(year, day, solver)?;
            found = true;
        }
    }
    if !found {
        println!("No implemented days found for year {}", year);
    }
    Ok(())
}

fn run_all() -> Result<()> {
    // We could dynamically iterate years 2015..=2025
    for year in 2015..=2025 {
        run_year(year)?;
    }
    Ok(())
}
