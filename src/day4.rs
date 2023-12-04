use regex::{Match, Regex};
use std::collections::{BTreeMap, HashSet};

fn parse_str(m: Match) -> HashSet<u32> {
    m.as_str()
        .split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| u32::from_str_radix(s, 10).unwrap())
        .collect::<HashSet<_>>()
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
            .map(|(index, line)| {
                let captures = pattern.captures(line).unwrap();
                let winning = captures.name("winning").map(parse_str).unwrap();
                let hand = captures.name("hand").map(parse_str).unwrap();
                let wins = hand.intersection(&winning).count();
                if wins > 0 {
                    (
                        index,
                        (0..wins)
                            .filter_map(|x| x.checked_add(index + 1))
                            .collect::<Vec<_>>(),
                    )
                } else {
                    (index, vec![])
                }
            })
            .fold(BTreeMap::new(), |mut acc, (index, values)| {
                let times = acc
                    .entry(index)
                    .and_modify(|x| *x += 1)
                    .or_insert(1)
                    .to_owned();
                if !values.is_empty() {
                    for value in values.into_iter() {
                        acc.entry(value)
                            .and_modify(|x| *x += times)
                            .or_insert(times);
                    }
                }
                acc
            })
            .values()
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
