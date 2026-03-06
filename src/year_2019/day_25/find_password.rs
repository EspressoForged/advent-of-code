// use advent_of_code::year_2019::intcode::{Intcode, Status};
// use anyhow::Result;
// use std::fs;

// /// A standalone utility to brute-force the Day 25 password using the project's Intcode module.
// /// This matches the logic from the find_password.rs snippet but uses standard infrastructure.
// fn main() -> Result<()> {
//     let input = fs::read_to_string("inputs/year_2019/day_25.txt")?;
//     let program: Vec<i64> = input
//         .trim()
//         .split(',')
//         .map(|s| s.parse().unwrap())
//         .collect();

//     let mut vm = Intcode::new(program);

//     // Walkthrough to collect all 8 safe items and reach the Security Checkpoint.
//     let walkthrough = [
//         "north",
//         "take mouse",
//         "north",
//         "take pointer",
//         "south",
//         "south",
//         "west",
//         "take monolith",
//         "north",
//         "west",
//         "take food ration",
//         "south",
//         "take space law space brochure",
//         "north",
//         "east",
//         "south",
//         "south",
//         "take sand",
//         "south",
//         "west",
//         "take asterisk",
//         "south",
//         "take mutex",
//         "north",
//         "east",
//         "north",
//         "north",
//         "east",
//         "south",
//         "south",
//         "west",
//         "south",
//     ];

//     for cmd in walkthrough {
//         run_command(&mut vm, cmd)?;
//     }

//     let safe_items = [
//         "mouse",
//         "pointer",
//         "monolith",
//         "food ration",
//         "space law space brochure",
//         "sand",
//         "asterisk",
//         "mutex",
//     ];

//     println!("Starting brute force at Security Checkpoint...");

//     // We start holding all 8 items (mask 255).
//     let mut current_mask: u32 = 255;
//     let n = safe_items.len();

//     for next_mask in 0u32..(1 << n) {
//         // Apply differential inventory changes.
//         for (i, item) in safe_items.iter().enumerate() {
//             let was_held = (current_mask >> i) & 1 == 1;
//             let should_hold = (next_mask >> i) & 1 == 1;

//             if should_hold && !was_held {
//                 run_command(&mut vm, &format!("take {}", item))?;
//             } else if !should_hold && was_held {
//                 run_command(&mut vm, &format!("drop {}", item))?;
//             }
//         }
//         current_mask = next_mask;

//         // Test the weight.
//         let output = run_command(&mut vm, "east")?;
//         if !output.contains("Alert!") {
//             println!("--- SUCCESS ---");
//             let mut held_items = Vec::new();
//             for (i, item) in safe_items.iter().enumerate() {
//                 if (next_mask >> i) & 1 == 1 {
//                     held_items.push(*item);
//                 }
//             }
//             println!("Winning items: {:?}", held_items);
//             println!("{}", output);
//             return Ok(());
//         }
//     }

//     println!("Failed to find password.");
//     Ok(())
// }

// fn run_command(vm: &mut Intcode, cmd: &str) -> Result<String> {
//     for c in cmd.chars() {
//         vm.add_input(c as i64);
//     }
//     vm.add_input(10); // \n

//     let mut buffer = String::new();
//     loop {
//         match vm.step()? {
//             Status::Output(val) => {
//                 if val < 256 {
//                     buffer.push(val as u8 as char);
//                 }
//             }
//             Status::NeedsInput | Status::Halted => break,
//         }
//     }
//     Ok(buffer)
// }
