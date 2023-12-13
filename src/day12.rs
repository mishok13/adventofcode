use std::{collections::HashSet, iter};

fn parse_numbers(s: &str) -> Vec<usize> {
    s.split(',').map(|part| part.parse().unwrap()).collect()
}

#[derive(Debug)]
enum Symbol {
    Dot,
    Hash,
    Either,
}

fn parse_pattern(s: &str) -> Vec<Symbol> {
    println!("Parsing {s}");
    let mut pattern: Vec<_> = s
        .chars()
        .map(|c| match c {
            '#' => Symbol::Hash,
            '.' => Symbol::Dot,
            '?' => Symbol::Either,
            _ => todo!(),
        })
        .collect();
    pattern.insert(0, Symbol::Dot);
    pattern.push(Symbol::Dot);
    pattern
}

fn find_matches(pattern: &[Symbol], length: &usize) -> Vec<(usize, usize)> {
    (0..(pattern.len() - length - 1))
        .filter(|&start| {
            if let (Symbol::Dot | Symbol::Either, Symbol::Dot | Symbol::Either) =
                (&pattern[start], &pattern[start + length + 1])
            {
                (start + 1..start + length + 1)
                    .map(|i| &pattern[i])
                    .all(|sym| matches!(sym, Symbol::Hash | Symbol::Either))
            } else {
                false
            }
        })
        .map(|start| (start + 1, start + length + 1))
        .collect::<Vec<_>>()
        .into()
}

fn solve(pattern: &[Symbol], lengths: &[usize]) -> Vec<Vec<(usize, usize)>> {
    let possibilities = lengths
        .iter()
        .map(|length| find_matches(pattern, length))
        .collect();
    shake(vec![], possibilities)
        .into_iter()
        .filter(|v| v.len() == lengths.len())
        .filter(|v| {
            let solved_positions = v
                .iter()
                .flat_map(|(start, end)| (*start..*end).into_iter())
                .collect::<HashSet<_>>();
            let prepopulated = pattern
                .iter()
                .enumerate()
                .filter_map(|(index, symbol)| {
                    if let Symbol::Hash = symbol {
                        Some(index)
                    } else {
                        None
                    }
                })
                .collect::<HashSet<_>>();
            prepopulated.is_subset(&solved_positions)
        })
        .collect()
}

fn shake(
    solution: Vec<(usize, usize)>,
    possibilities: Vec<Vec<(usize, usize)>>,
) -> Vec<Vec<(usize, usize)>> {
    if possibilities.is_empty() {
        vec![solution]
    } else {
        possibilities[0]
            .iter()
            .skip_while(|&x| !solution.is_empty() && x.0 <= solution[solution.len() - 1].1)
            .flat_map(|&x| {
                shake(
                    {
                        let mut s = solution.clone();
                        s.push(x);
                        s
                    },
                    possibilities[1..possibilities.len()].to_vec(),
                )
            })
            .collect()
    }
}

pub fn part1(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .map(|line| {
            line.split_once(' ')
                .map(|(pattern, numbers)| (parse_pattern(pattern), parse_numbers(numbers)))
                .unwrap()
        })
        .map(|(pattern, lengths)| solve(&pattern, &lengths))
        .map(|v| v.len())
        .reduce(|a, b| a + b)
}

fn unfold(s: &str, separator: char, times: usize) -> String {
    let length = (s.len() + 1) * times - 1;
    s.chars()
        .chain(iter::once(separator))
        .cycle()
        .take(length)
        .collect()
}

pub fn part2(lines: Vec<String>) -> Option<usize> {
    lines
        .iter()
        .map(|line| {
            line.split_once(' ')
                .map(|(pattern, numbers)| {
                    (
                        parse_pattern(&unfold(pattern, '?', 5)),
                        parse_numbers(&unfold(numbers, ',', 5)),
                    )
                })
                .unwrap()
        })
        .map(|(pattern, lengths)| {
            let solution = solve(&pattern, &lengths);
            for x in solution.iter() {
                let mut s = vec!['.'; pattern.len()];
                for pos in x {
                    for h in pos.0..pos.1 {
                        s[h] = '#';
                    }
                }
            }
            solution
        })
        .map(|v| v.len())
        .reduce(|a, b| a + b)
}

#[test]
fn test_part1() {
    {
        let lines: Vec<_> = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part1(lines), Some(21));
    }
    {
        let lines = ".##.?#??.#.?# 2,1,1,1".lines().map(String::from).collect();
        assert_eq!(part1(lines), Some(1));
    }
}

#[test]
fn test_part2() {
    {
        let lines = "????.######..#####. 1,6,5"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part2(lines), Some(2500));
    }

    {
        let lines = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part2(lines), Some(525152));
    }
}
