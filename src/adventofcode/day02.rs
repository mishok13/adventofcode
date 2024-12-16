use itertools::Itertools;
use tracing::Level;

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<String> {
    Some(
        lines
            .iter()
            .map(|line| {
                line.to_string()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .filter(|line| is_valid(line.iter()))
            .count()
            .to_string(),
    )
}

fn is_valid<'a>(numbers: impl Iterator<Item = &'a i64>) -> bool {
    let diffs = numbers.tuple_windows().map(|(x, y)| (x - y)).collect_vec();
    tracing::info!("wtf {:?}", diffs);
    diffs.iter().map(|diff| diff.signum()).all_equal() && diffs.iter().all(|diff| diff.abs() <= 3)
}

pub fn part2<T: ToString>(lines: Vec<T>) -> Option<String> {
    Some(
        lines
            .iter()
            .map(|line| {
                line.to_string()
                    .split_whitespace()
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect_vec()
            })
            .filter(|line| {
                is_valid(line.iter())
                    || (0..line.len())
                        .map(|index| {
                            is_valid(line.iter().get(..index).chain(line.iter().get(index + 1..)))
                        })
                        .any(|result| result)
            })
            .count()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[traced_test]
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
    fn part2() {
        let lines: Vec<&str> = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
            "71 69 70 71 72 75",
            "2 1 2 3 4 5 6",
        ];
        let result = super::part2(lines);
        assert_eq!(result, Some("6".to_string()));
    }
}
