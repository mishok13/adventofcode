use primefactor::PrimeFactors;
use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

#[derive(Debug)]
struct Map<'a> {
    values: HashMap<&'a str, (&'a str, &'a str)>,
}

enum Direction {
    Left,
    Right,
}

impl<'a> Map<'a> {
    fn new(v: &'a [String]) -> Self {
        let pattern =
            Regex::new(r"(?<key>[0-9A-Z]+) \= \((?<left>[0-9A-Z]+), (?<right>[0-9A-Z]+)\)")
                .unwrap();
        let values = v
            .iter()
            .flat_map(|l| pattern.captures(l))
            .map(|c| {
                (
                    c.name("key").unwrap().as_str(),
                    (
                        c.name("left").unwrap().as_str(),
                        c.name("right").unwrap().as_str(),
                    ),
                )
            })
            .fold(HashMap::new(), |mut acc, (k, v)| {
                acc.insert(k, v);
                acc
            });
        Map { values }
    }

    fn step(&self, direction: &Direction, value: &str) -> &str {
        self.values
            .get(value)
            .map(|v| match direction {
                Direction::Left => v.0,
                Direction::Right => v.1,
            })
            .unwrap()
    }

    fn solve(&self, directions: &[Direction], start: &str) -> usize {
        directions
            .iter()
            .cycle()
            .fold_while((start, 0), |(key, count), direction| {
                if key.ends_with('Z') {
                    itertools::FoldWhile::Done((key, count))
                } else {
                    itertools::FoldWhile::Continue((self.step(direction, key), count + 1))
                }
            })
            .into_inner()
            .1
    }
}

fn parse_directions(s: &str) -> Vec<Direction> {
    s.chars()
        .map(|c| {
            if c == 'L' {
                Direction::Left
            } else {
                Direction::Right
            }
        })
        .collect()
}

pub fn part1(lines: Vec<String>) -> Option<String> {
    let directions = parse_directions(&lines[0]);
    let map = Map::new(&lines[2..]);
    Some(map.solve(&directions, "AAA").to_string())
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    let directions = parse_directions(&lines[0]);
    let map = Map::new(&lines[2..]);
    Some(
        map.values
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|x| {
                PrimeFactors::from(map.solve(&directions, x) as u128)
                    .iter()
                    .fold(HashMap::new(), |mut acc, factor| {
                        acc.insert(factor.integer, factor.exponent);
                        acc
                    })
            })
            .reduce(|left, right| {
                right.iter().fold(left, |mut acc, (f, e)| {
                    acc.insert(*f, *acc.get(f).map(|x| x.max(e)).unwrap_or(e));
                    acc
                })
            })
            .unwrap()
            .iter()
            .fold(1, |acc, (factor, &exp)| acc * factor.pow(exp))
            .to_string(),
    )
}

#[test]
fn test_part1() {
    let lines = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines), Some("2".into()));
    let lines2 = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines2), Some("6".into()));
}

#[test]
fn test_part2() {
    let lines = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(
        part2(lines.lines().map(String::from).collect()),
        Some("6".into())
    );
}
