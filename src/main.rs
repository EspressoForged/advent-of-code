use anyhow::Result;

mod year_2025;

fn main() -> Result<()> {
    year_2025::solution()?;
    Ok(())
}
