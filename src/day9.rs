use itertools::Itertools;

use crate::util;

#[derive(Debug)]
struct Progression {
    values: Vec<i128>,
    starts: Vec<i128>,
}

impl Progression {
    fn is_arithmetic(&self) -> bool {
        diffs(&self.values).iter().all_equal()
    }

    fn up(&mut self) {
        let start = self.starts.pop().unwrap();
        self.values = from_diffs(start, &self.values);
    }

    fn expand(&mut self) {
        self.values
            .push(self.values.last().unwrap() + diffs(&self.values).last().unwrap());
    }

    fn down(&mut self) {
        self.starts.push(self.values[0]);
        self.values = diffs(&self.values);
    }

    fn new(values: &[i128]) -> Self {
        Self {
            values: values.to_vec(),
            starts: vec![],
        }
    }
}

fn diffs(values: &[i128]) -> Vec<i128> {
    let res = values[1..]
        .iter()
        .fold((Vec::new(), &values[0]), |(mut acc, previous), current| {
            acc.push(current - previous);
            (acc, current)
        });
    res.0
}

fn from_diffs(start: i128, values: &[i128]) -> Vec<i128> {
    values.iter().fold(vec![start], |mut acc, x| {
        acc.push(x + acc.last().unwrap());
        acc
    })
}

pub fn part1(lines: Vec<String>) -> Option<i128> {
    lines
        .iter()
        .map(|line| util::parse_space_separated_str(line))
        .map(|v| {
            let mut p = Progression::new(&v);
            while !p.is_arithmetic() {
                p.down();
            }
            p.expand();
            while !p.starts.is_empty() {
                p.up();
            }
            *p.values.last().unwrap()
        })
        .reduce(|a, b| a + b)
}

pub fn part2(lines: Vec<String>) -> Option<i128> {
    lines
        .iter()
        .map(|line| util::parse_space_separated_str(line))
        .map(|mut v| {
            v.reverse();
            let mut p = Progression::new(&v);
            while !p.is_arithmetic() {
                p.down();
            }
            p.expand();
            while !p.starts.is_empty() {
                p.up();
            }
            *p.values.last().unwrap()
        })
        .reduce(|a, b| a + b)
}

#[test]
fn test_part1() {
    let lines = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines), Some(114));
}

#[test]
fn test_part2() {
    let lines = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part2(lines), Some(2));
}
