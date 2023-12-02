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
    Some(
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
            .sum::<u32>()
            .to_string(),
    )
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

fn day02_1(lines: Vec<String>) -> Option<String> {
    let game_pattern = Regex::new(r"Game (?<game_id>\d+): (?<plays>.+)").unwrap();
    let step_pattern = Regex::new(r"(?<amount>\d+) (?<color>blue|green|red)").unwrap();
    Some(
        lines
            .iter()
            .filter_map(|line| {
                let game_id =
                    u32::from_str_radix(&game_pattern.captures(&line).unwrap()["game_id"], 10)
                        .unwrap();
                let rest = &game_pattern.captures(&line).unwrap()["plays"];
                let plays: Vec<Vec<(String, u32)>> = rest
                    .split(";")
                    .map(|play| {
                        play.split(",")
                            .map(|step| {
                                let captures = step_pattern.captures(step).unwrap();
                                (
                                    captures["color"].to_string(),
                                    u32::from_str_radix(&captures["amount"], 10).unwrap(),
                                )
                            })
                            .collect()
                    })
                    .collect();
                if plays.iter().flatten().all(|(color, amount)| {
                    (color == "red" && *amount <= 12)
                        || (color == "green" && *amount <= 13)
                        || (color == "blue" && *amount <= 14)
                }) {
                    Some(game_id)
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string(),
    )
}

fn day02_2(lines: Vec<String>) -> Option<String> {
    let game_pattern = Regex::new(r"Game (?<game_id>\d+): (?<plays>.+)").unwrap();
    let step_pattern = Regex::new(r"(?<amount>\d+) (?<color>blue|green|red)").unwrap();
    Some(
        lines
            .iter()
            .map(|line| {
                let rest = &game_pattern.captures(&line).unwrap()["plays"];
                let plays: Vec<Vec<(String, u32)>> = rest
                    .split(";")
                    .map(|play| {
                        play.split(",")
                            .map(|step| {
                                let captures = step_pattern.captures(step).unwrap();
                                (
                                    captures["color"].to_string(),
                                    u32::from_str_radix(&captures["amount"], 10).unwrap(),
                                )
                            })
                            .collect()
                    })
                    .collect();
                plays
                    .iter()
                    .flatten()
                    .fold(
                        vec![u32::MIN, u32::MIN, u32::MIN],
                        |mut acc, (color, amount)| {
                            match color.as_str() {
                                "red" => acc[0] = std::cmp::max(acc[0], *amount),
                                "blue" => acc[1] = std::cmp::max(acc[1], *amount),
                                "green" => acc[2] = std::cmp::max(acc[2], *amount),
                                _ => panic!("unknown color"),
                            };
                            acc
                        },
                    )
                    .into_iter()
                    .reduce(|a, i| a * i)
                    .unwrap()
            })
            .sum::<u32>()
            .to_string(),
    )
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
        ("2", "1") => day02_1(lines),
        ("2", "2") => day02_2(lines),
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

    #[test]
    fn test02() {
        let lines = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(
            day02_1(lines.lines().map(String::from).collect()),
            Some("8".into())
        );
        assert_eq!(
            day02_2(lines.lines().map(String::from).collect()),
            Some("2286".into())
        );
    }
}
