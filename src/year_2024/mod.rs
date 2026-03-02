use crate::utils::run_day;
use anyhow::Result;

pub mod day_02;

pub fn solution() -> Result<()> {
    println!("Year 2024");

    run_day(2, day_02::solution::solve)?;

    Ok(())
}
