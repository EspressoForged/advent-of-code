use crate::utils::{read_input, Day, Year};
use anyhow::{anyhow, Result};
use std::collections::{HashSet, VecDeque};

/// Solves Year 2020, Day 22: Crab Combat.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let content = read_input(Year(2020), Day(22))?;
    let (p1, p2) = calculate_solution(&content)?;
    Ok((p1, p2))
}

fn parse_decks(input: &str) -> Result<(VecDeque<u32>, VecDeque<u32>)> {
    let normalized = input.replace("\r\n", "\n");
    let players: Vec<&str> = normalized.split("\n\n").collect();
    if players.len() != 2 {
        return Err(anyhow!("Invalid input: expected 2 players, found {}", players.len()));
    }

    let p1_deck = players[0]
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u32>().map_err(|e| anyhow!(e)))
        .collect::<Result<VecDeque<u32>>>()?;

    let p2_deck = players[1]
        .lines()
        .skip(1)
        .filter(|l| !l.is_empty())
        .map(|l| l.parse::<u32>().map_err(|e| anyhow!(e)))
        .collect::<Result<VecDeque<u32>>>()?;

    Ok((p1_deck, p2_deck))
}

fn calculate_score(deck: &VecDeque<u32>) -> u64 {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, &card)| (i as u64 + 1) * card as u64)
        .sum()
}

fn play_combat(mut p1: VecDeque<u32>, mut p2: VecDeque<u32>) -> u64 {
    while !p1.is_empty() && !p2.is_empty() {
        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        } else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }

    if p1.is_empty() {
        calculate_score(&p2)
    } else {
        calculate_score(&p1)
    }
}

enum Player {
    One,
    Two,
}

fn play_recursive_combat(
    mut p1: VecDeque<u32>,
    mut p2: VecDeque<u32>,
) -> (Player, VecDeque<u32>, VecDeque<u32>) {
    let mut history = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        let state = (p1.clone(), p2.clone());
        if !history.insert(state) {
            return (Player::One, p1, p2);
        }

        let c1 = p1.pop_front().unwrap();
        let c2 = p2.pop_front().unwrap();

        let winner = if p1.len() >= c1 as usize && p2.len() >= c2 as usize {
            let sub_p1 = p1.iter().take(c1 as usize).copied().collect();
            let sub_p2 = p2.iter().take(c2 as usize).copied().collect();
            let (sub_winner, _, _) = play_recursive_combat(sub_p1, sub_p2);
            sub_winner
        } else if c1 > c2 {
            Player::One
        } else {
            Player::Two
        };

        match winner {
            Player::One => {
                p1.push_back(c1);
                p1.push_back(c2);
            }
            Player::Two => {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }

    if p1.is_empty() {
        (Player::Two, p1, p2)
    } else {
        (Player::One, p1, p2)
    }
}

fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let (p1_deck, p2_deck) = parse_decks(input)?;

    let part1 = play_combat(p1_deck.clone(), p2_deck.clone());

    let (winner, p1_final, p2_final) = play_recursive_combat(p1_deck, p2_deck);
    let part2 = match winner {
        Player::One => calculate_score(&p1_final),
        Player::Two => calculate_score(&p2_final),
    };

    Ok((part1, part2))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_example_part1() -> Result<()> {
        let (p1, _) = calculate_solution(EXAMPLE)?;
        assert_eq!(p1, 306);
        Ok(())
    }

    #[test]
    fn test_example_part2() -> Result<()> {
        let (_, p2) = calculate_solution(EXAMPLE)?;
        assert_eq!(p2, 291);
        Ok(())
    }

    #[test]
    fn test_infinite_loop_prevention() -> Result<()> {
        let input = "Player 1:
43
19

Player 2:
2
29
14";
        let (p1_deck, p2_deck) = parse_decks(input)?;
        let (winner, _, _) = play_recursive_combat(p1_deck, p2_deck);
        assert!(matches!(winner, Player::One));
        Ok(())
    }
}
