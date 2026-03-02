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

    // Create a list of all the solution functions
    let solutions: Vec<fn() -> Result<()>> = vec![
        day_01::solution::solve,
        day_02::solution::solve,
        day_03::solution::solve,
        day_04::solution::solve,
        day_05::solution::solve,
        day_06::solution::solve,
        day_07::solution::solve,
        day_08::solution::solve,
        day_09::solution::solve,
        day_10::solution::solve,
        day_11::solution::solve,
        day_12::solution::solve,
    ];

    // Iterate over the solutions, running each and printing any errors
    for (i, solve_fn) in solutions.iter().enumerate() {
        if let Err(e) = solve_fn() {
            eprintln!("Error running Day {:02}: {}", i + 1, e);
        }
    }

    Ok(())
}
