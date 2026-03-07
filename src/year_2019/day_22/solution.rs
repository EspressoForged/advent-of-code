use crate::utils::{read_input, Day, Year};
use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i128 as nom_i128, multispace0},
    combinator::{map, value},
    multi::separated_list1,
    sequence::preceded,
    IResult, Parser,
};

/// Solves Year 2019, Day 22: Slam Shuffle.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2019), Day(22))?;
    calculate_solution(&input)
}

#[derive(Debug, Clone, Copy)]
enum Shuffle {
    DealIntoNewStack,
    Cut(i128),
    DealWithIncrement(i128),
}

fn parse_shuffle(input: &str) -> IResult<&str, Shuffle> {
    alt((
        value(Shuffle::DealIntoNewStack, tag("deal into new stack")),
        map(preceded(tag("cut "), nom_i128), Shuffle::Cut),
        map(
            preceded(tag("deal with increment "), nom_i128),
            Shuffle::DealWithIncrement,
        ),
    ))
    .parse(input)
}

fn parse_shuffles(input: &str) -> IResult<&str, Vec<Shuffle>> {
    separated_list1(multispace0, parse_shuffle).parse(input)
}

/// Core logic for Year 2019, Day 22.
fn calculate_solution(input: &str) -> Result<(u64, u64)> {
    let p1 = run_shuffle(input, 10007, 2019)?;
    let p2 = run_shuffle_part2(input, 119315717514047, 101741582076661, 2020)?;
    Ok((p1, p2))
}

fn run_shuffle(input: &str, deck_size: i128, card_to_track: i128) -> Result<u64> {
    let (_, shuffles) =
        parse_shuffles(input.trim()).map_err(|e| anyhow!("Failed to parse shuffles: {}", e))?;

    let mut pos = card_to_track;

    for shuffle in shuffles {
        match shuffle {
            Shuffle::DealIntoNewStack => {
                pos = (deck_size - 1 - pos) % deck_size;
            }
            Shuffle::Cut(n) => {
                pos = (pos - n) % deck_size;
                if pos < 0 {
                    pos += deck_size;
                }
            }
            Shuffle::DealWithIncrement(n) => {
                pos = (pos * n) % deck_size;
            }
        }
    }

    Ok(pos as u64)
}

fn run_shuffle_part2(
    input: &str,
    deck_size: i128,
    iterations: i128,
    target_pos: i128,
) -> Result<u64> {
    let (_, shuffles) =
        parse_shuffles(input.trim()).map_err(|e| anyhow!("Failed to parse shuffles: {}", e))?;

    // Represent the shuffle as a linear function: f(x) = (ax + b) % m
    // We want to find the inverse: f^-1(x) = (x - b) * a^-1 % m
    // Initial state: f(x) = 1*x + 0 (identity)
    let mut a: i128 = 1;
    let mut b: i128 = 0;

    for shuffle in shuffles.iter().rev() {
        match shuffle {
            Shuffle::DealIntoNewStack => {
                // forward: x -> m - 1 - x = -x - 1
                // reverse: y -> m - 1 - y = -y - 1
                // new_x = -x - 1
                // a' = -a, b' = -b - 1
                a = -a;
                b = -b - 1;
            }
            Shuffle::Cut(n) => {
                // forward: x -> x - n
                // reverse: y -> y + n
                // new_x = x + n
                // a' = a, b' = b + n
                b += n;
            }
            Shuffle::DealWithIncrement(n) => {
                // forward: x -> x * n
                // reverse: y -> y * modinv(n)
                // new_x = x * modinv(n)
                // a' = a * modinv(n), b' = b * modinv(n)
                let inv = mod_inverse(*n, deck_size);
                a = mul_mod(a, inv, deck_size);
                b = mul_mod(b, inv, deck_size);
            }
        }
        a %= deck_size;
        b %= deck_size;
    }

    // Now we have the linear function for ONE reverse iteration: f(x) = ax + b
    // We want to apply this `iterations` times.
    // Composition of linear functions:
    // f(x) = ax + b
    // f(f(x)) = a(ax + b) + b = a^2 x + ab + b
    // f^k(x) = a^k x + b * (1 + a + ... + a^(k-1))
    // Sum of geometric series: (a^k - 1) / (a - 1)  (if a != 1)

    let a_k = mod_pow(a, iterations, deck_size);
    let b_k = if a == 1 {
        mul_mod(b, iterations % deck_size, deck_size)
    } else {
        let num = mul_mod(b, a_k - 1, deck_size);
        let den = mod_inverse(a - 1, deck_size);
        mul_mod(num, den, deck_size)
    };

    let res = (mul_mod(a_k, target_pos, deck_size) + b_k) % deck_size;
    Ok((if res < 0 { res + deck_size } else { res }) as u64)
}

