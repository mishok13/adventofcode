use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::{HashSet, HashMap};

fn day1part1(path: &Path) -> Option<String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap());
    let offset = numbers.fold(0, |c, x| c + x);
    println!("Day 1, part 1 answer: {}", offset);
    Some(offset.to_string())
}

fn day1part2(path: &Path) -> Option<String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut current = 0i32;
    for offset in numbers.iter().cycle() {
        current += offset;
        if !seen.insert(current) {
            return Some(current.to_string())
        }
    }
    None
}

fn count_chars(string: String) -> HashMap<char, u32> {
    let mut counter = HashMap::new();
    for char in string.chars() {
        let entry = counter.entry(char).or_insert(0);
        *entry += 1;
    }
    counter
}

fn day2part1(path: &Path) -> Option<String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let box_ids = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| count_chars(line))
        .map(|counter| (counter.values().any(|&count| count == 2), counter.values().any(|&count| count == 3)))
        .fold((0, 0), |collector, matches| {
            match matches {
                (true, true) => (collector.0 + 1, collector.1 + 1),
                (true, false) => (collector.0 + 1, collector.1),
                (false, true) => (collector.0, collector.1 + 1),
                (false, false) => collector
            }});
    Some((box_ids.0 * box_ids.1).to_string())
}

fn unique_subwords(word: String) -> impl Iterator<Item = String> {
    let mut subwords = HashSet::new();
    for index in 0..word.chars().count() {
        let mut subword = word.clone();
        subword.remove(index);
        subwords.insert(subword);
    }
    subwords.into_iter()
}

fn day2part2(path: &Path) -> Option<String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let subwords: Vec<String> = reader.lines().map(|line| line.unwrap()).flat_map(unique_subwords).collect();
    let mut seen = HashSet::new();
    for word in subwords.iter() {
        if !seen.insert(word.clone()) {
            return Some(word.to_string())
        }
    }
    None
}

struct Solution<'a> {
    path: &'a Path,
    day: u8,
    part: u8,
    solver: fn(&Path) -> Option<String>
}

impl<'a> Solution<'a> {
    fn new(path: &'a str, day: u8, part: u8, solver: fn(&Path) -> Option<String>) -> Solution<'a> {
        Solution{path: &Path::new(path),
                 day: day,
                 part: part,
                 solver: solver}
    }

    fn solve(self) {
        if let Some(answer) = (self.solver)(self.path) {
            println!("Day: {}, part: {}, answer: {}", self.day, self.part, answer)
        } else {
            println!("Day: {}, part: {}, no solution found", self.day, self.part)
        }
    }
}

fn main()  {
    // for day, part, path, solver
    // open path, pass to solver
    // print out returned result
    let solutions = vec![
        Solution::new("00.txt", 1, 1, day1part1),
        Solution::new("00.txt", 1, 2, day1part2),
        Solution::new("01.txt", 1, 1, day2part1),
        Solution::new("01.txt", 1, 2, day2part2),
    ];
    for solution in solutions {
        solution.solve()
    }
    // let path = Path::new("00.txt");
    // day1part1(path);
    // day1part2(path);
    // day2part1(Path::new("01.txt"));
    // day2part2(Path::new("01.txt"));
}
