use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashSet;

fn day1part1(path: &Path) {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap());
    let offset = numbers.fold(0, |c, x| c + x);
    println!("Day 1, part 1 answer: {}", offset);
}

fn day1part2(path: &Path) {
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let numbers = reader.lines().map(|line| line.unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
    let mut seen = HashSet::new();
    seen.insert(0);
    let mut current = 0i32;
    for offset in numbers.iter().cycle() {
        current += offset;
        if !seen.insert(current) {
            println!("Day 1, part 2 answer: {}", current);
            break
        }
    }
}

fn day2(path: &Path) {

}

fn main()  {
    let path = Path::new("00.txt");
    day1part1(path);
    day1part2(path);
}
