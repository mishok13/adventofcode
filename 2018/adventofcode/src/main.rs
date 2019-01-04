#[macro_use] extern crate itertools;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::{HashSet, HashMap};


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

fn read_lines(path: &Path) -> impl Iterator<Item = String> {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap())
}

fn day1part1(path: &Path) -> Option<String> {
    let numbers = read_lines(path).map(|line| line.parse::<i32>().unwrap());
    let offset = numbers.fold(0, |c, x| c + x);
    println!("Day 1, part 1 answer: {}", offset);
    Some(offset.to_string())
}

fn day1part2(path: &Path) -> Option<String> {
    let numbers = read_lines(path).map(|line| line.parse::<i32>().unwrap()).collect::<Vec<i32>>();
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
    let box_ids = read_lines(path)
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
    let subwords: Vec<String> = read_lines(path).flat_map(unique_subwords).collect();
    let mut seen = HashSet::new();
    for word in subwords.iter() {
        if !seen.insert(word.clone()) {
            return Some(word.to_string())
        }
    }
    None
}

fn day3part1(path: &Path) -> Option<String> {
    let mut coordinates = HashMap::new();
    for line in read_lines(path) {
        let parts = line.split(' ').collect::<Vec<_>>();
        let index = parts[0].trim_matches('#').parse::<i32>().unwrap();
        let start = parts[2].trim_matches(':').split(',').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let size = parts[3].split('x').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        for (x, y) in iproduct!(start[0]..start[0] + size[0], start[1]..start[1] + size[1]) {
            let value = 1 + coordinates.get(&(x, y)).unwrap_or(&0);
            coordinates.insert((x, y), value);
        }
    }
    Some(coordinates.values().filter(|&x| *x > 1).count().to_string())
}

fn day3part2(path: &Path) -> Option<String> {
    let mut coordinates = HashMap::new();
    for line in read_lines(path) {
        let parts = line.split(' ').collect::<Vec<_>>();
        let index = parts[0].trim_matches('#').parse::<i32>().unwrap();
        let start = parts[2].trim_matches(':').split(',').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let size = parts[3].split('x').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        for (x, y) in iproduct!(start[0]..start[0] + size[0], start[1]..start[1] + size[1]) {
            let value = 1 + coordinates.get(&(x, y)).unwrap_or(&0);
            coordinates.insert((x, y), value);
        }
    }
    for line in read_lines(path) {
        let mut intersects = false;
        let parts = line.split(' ').collect::<Vec<_>>();
        let index = parts[0].trim_matches('#').parse::<i32>().unwrap();
        let start = parts[2].trim_matches(':').split(',').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        let size = parts[3].split('x').map(|index| index.parse::<u32>().unwrap()).collect::<Vec<_>>();
        for (x, y) in iproduct!(start[0]..start[0] + size[0], start[1]..start[1] + size[1]) {
            let value = coordinates.get(&(x, y)).unwrap_or(&0);
            if *value > 1 {
                intersects = true;
                break
            }
        }
        if !intersects {
            return Some(index.to_string())
        }
    }
    None
}

fn main()  {
    // for day, part, path, solver
    // open path, pass to solver
    // print out returned result
    let solutions = vec![
        Solution::new("00.txt", 1, 1, day1part1),
        Solution::new("00.txt", 1, 2, day1part2),
        Solution::new("01.txt", 2, 1, day2part1),
        Solution::new("01.txt", 2, 2, day2part2),
        Solution::new("02.txt", 3, 1, day3part1),
        Solution::new("02.txt", 3, 1, day3part2),
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
