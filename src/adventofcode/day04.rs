use std::collections::HashSet;

use itertools::{iterate, Itertools};

struct LetterBox {
    characters: Vec<char>,
    dimensions: Pos,
}

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
struct Pos(i32, i32);

impl Pos {
    fn diff(&self, other: &Pos) -> Pos {
        Pos(self.0 - other.0, self.1 - other.1)
    }
}

impl LetterBox {
    fn new(lines: Vec<String>) -> Self {
        Self {
            characters: lines.iter().flat_map(|line| line.chars()).collect(),
            dimensions: Pos(lines[0].len() as i32, lines.len() as i32),
        }
    }

    fn neighbors(&self, pos: &Pos) -> HashSet<Pos> {
        let mut positions = HashSet::new();
        for x in
            (pos.0 - 1).clamp(0, self.dimensions.0 - 1)..(pos.0 + 2).clamp(0, self.dimensions.0 - 1)
        {
            for y in (pos.1 - 1).clamp(0, self.dimensions.1 - 1)
                ..(pos.1 + 2).clamp(0, self.dimensions.1 - 1)
            {
                positions.insert(Pos(x, y));
            }
        }
        positions
    }

    fn all(&self) -> Vec<Pos> {
        let mut result = vec![];
        for x in 0..self.dimensions.0 {
            for y in 0..self.dimensions.1 {
                result.push(Pos(x, y));
            }
        }
        result
    }

    fn get(&self, pos: &Pos) -> char {
        self.characters[(pos.1 * self.dimensions.0 + pos.0) as usize]
    }

    fn solve(&self, pos: &Pos) -> usize {
        self.paths(pos)
            .iter()
            .filter(|path| {
                path.iter().map(|pos| self.get(&pos)).collect_vec() == vec!['X', 'M', 'A', 'S']
                    && path
                        .iter()
                        .tuple_windows()
                        .map(|(x, y)| x.diff(y))
                        .all_equal()
            })
            .map(|path| {
                tracing::debug!("AAAAAAA {:?}", path);
                path
            })
            .count()
    }

    fn solve2(&self, a: &Pos) -> bool {
        if a.0 == 0 || a.0 == self.dimensions.0 - 1 || a.1 == 0 || a.1 == self.dimensions.1 - 1 {
            false
        } else {
            let diag1 = (
                self.get(&self.shift(&a, &Pos(-1, -1))),
                self.get(&self.shift(&a, &Pos(1, 1))),
            );
            let diag2 = (
                self.get(&self.shift(&a, &Pos(-1, 1))),
                self.get(&self.shift(&a, &Pos(1, -1))),
            );

            let result = (diag1 == ('M', 'S') || diag1 == ('S', 'M'))
                && (diag2 == ('M', 'S') || diag2 == ('S', 'M'));
            tracing::debug!("{:?} {}", a, result);
            result
        }
    }

    fn paths(&self, x: &Pos) -> HashSet<Vec<Pos>> {
        let mut result = HashSet::new();
        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        for direction in directions {
            let path = iterate(*x, |p| self.shift(p, &Pos(direction.0, direction.1)))
                .take(4)
                .collect_vec();
            result.insert(path);
        }
        result
    }

    fn shift(&self, pos: &Pos, direction: &Pos) -> Pos {
        Pos(
            (pos.0 + direction.0).clamp(0, self.dimensions.0 - 1),
            (pos.1 + direction.1).clamp(0, self.dimensions.1 - 1),
        )
    }
}

pub fn part1<T: ToString>(lines: Vec<T>) -> Option<usize> {
    let letterbox = LetterBox::new(lines.iter().map(|l| l.to_string()).collect());
    Some(
        letterbox
            .all()
            .iter()
            .filter(|&x| letterbox.get(x) == 'X')
            .map(|p| letterbox.solve(p))
            .sum(),
    )
}

pub fn part2<T: ToString>(lines: Vec<T>) -> Option<usize> {
    let letterbox = LetterBox::new(lines.iter().map(|l| l.to_string()).collect());
    Some(
        letterbox
            .all()
            .iter()
            .filter(|&x| letterbox.get(x) == 'A')
            .filter(|p| letterbox.solve2(p))
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use tracing_test::traced_test;

    #[test]
    fn part1() {
        let lines: Vec<&str> = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let result = super::part1(lines);
        assert_eq!(result, Some(18));
    }

    #[traced_test]
    #[test]
    fn part2() {
        let lines: Vec<&str> = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];
        let result = super::part2(lines);
        assert_eq!(result, Some(9))
    }
}
