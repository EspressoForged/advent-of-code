pub mod utils;
pub mod year_2024;
pub mod year_2025;

use crate::utils::SolveFn;

/// Top-level dispatcher to resolve a solver for a specific year and day.
pub fn get_year_solver(year: u16, day: u8) -> Option<SolveFn> {
    match year {
        2024 => year_2024::get_solver(day),
        2025 => year_2025::get_solver(day),
        _ => None,
    }
}
