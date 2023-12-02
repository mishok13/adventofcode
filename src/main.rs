use clap::Parser;
use regex::Regex;
use std::char;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: String,
    task: String,
    input: Option<PathBuf>,
}

fn day01_1(mut input: Vec<String>) -> Option<String> {
    input
        .iter_mut()
        .map(|line| {
            let candidate = &format!(
                "{}{}",
                line.chars()
                    .nth(line.find(|c| char::is_digit(c, 10)).unwrap())
                    .unwrap(),
                line.chars()
                    .nth(line.rfind(|c| char::is_digit(c, 10)).unwrap())
                    .unwrap(),
            )[..];
            u32::from_str_radix(candidate, 10).unwrap()
        })
        .reduce(|acc, s| acc + s)
        .map(|total| format!("{}", total))
}

fn day01_2(input: Vec<String>) -> Option<String> {
    let forward_pattern = Regex::new("one|two|three|four|five|six|seven|eight|nine|[0-9]").unwrap();
    let backward_pattern =
        Regex::new("[0-9]|enin|thgie|neves|xis|evif|ruof|eerht|owt|eno").unwrap();
    input
        .iter()
        .map(|line| {
            let backward_line = line.chars().rev().collect::<String>();
            let forward_match = forward_pattern
                .find_iter(line)
                .map(|m| m.as_str())
                .nth(0)
                .unwrap();
            let backward_match = backward_pattern
                .find_iter(&backward_line)
                .map(|m| m.as_str())
                .nth(0)
                .unwrap();
            10 * match_english_to_digit(forward_match) + match_english_to_digit(backward_match)
        })
        .reduce(|acc, s| acc + s)
        .map(|total| format!("{}", total))
}

fn match_english_to_digit(s: &str) -> u32 {
    match s {
        "one" | "eno" | "1" => 1,
        "two" | "owt" | "2" => 2,
        "three" | "eerht" | "3" => 3,
        "four" | "ruof" | "4" => 4,
        "five" | "evif" | "5" => 5,
        "six" | "xis" | "6" => 6,
        "seven" | "neves" | "7" => 7,
        "eight" | "thgie" | "8" => 8,
        "nine" | "enin" | "9" => 9,
        _ => panic!("Can not match some things {}", s),
    }
}

fn main() {
    let cli = Cli::parse();
    let lines = cli
        .input
        .map(|p| {
            read_to_string(p)
                .unwrap()
                .lines()
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap();
    let result = match (cli.day.as_str(), cli.task.as_str()) {
        ("1", "1") => day01_1(lines),
        ("1", "2") => day01_2(lines),
        _ => None,
    };

    match result {
        Some(lines) => println!("{}", lines),
        None => println!("No solution available"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_1() {
        let lines = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(
            day01_1(lines.lines().map(String::from).collect()),
            Some("142".into())
        )
    }

    #[test]
    fn test01_2() {
        let lines = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(
            day01_2(lines.lines().map(String::from).collect()),
            Some("281".into())
        )
    }
}
