use regex::Regex;

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<String> {
    // tracing::info!(
    //     "Total matches: {:?}",
    //     pattern.find_iter(&lines[0].to_string()).count()
    // );

    Some(
        lines
            .iter()
            .map(|l| count(&l.to_string()))
            .sum::<i64>()
            .to_string(),
    )
}

fn count(line: &str) -> i64 {
    let pattern = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    pattern
        .find_iter(line)
        .map(|m| {
            tracing::info!("Match: {:?}", m);
            m
        })
        .map(|m| m.as_str().get(4..).unwrap().trim_matches(')'))
        .map(|s| {
            s.split(',')
                .map(|part| part.parse::<i64>().unwrap())
                .product::<i64>()
        })
        .sum()
}

fn find_ranges(line: &str) -> Vec<String> {
    let start_pattern = Regex::new(r"do\(\)").unwrap();
    let stop_pattern = Regex::new(r"don't\(\)").unwrap();
    let mut start = 0usize;
    let mut result = vec![];
    loop {
        let chunk = line.get(start..line.len()).unwrap();
        tracing::debug!("start {} {}", start, chunk);
        if let Some(stop) = stop_pattern.find(chunk).map(|m| m.start()) {
            tracing::debug!("stop {} {}", stop, line.get(stop..).unwrap());
            result.push(line.get(start..start + stop).unwrap().into());
            if let Some(new_start) = start_pattern
                .find(chunk.get(stop..).unwrap())
                .map(|m| m.end())
            {
                start += stop + new_start;
            } else {
                break;
            }
        } else {
            result.push(chunk.into());
            break;
        }
    }
    result
}

pub fn part2<T: ToString>(lines: Vec<T>) -> Option<String> {
    // find do's and don'ts, find acceptable range and feed said substring into `count`
    // that's it?
    Some(
        lines
            .iter()
            .map(|line| line.to_string())
            .map(|line| {
                find_ranges(&line)
                    .iter()
                    .map(|substring| count(substring))
                    .sum::<i64>()
            })
            .sum::<i64>()
            .to_string(),
    )
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[test]
    fn part1() {
        let lines: Vec<&str> =
            vec!["xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"];
        let result = super::part1(lines);
        assert_eq!(result, Some(161.to_string()));
    }

    #[traced_test]
    #[test]
    fn part2() {
        let lines: Vec<&str> =
            vec!["xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"];
        let result = super::part2(lines);
        assert_eq!(result, Some("48".to_string()));
    }
}
