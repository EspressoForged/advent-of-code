use crate::utils::run_day;
use anyhow::Result;

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;

pub fn solution() -> Result<()> {
    println!("Year 2025");

    run_day(1, day_01::solution::solve)?;
    run_day(2, day_02::solution::solve)?;
    run_day(3, day_03::solution::solve)?;
    run_day(4, day_04::solution::solve)?;
    run_day(5, day_05::solution::solve)?;
    run_day(6, day_06::solution::solve)?;
    run_day(7, day_07::solution::solve)?;
    run_day(8, day_08::solution::solve)?;
    run_day(9, day_09::solution::solve)?;
    run_day(10, day_10::solution::solve)?;
    run_day(11, day_11::solution::solve)?;
    run_day(12, day_12::solution::solve)?;

    Ok(())
}
