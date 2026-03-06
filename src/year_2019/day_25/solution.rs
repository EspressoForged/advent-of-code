use crate::utils::{read_input, Day, Year};
use crate::year_2019::intcode::{Intcode, Status};
use anyhow::{Context, Result};

/// Solves Year 2019, Day 25: Cryostasis.
///
/// ROOM DATA TABLE:
/// | Room Name                | Items Present              | Exits                |
/// | :---                     | :---                       | :---                 |
/// | Hull Breach (Start)      | -                          | north, south, west   |
/// | Arcade                   | infinite loop (TRAP)       | north, south         |
/// | Corridor                 | -                          | north, west          |
/// | Crew Quarters            | -                          | east                 |
/// | Engineering              | photons (TRAP)             | east, south          |
/// | Gift Wrapping Center     | giant electromagnet (TRAP) | south, west          |
/// | Hallway                  | mouse                      | north, east, south   |
/// | Holodeck                 | escape pod (TRAP)          | west                 |
/// | Hot Chocolate Fountain   | space law space brochure   | north                |
/// | Kitchen                  | pointer                    | east, south          |
/// | Navigation               | sand                       | north, south         |
/// | Observatory              | food ration                | east, south          |
/// | Passages                 | -                          | north                |
/// | Pressure-Sensitive Floor | -                          | west                 |
/// | Security Checkpoint      | -                          | north, east          |
/// | Science Lab              | -                          | north, west          |
/// | Sick Bay                 | molten lava (TRAP)         | south, west          |
/// | Stables                  | monolith                   | north, east, south   |
/// | Storage                  | mutex                      | north, west          |
/// | Warp Drive Maintenance   | asterisk                   | east, south          |
///
/// ADJACENCY MAP:
/// Hull Breach ↔ Hallway (North) / Arcade (South) / Stables (West)
/// Hallway ↔ Kitchen (North) / Holodeck (East)
/// Arcade ↔ Science Lab (South)
/// Science Lab ↔ Engineering (West)
/// Engineering ↔ Security Checkpoint (South)
/// Security Checkpoint → Pressure-Sensitive Floor (East)
/// Kitchen ↔ Gift Wrapping Center (East)
/// Gift Wrapping Center ↔ Passages (South)
/// Stables ↔ Sick Bay (North) / Navigation (South)
/// Navigation ↔ Corridor (South)
/// Corridor ↔ Warp Drive Maintenance (West)
/// Warp Drive Maintenance ↔ Storage (South)
/// Storage ↔ Crew Quarters (West)
/// Sick Bay ↔ Observatory (West)
/// Observatory ↔ Hot Chocolate Fountain (South)
///
/// # Errors
/// Returns an error if the input cannot be read or program execution fails.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(25))?;
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<i64>()
                .with_context(|| format!("Failed to parse program value: '{}'", s))
        })
        .collect::<Result<Vec<i64>>>()?;

    let password = solve_part1(&program)?;

    Ok((password, 0))
}

fn solve_part1(program: &[i64]) -> Result<u64> {
    let mut vm = Intcode::new(program.to_vec());

    // Phase 1: Walkthrough to collect all 8 safe items and reach the Security Checkpoint.
    // The sequence includes backtracking steps to ensure the VM state stays synchronized.
    let walkthrough = [
        "north",
        "take mouse",
        "north",
        "take pointer",
        "south",
        "south",
        "west",
        "take monolith",
        "north",
        "west",
        "take food ration",
        "south",
        "take space law space brochure",
        "north",
        "east",
        "south",
        "south",
        "take sand",
        "south",
        "west",
        "take asterisk",
        "south",
        "take mutex",
        "north",
        "east",
        "north",
        "north",
        "east",
        "south",
        "south",
        "west",
        "south",
    ];

    for cmd in walkthrough {
        run_command(&mut vm, cmd)?;
    }

    let safe_items = [
        "mouse",
        "pointer",
        "monolith",
        "food ration",
        "space law space brochure",
        "sand",
        "asterisk",
        "mutex",
    ];

    // Phase 2: Brute force the correct weight combination using Differential Inventory Management.
    // We start with all items (mask 255) and XOR the current mask with the next to minimize commands.
    let mut current_mask: u32 = 255;
    let n = safe_items.len();

    for next_mask in 0u32..(1 << n) {
        for (i, item) in safe_items.iter().enumerate() {
            let was_held = (current_mask >> i) & 1 == 1;
            let should_hold = (next_mask >> i) & 1 == 1;

            if should_hold && !was_held {
                run_command(&mut vm, &format!("take {}", item))?;
            } else if !should_hold && was_held {
                run_command(&mut vm, &format!("drop {}", item))?;
            }
        }
        current_mask = next_mask;

        // Moving 'east' from the Security Checkpoint tests the current weight.
        let output = run_command(&mut vm, "east")?;
        if !output.contains("Alert!") {
            // Parsing the numeric password from the success message.
            if let Some(pos) = output.find("password is ") {
                let start = pos + "password is ".len();
                let end = output[start..]
                    .find(|c: char| !c.is_ascii_digit())
                    .map(|i| start + i)
                    .unwrap_or(output.len());
                return Ok(output[start..end].parse()?);
            }
        }
    }

    Err(anyhow::anyhow!("Could not find airlock password"))
}

/// Helper to execute a command and return the VM's ASCII output.
fn run_command(vm: &mut Intcode, cmd: &str) -> Result<String> {
    for c in cmd.chars() {
        vm.add_input(c as i64);
    }
    vm.add_input(10); // Newline (ASCII 10)

    let mut buffer = String::new();
    while let Status::Output(val) = vm.step()? {
        if val < 256 {
            buffer.push(val as u8 as char);
        }
    }
    Ok(buffer)
}
