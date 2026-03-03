pub mod utils;
pub(crate) mod year_2015;
pub(crate) mod year_2019;
pub(crate) mod year_2024;
pub(crate) mod year_2025;

pub use crate::utils::run_day;
pub use crate::utils::SolveFn;
pub use crate::utils::{Day, Year};

/// Top-level dispatcher to resolve a solver for a specific year and day.
#[must_use]
pub fn get_year_solver(year: u16, day: u8) -> Option<SolveFn> {
    match year {
        2015 => year_2015::get_solver(day),
        2019 => year_2019::get_solver(day),
        2024 => year_2024::get_solver(day),
        2025 => year_2025::get_solver(day),
        _ => None,
    }
}
