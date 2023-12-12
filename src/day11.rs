use itertools::{repeat_n, Itertools};

use crate::util::Vec2D;

enum Tile {
    Empty,
    Full,
}

// struct Tile {
//     value: TileValue,
// }

impl Vec2D<Tile> {
    fn new(lines: Vec<String>) -> Self {
        Self {
            tiles: lines
                .iter()
                .flat_map(|line| {
                    line.chars().map(|c| match c {
                        '#' => Tile::Full,
                        '.' => Tile::Empty,
                        _ => todo!(),
                    })
                })
                .collect(),
            shape: (lines.len(), lines[0].len()),
        }
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.rows()
            .iter()
            .enumerate()
            .filter(|(index, row)| {
                row.iter().map(|pos| self.get(pos)).all(|x| match x {
                    Some(Tile::Empty) => true,
                    _ => false,
                })
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn empty_columns(&self) -> Vec<usize> {
        self.columns()
            .iter()
            .enumerate()
            .filter(|(index, column)| {
                column.iter().map(|pos| self.get(pos)).all(|x| match x {
                    Some(Tile::Empty) => true,
                    _ => false,
                })
            })
            .map(|(index, _)| index)
            .collect()
    }

    fn old(&mut self) {
        let mut empty_rows = self.empty_rows();
        let mut empty_columns = self.empty_columns();

        for (index, &row_number) in empty_rows.iter().enumerate() {
            for _ in 0..self.shape.1 {
                let position = (index + row_number) * self.shape.1;
                self.tiles.insert(position, Tile::Empty);
            }
        }
        self.shape.0 += empty_rows.len();

        for (index, &column_number) in empty_columns.iter().enumerate() {
            for row_number in 0..self.shape.0 {
                let position = row_number * (1 + self.shape.1) + column_number + index;
                self.tiles.insert(position, Tile::Empty);
            }
            self.shape.1 += 1;
        }
    }

    fn distance(&self, from: (usize, usize), to: (usize, usize), multiplier: usize) -> usize {
        let empty_columns = self
            .empty_columns()
            .iter()
            .filter(|&index| *index > from.1.min(to.1) && *index < from.1.max(to.1))
            .count();
        let empty_rows = self
            .empty_rows()
            .iter()
            .filter(|&index| *index > from.0.min(to.0) && *index < from.0.max(to.0))
            .count();
        from.0.max(to.0) - from.0.min(to.0) + from.1.max(to.1) - from.1.min(to.1)
            + empty_rows * multiplier
            + empty_columns * multiplier
    }

    fn print(&self) {
        println!(
            "Shape: {}x{}; items: {:?}",
            self.shape.0,
            self.shape.1,
            self.tiles.len()
        );
        println!("{:?} {:?}", self.empty_rows(), self.empty_columns());
        for row in self.rows() {
            for tile in row {
                match self.get(&tile) {
                    Some(Tile::Empty) => print!("."),
                    Some(Tile::Full) => print!("X"),
                    _ => panic!("wtf"),
                }
            }
            print!("\n");
        }
    }

    fn positions(&self) -> Vec<(usize, usize)> {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| match t {
                Tile::Full => true,
                _ => false,
            })
            .map(|(index, _)| (index / self.shape.1, index % self.shape.1))
            .collect()
    }
}

fn distance(pos1: (usize, usize), pos2: (usize, usize)) -> usize {
    pos1.0.max(pos2.0) - pos1.0.min(pos2.0) + pos1.1.max(pos2.1) - pos1.1.min(pos2.1)
}

pub fn part1(lines: Vec<String>) -> Option<usize> {
    solve(lines, 1)
}

fn solve(lines: Vec<String>, multiplier: usize) -> Option<usize> {
    let map = Vec2D::new(lines);
    map.print();
    map.positions()
        .into_iter()
        .combinations(2)
        .map(|v| map.distance(v[0], v[1], multiplier))
        .reduce(|a, b| a + b)
}

pub fn part2(lines: Vec<String>) -> Option<usize> {
    solve(lines, 999999)
}

#[test]
fn test_part1() {
    {
        let lines: Vec<_> = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(solve(lines.clone(), 1), Some(374));
        assert_eq!(solve(lines.clone(), 9), Some(1030));
        assert_eq!(solve(lines.clone(), 99), Some(8410));
    }
}

// #[test]
// fn test_part2() {
//     let lines = "".lines().map(String::from).collect();
//     assert_eq!(part2(lines), None);
// }
