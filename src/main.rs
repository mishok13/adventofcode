pub mod util;

mod adventofcode;

use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    day: String,
    task: String,
    input: Option<PathBuf>,
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
        ("1", "1") => adventofcode::day01::part1(lines),
        ("1", "2") => adventofcode::day01::part2(lines),
        ("2", "1") => adventofcode::day02::part1(lines),
        // ("1", "2") => day01_2(lines),
        // ("2", "1") => day02_1(lines),
        // ("2", "2") => day02_2(lines),
        // ("3", "1") => day03_1(lines),
        // ("3", "2") => day03_2(lines),
        // ("4", "1") => day4::part1(lines),
        // ("4", "2") => day4::part2(lines),
        // ("5", "1") => day5::part1(lines),
        // ("5", "2") => day5::part2(lines),
        // ("6", "1") => day6::part1(lines),
        // ("6", "2") => day6::part2(lines),
        // ("7", "1") => day7::part1(lines),
        // ("7", "2") => day7::part2(lines),
        // ("8", "1") => day8::part1(lines),
        // ("8", "2") => day8::part2(lines),
        // ("9", "1") => day9::part1(lines).map(|x| x.to_string()),
        // ("9", "2") => day9::part2(lines).map(|x| x.to_string()),
        // ("10", "1") => day10::part1(lines).map(|x| x.to_string()),
        // ("10", "2") => day10::part2(lines).map(|x| x.to_string()),
        // ("11", "1") => day11::part1(lines).map(|x| x.to_string()),
        // ("11", "2") => day11::part2(lines).map(|x| x.to_string()),
        // ("12", "1") => day12::part1(lines).map(|x| x.to_string()),
        // ("12", "2") => day12::part2(lines).map(|x| x.to_string()),
        _ => None,
    };

    match result {
        Some(result) => println!("{}", result),
        None => println!("No solution available"),
    }
}
