use regex::{Match, Regex};
use std::collections::HashSet;

fn parse_str(m: Match) -> HashSet<u32> {
    m.as_str()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn part1(lines: Vec<String>) -> Option<String> {
    let pattern =
        Regex::new(r"Card\s+(?<card_id>\d+): (?<winning>[\d\s]+) \| (?<hand>[\d\s]+)").unwrap();
    Some(
        lines
            .iter()
            .map(|line| {
                let captures = pattern.captures(line).unwrap();
                let winning = captures.name("winning").map(parse_str).unwrap();
                let hand = captures.name("hand").map(parse_str).unwrap();
                hand.intersection(&winning)
                    .count()
                    .checked_sub(1)
                    .map(|x| 2u32.pow(x as u32))
                    .unwrap_or(0)
            })
            .sum::<u32>()
            .to_string(),
    )
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    let pattern =
        Regex::new(r"Card\s+(?<card_id>\d+): (?<winning>[\d\s]+) \| (?<hand>[\d\s]+)").unwrap();
    Some(
        lines
            .iter()
            .enumerate()
            .fold(vec![1u32; lines.len()], |mut acc, (index, line)| {
                let captures = pattern.captures(line).unwrap();
                let winning = captures.name("winning").map(parse_str).unwrap();
                let hand = captures.name("hand").map(parse_str).unwrap();
                let wins = hand.intersection(&winning).count();
                for offset in 0..wins {
                    acc[offset + 1 + index] += acc[index]
                }
                acc
            })
            .iter()
            .sum::<u32>()
            .to_string(),
    )
}

#[test]
fn test_part1() {
    let lines = " Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";
    assert_eq!(
        part1(lines.lines().map(String::from).collect()),
        Some("13".into())
    );
}

#[test]
fn test_part2() {
    let lines = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(
        part2(lines.lines().map(String::from).collect()),
        Some("30".into())
    );
}
