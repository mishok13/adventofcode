use itertools::Itertools;

fn parse_str(s: &str) -> Vec<f64> {
    s.split(" ")
        .map(|s| s.parse::<f64>().ok())
        .filter_map(|x| x)
        .collect()
}

pub fn part1(lines: Vec<String>) -> Option<String> {
    let times = parse_str(&lines[0]);
    let distances = parse_str(&lines[1]);
    let zipped: Vec<_> = times.into_iter().zip(distances).collect();
    zipped
        .into_iter()
        .map(|(t, d)| {
            let max_point = ((t.powf(2.0) / 4.0) - d).sqrt();
            ((t / 2.0 + max_point - 1e-10).floor() + 1.0 - (t / 2.0 - max_point + 1e-10).ceil())
                as i64
        })
        .reduce(|a, b| a * b)
        .map(|x| x.to_string())
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    let t = lines[0]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .join("")
        .parse::<f64>()
        .unwrap();
    let d = lines[1]
        .chars()
        .filter(|c| c.is_ascii_digit())
        .join("")
        .parse::<f64>()
        .unwrap();
    let m = ((t.powf(2.0) / 4.0) - d).sqrt();
    Some((((t / 2.0 + m - 1e-10).floor() + 1.0 - (t / 2.0 - m + 1e-10).ceil()) as i64).to_string())
}

#[test]
fn test_part1() {
    let lines = "Time:      7  15   30
Distance:  9  40  200"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines), Some("288".into()));
}

#[test]
fn test_part2() {
    let lines = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(
        part2(lines.lines().map(String::from).collect()),
        Some("71503".into())
    );
}