fn mul_mod(a: i128, b: i128, m: i128) -> i128 {
    let res = (a * b) % m;
    if res < 0 {
        res + m
    } else {
        res
    }
}

fn mod_pow(mut base: i128, mut exp: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base %= modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = mul_mod(result, base, modulus);
        }
        base = mul_mod(base, base, modulus);
        exp /= 2;
    }
    result
}

fn mod_inverse(a: i128, m: i128) -> i128 {
    let (g, x, _) = extended_gcd(a, m);
    if g != 1 {
        panic!("Modular inverse does not exist");
    }
    (x % m + m) % m
}

fn extended_gcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x1, y1) = extended_gcd(b % a, a);
        let x = y1 - (b / a) * x1;
        let y = x1;
        (g, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn simulate_deck(input: &str, deck_size: i128) -> Result<Vec<i128>> {
        let mut deck = vec![0; deck_size as usize];
        for card in 0..deck_size {
            let final_pos = run_shuffle(input, deck_size, card)?;
            deck[final_pos as usize] = card;
        }
        Ok(deck)
    }

    #[test]
    fn test_example_1() -> Result<()> {
        let input = "deal with increment 7
deal into new stack
deal into new stack";
        assert_eq!(simulate_deck(input, 10)?, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
        Ok(())
    }

    #[test]
    fn test_example_2() -> Result<()> {
        let input = "cut 6
deal with increment 7
deal into new stack";
        assert_eq!(simulate_deck(input, 10)?, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
        Ok(())
    }

    #[test]
    fn test_example_3() -> Result<()> {
        let input = "deal with increment 7
deal with increment 9
cut -2";
        assert_eq!(simulate_deck(input, 10)?, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
        Ok(())
    }

    #[test]
    fn test_example_4() -> Result<()> {
        let input = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        assert_eq!(simulate_deck(input, 10)?, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
        Ok(())
    }

    #[test]
    fn test_parsing() -> Result<()> {
        let input = "deal with increment 7\ndeal into new stack\ncut -2";
        let (_, shuffles) = parse_shuffles(input).map_err(|e| anyhow!("Parse error: {}", e))?;
        assert_eq!(shuffles.len(), 3);
        match shuffles[0] {
            Shuffle::DealWithIncrement(7) => (),
            _ => panic!("Expected DealWithIncrement(7), found {:?}", shuffles[0]),
        }
        match shuffles[1] {
            Shuffle::DealIntoNewStack => (),
            _ => panic!("Expected DealIntoNewStack, found {:?}", shuffles[1]),
        }
        match shuffles[2] {
            Shuffle::Cut(-2) => (),
            _ => panic!("Expected Cut(-2), found {:?}", shuffles[2]),
        }
        Ok(())
    }

    #[test]
    fn test_part2_math_check() -> Result<()> {
        // Verify that run_shuffle_part2 correctly reverses the shuffle.
        // For a large deck, shuffle once, pick a result position, find the card.
        let input = read_input(Year(2019), Day(22))?;
        let deck_size = 119315717514047i128;
        let iterations = 1;
        
        let card_to_track = 2019;
        // Forward: find where card 2019 ends up
        let pos = run_shuffle(&input, deck_size, card_to_track)?;
        
        // Reverse: at that position, we should find card 2019.
        let card = run_shuffle_part2(&input, deck_size, iterations, pos as i128)?;
        assert_eq!(card as i128, card_to_track);
        
        Ok(())
    }
}
