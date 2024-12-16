use std::{collections::HashMap, iter::zip};

use itertools::sorted;

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<String> {
    let pairs = lines.iter().map(|line| {
        line.to_string()
            .split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });
    let (left, right) = pairs.fold((vec![], vec![]), |(mut left, mut right), pair| {
        left.push(pair[0]);
        right.push(pair[1]);
        (left, right)
    });
    Some(
        zip(sorted(left), sorted(right))
            .map(|(x, y)| x.abs_diff(y))
            .sum::<u64>()
            .to_string(),
    )
}

pub fn part2<T: ToString>(lines: Vec<T>) -> Option<String> {
    let pairs = lines.iter().map(|line| {
        line.to_string()
            .split_whitespace()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    });
    let (left, right) = pairs.fold((vec![], vec![]), |(mut left, mut right), pair| {
        left.push(pair[0]);
        right.push(pair[1]);
        (left, right)
    });
    let right = right.iter().fold(HashMap::new(), |mut counter, number| {
        *counter.entry(number).or_insert(0) += 1;
        counter
    });
    Some(
        left.iter()
            .map(|x| right.get(x).unwrap_or(&0) * x)
            .sum::<i64>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let lines: Vec<&str> = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
        let result = super::part1(lines);
        assert_eq!(result, Some("11".to_string()));
    }

    #[test]
    fn part2() {
        let lines: Vec<&str> = vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"];
        let result = super::part2(lines);
        assert_eq!(result, Some("31".to_string()));
    }
}
