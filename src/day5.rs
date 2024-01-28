use core::ops::Range;

use regex::Regex;

#[derive(Debug)]
struct Map {
    range: Range<usize>,
    target: usize,
}

impl Map {
    fn new(source: usize, target: usize, offset: usize) -> Self {
        Self {
            range: source..source + offset,
            target,
        }
    }

    fn path(&self, item: &usize) -> Option<usize> {
        if self.range.contains(item) {
            Some(item - self.range.start + self.target)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Game {
    seeds: Vec<usize>,
    maps: Vec<Vec<Map>>,
}

impl Game {
    fn new() -> Self {
        Self {
            seeds: Vec::new(),
            maps: Vec::new(),
        }
    }

    fn solve(&self) -> usize {
        self.seeds
            .iter()
            .map(|seed| {
                let mut result = *seed;
                for map in self.maps.iter() {
                    if let Some(update) = map.iter().filter_map(|x| x.path(&result)).nth(0) {
                        result = update;
                    }
                }
                result
            })
            .min()
            .unwrap()
    }

    fn solve_ranges(&self) -> usize {
        let mut ranges = vec![];
        for index in 0..self.seeds.len() {
            if index % 2 == 0 {
                ranges.push(self.seeds[index]..self.seeds[index] + self.seeds[index + 1])
            }
        }
        ranges
            .iter()
            .map(|r| {
                r.clone()
                    .map(|seed| {
                        let mut result = seed;
                        for map in self.maps.iter() {
                            if let Some(update) = map.iter().filter_map(|x| x.path(&result)).nth(0)
                            {
                                result = update;
                            }
                        }
                        result
                    })
                    .min()
                    .unwrap()
            })
            .min()
            .unwrap()
    }
}

pub fn part1(lines: Vec<String>) -> Option<String> {
    let seeds_pattern = Regex::new(r"seeds: (?<seeds>[\d\s]+)").unwrap();
    let maps_pattern = Regex::new(r"(?<target>\d+) (?<source>\d+) (?<range>\d+)").unwrap();
    let game = lines.iter().fold(Game::new(), |mut acc, line| {
        if acc.seeds.is_empty() {
            seeds_pattern
                .captures(line)
                .map(|captures| {
                    captures
                        .name("seeds")
                        .unwrap()
                        .as_str()
                        .split(' ')
                        .for_each(|x| acc.seeds.push(x.parse::<usize>().unwrap()))
                })
                .unwrap();
        } else {
            if line.is_empty() {
                acc.maps.push(Vec::new());
            }
            if let Some(captures) = maps_pattern.captures(line) {
                // println!("{line} {:?}", acc.maps.last().unwrap());
                let source = captures
                    .name("source")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let target = captures
                    .name("target")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let range = captures
                    .name("range")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();

                let last = acc.maps.last_mut().unwrap();
                last.push(Map::new(source, target, range))
            }
        }
        acc
    });
    Some(game.solve().to_string())
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    let seeds_pattern = Regex::new(r"seeds: (?<seeds>[\d\s]+)").unwrap();
    let maps_pattern = Regex::new(r"(?<target>\d+) (?<source>\d+) (?<range>\d+)").unwrap();
    let game = lines.iter().fold(Game::new(), |mut acc, line| {
        if acc.seeds.is_empty() {
            seeds_pattern
                .captures(line)
                .map(|captures| {
                    captures
                        .name("seeds")
                        .unwrap()
                        .as_str()
                        .split(' ')
                        .for_each(|x| acc.seeds.push(x.parse::<usize>().unwrap()))
                })
                .unwrap();
        } else {
            if line.is_empty() {
                acc.maps.push(Vec::new());
            }
            if let Some(captures) = maps_pattern.captures(line) {
                // println!("{line} {:?}", acc.maps.last().unwrap());
                let source = captures
                    .name("source")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let target = captures
                    .name("target")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();
                let range = captures
                    .name("range")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap();

                let last = acc.maps.last_mut().unwrap();
                last.push(Map::new(source, target, range))
            }
        }
        acc
    });
    Some(game.solve_ranges().to_string())
}
#[test]
fn test_part1() {
    let lines = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines), Some("35".into()));
}

#[test]
fn test_part2() {
    let lines = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(
        part2(lines.lines().map(String::from).collect()),
        Some("46".into())
    );
}
