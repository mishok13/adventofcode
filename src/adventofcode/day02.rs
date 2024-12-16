use std::{collections::HashMap, iter::zip};

use itertools::{sorted, Itertools};

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<String> {
    Some(
        lines
            .iter()
            .map(|line| {
                line.to_string()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .tuple_windows()
                    .map(|(x, y)| (x - y))
                    .collect_vec()
            })
            .filter(|diffs| {
                diffs.iter().map(|diff| diff.signum()).all_equal()
                    && diffs.iter().all(|diff| diff.abs() <= 3)
            })
            .count()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let lines: Vec<&str> = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ];
        let result = super::part1(lines);
        assert_eq!(result, Some("2".to_string()));
    }

    #[test]
    fn part2() {}
}
