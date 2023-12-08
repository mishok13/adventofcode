use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, PartialEq, Ord, Eq)]
struct Hand {
    cards: Vec<u8>,
    bet: u64,
    type_: Type,
}

impl Hand {
    fn new(s: &str) -> Self {
        let (left, right) = s.split_once(' ').unwrap();
        let cards: Vec<_> = left
            .chars()
            .map(|c| match c {
                'A' => 12u8,
                'K' => 11u8,
                'Q' => 10u8,
                'J' => 9u8,
                'T' => 8u8,
                _ => c
                    .to_digit(10)
                    .map(|x| (x as u8).checked_sub(2u8).unwrap())
                    .unwrap(),
            })
            .collect();
        let type_ = match cards
            .iter()
            .fold(vec![0u8; 13], |mut acc, x| {
                acc[*x as usize] += 1;
                acc
            })
            .iter()
            .filter(|x| **x > 0)
            .sorted()
            .collect_vec()
            .as_slice()
        {
            [5] => Type::FiveOfAKind,
            [1, 4] => Type::FourOfAKind,
            [2, 3] => Type::FullHouse,
            [1, 1, 3] => Type::ThreeOfAKind,
            [1, 2, 2] => Type::DoublePair,
            [1, 1, 1, 2] => Type::SinglePair,
            _ => Type::HighCard,
        };
        let bet = right.parse().unwrap();
        Self { cards, bet, type_ }
    }

    fn new2(s: &str) -> Self {
        let (left, right) = s.split_once(' ').unwrap();
        let cards: Vec<_> = left
            .chars()
            .map(|c| match c {
                'A' => 12u8,
                'K' => 11u8,
                'Q' => 10u8,
                'T' => 9u8,
                'J' => 0u8,
                _ => c
                    .to_digit(10)
                    .map(|x| (x as u8).checked_sub(1u8).unwrap())
                    .unwrap(),
            })
            .collect();
        let type_ = match cards
            .iter()
            .fold(vec![0u8; 13], |mut acc, x| {
                acc[*x as usize] += 1;
                acc
            })
            .iter()
            .skip(1) // Skip joker
            .filter(|x| **x > 0)
            .sorted()
            .collect_vec()
            .as_slice()
        {
            [5] | [4] | [3] | [2] | [1] | [] => Type::FiveOfAKind,
            [1, 4] | [1, 3] | [1, 2] | [1, 1] => Type::FourOfAKind,
            [2, 3] | [2, 2] => Type::FullHouse,
            [1, 1, 3] | [1, 1, 2] | [1, 1, 1] => Type::ThreeOfAKind,
            [1, 2, 2] => Type::DoublePair,
            [1, 1, 1, 2] | [1, 1, 1, 1] => Type::SinglePair,
            _ => Type::HighCard,
        };

        let bet = right.parse().unwrap();
        Self { cards, bet, type_ }
    }
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum Type {
    HighCard,
    SinglePair,
    DoublePair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.type_.partial_cmp(&other.type_) {
            Some(Ordering::Equal) => self
                .cards
                .iter()
                .zip(other.cards.iter())
                .skip_while(|(s, o)| **s == **o)
                .map(|(s, o)| s.partial_cmp(o))
                .nth(0)
                .unwrap(),
            x => x,
        }
    }
}

pub fn part1(lines: Vec<String>) -> Option<String> {
    Some(
        lines
            .iter()
            .map(|s| Hand::new(s.as_str()))
            .sorted()
            .enumerate()
            .fold(0u64, |acc, (index, hand)| {
                acc + ((index as u64) + 1) * hand.bet
            })
            .to_string(),
    )
}

pub fn part2(lines: Vec<String>) -> Option<String> {
    Some(
        lines
            .iter()
            .map(|s| Hand::new2(s.as_str()))
            .sorted()
            .enumerate()
            .fold(0u64, |acc, (index, hand)| {
                println!("{} {:?}", index + 1, hand);
                acc + ((index as u64) + 1) * hand.bet
            })
            .to_string(),
    )
}

#[test]
fn test_part1() {
    let lines = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
        .lines()
        .map(String::from)
        .collect();
    assert_eq!(part1(lines), Some("6440".into()));
}

#[test]
fn test_part2() {
    let lines = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(
        part2(lines.lines().map(String::from).collect()),
        Some("5905".into())
    );
}
