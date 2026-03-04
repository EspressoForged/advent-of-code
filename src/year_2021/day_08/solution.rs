use crate::utils::{read_input, Day, Year};
use anyhow::Result;
use std::collections::{HashMap, HashSet};

/// Solves Year 2021, Day 08: Seven Segment Search.
///
/// # Errors
/// Returns an error if the input cannot be read or parsed.
pub fn solve() -> Result<(u64, u64)> {
    let input = read_input(Year(2021), Day(8))?;
    let (p1, p2) = calculate_solution(&input);
    Ok((p1, p2))
}

fn calculate_solution(input: &str) -> (u64, u64) {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(" | ").collect();
        let signals: Vec<&str> = parts[0].split_whitespace().collect();
        let outputs: Vec<&str> = parts[1].split_whitespace().collect();

        // Part 1
        for output in &outputs {
            let len = output.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                part1 += 1;
            }
        }

        // Part 2
        part2 += decode_line(&signals, &outputs);
    }

    (part1, part2)
}

fn decode_line(signals: &[&str], outputs: &[&str]) -> u64 {
    let mut patterns = HashMap::new();
    let mut by_len: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();

    for sig in signals {
        let set: HashSet<char> = sig.chars().collect();
        by_len.entry(sig.len()).or_default().push(set);
    }

    let s1 = by_len[&2][0].clone();
    let s4 = by_len[&4][0].clone();
    let s7 = by_len[&3][0].clone();
    let s8 = by_len[&7][0].clone();

    // Length 6: 0, 6, 9
    let mut s0 = HashSet::new();
    let mut s6 = HashSet::new();
    let mut s9 = HashSet::new();

    for set in &by_len[&6] {
        if s4.is_subset(set) {
            s9 = set.clone();
        } else if s7.is_subset(set) {
            s0 = set.clone();
        } else {
            s6 = set.clone();
        }
    }

    // Length 5: 2, 3, 5
    let mut s2 = HashSet::new();
    let mut s3 = HashSet::new();
    let mut s5 = HashSet::new();

    for set in &by_len[&5] {
        if s7.is_subset(set) {
            s3 = set.clone();
        } else if set.is_subset(&s6) {
            s5 = set.clone();
        } else {
            s2 = set.clone();
        }
    }

    patterns.insert(sort_chars(&s0), 0);
    patterns.insert(sort_chars(&s1), 1);
    patterns.insert(sort_chars(&s2), 2);
    patterns.insert(sort_chars(&s3), 3);
    patterns.insert(sort_chars(&s4), 4);
    patterns.insert(sort_chars(&s5), 5);
    patterns.insert(sort_chars(&s6), 6);
    patterns.insert(sort_chars(&s7), 7);
    patterns.insert(sort_chars(&s8), 8);
    patterns.insert(sort_chars(&s9), 9);

    let mut val = 0;
    for out in outputs {
        let out_sig = sort_string(out);
        val = val * 10 + patterns[&out_sig];
    }

    val
}

fn sort_chars(set: &HashSet<char>) -> String {
    let mut v: Vec<char> = set.iter().copied().collect();
    v.sort_unstable();
    v.into_iter().collect()
}

fn sort_string(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v.sort_unstable();
    v.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_example() {
        let (p1, p2) = calculate_solution(EXAMPLE);
        assert_eq!(p1, 26);
        assert_eq!(p2, 61229);
    }

    #[test]
    fn test_single_line() {
        let input = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let (_, p2) = calculate_solution(input);
        assert_eq!(p2, 5353);
    }
}
