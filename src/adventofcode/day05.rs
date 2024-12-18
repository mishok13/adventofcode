use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use itertools::Itertools;
use regex::Regex;

pub fn rules(lines: &Vec<String>) -> Vec<Regex> {
    lines
        .iter()
        .take_while(|&line| line != "")
        .map(|rule| Regex::new(&rule.split('|').rev().join(".*")).unwrap())
        .collect_vec()
}

pub fn rules2(lines: &Vec<String>) -> Vec<Regex> {
    lines
        .iter()
        .take_while(|&line| line != "")
        .map(|rule| Regex::new(&rule.split('|').join(".*")).unwrap())
        .collect_vec()
}

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<i32> {
    let lines = lines.iter().map(|line| line.to_string()).collect_vec();
    let rules = rules(&lines);
    let pages = lines
        .iter()
        .skip_while(|&line| line != "")
        .skip(1)
        .collect_vec();

    Some(
        pages
            .iter()
            .filter(|page| rules.iter().all(|pattern| !pattern.is_match(&page)))
            .map(|page| {
                let numbers = page.split(',').collect_vec();
                numbers[numbers.len() / 2].parse::<i32>().unwrap()
            })
            .sum(),
    )
}

pub fn part2<T: ToString>(lines: Vec<T>) -> Option<i32> {
    let lines = lines.iter().map(|line| line.to_string()).collect_vec();
    let sort_keys: HashMap<i32, HashSet<i32>> = lines
        .iter()
        .take_while(|&line| line != "")
        .map(|rule| {
            rule.split('|')
                .map(|n| n.parse::<i32>().unwrap())
                .collect_vec()
        })
        .fold(HashMap::new(), |mut map, value| {
            map.entry(value[0])
                .or_insert(HashSet::new())
                .insert(value[1]);
            map
        });

    let rules = rules(&lines);

    Some(
        lines
            .iter()
            .skip_while(|&line| line != "")
            .skip(1)
            .filter(|&page| rules.iter().any(|pattern| pattern.is_match(&page)))
            .map(|page| {
                page.split(',')
                    .map(|n| n.parse::<i32>().unwrap())
                    .sorted_by(|x, y| {
                        sort_keys
                            .get(x)
                            .map(|higher| {
                                if higher.contains(y) {
                                    Ordering::Less
                                } else {
                                    Ordering::Greater
                                }
                            })
                            .unwrap_or(Ordering::Greater)
                    })
                    .collect_vec()
            })
            .map(|ordered| {
                tracing::debug!("{:?}", ordered);
                ordered[ordered.len() / 2]
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[test]
    fn part1() {
        let lines: Vec<&str> = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];
        let result = super::part1(lines);
        assert_eq!(result, Some(143));
    }

    #[traced_test]
    #[test]
    fn part2() {
        let lines: Vec<&str> = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ];
        let result = super::part2(lines);
        assert_eq!(result, Some(123))
    }
}
