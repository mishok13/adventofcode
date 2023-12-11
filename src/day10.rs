use std::collections::HashSet;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    NS,
    EW,
    NW,
    NE,
    SW,
    SE,
    Start,
    Ground,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

struct Sewer {
    tiles: Vec<Tile>,
    shape: (usize, usize),
}

// struct Position {
//     row: usize,
//     col: usize,
// }

// impl Position {
//     fn new(row: usize, col: usize) -> Self {
//         Self { row, col }
//     }
// }

impl Sewer {
    fn new(lines: Vec<String>) -> Self {
        let shape = (lines.len(), lines[0].len());

        let tiles: Vec<_> = lines
            .iter()
            .flat_map(|line| {
                line.chars().map(|c| match c {
                    'F' => Tile::SE,
                    'J' => Tile::NW,
                    'L' => Tile::NE,
                    '7' => Tile::SW,
                    '|' => Tile::NS,
                    '-' => Tile::EW,
                    'S' => Tile::Start,
                    _ => Tile::Ground,
                })
            })
            .collect();
        Self { tiles, shape }
    }

    fn compute_position(&self, index: usize) -> (usize, usize) {
        (index / self.shape.1, index % self.shape.1)
    }

    fn solve(&self, start_as: &Tile) -> Option<Vec<(usize, usize)>> {
        println!("\n---\nSolving for {:?}", start_as);
        let start = self.compute_position(
            self.tiles
                .iter()
                .find_position(|&x| *x == Tile::Start)
                .unwrap()
                .0,
        );
        let direction = match start_as {
            Tile::EW | Tile::NW | Tile::SW => Direction::West,
            Tile::NE | Tile::SE => Direction::East,
            Tile::NS => Direction::North,
            Tile::Start | Tile::Ground => panic!("Invalid value"),
        };
        let mut path = vec![];
        let mut is_loop = false;
        let mut step = Some((direction, start));
        while let Some((direction, position)) = step {
            if !path.is_empty() && position == start {
                is_loop = true;
                break;
            }
            path.push(position);
            step = self.advance(&direction, position);
        }
        if is_loop {
            println!("{:?} {}", path, path.len() / 2);
            Some(path)
        } else {
            None
        }
    }

    fn advance(
        &self,
        direction: &Direction,
        position: (usize, usize),
    ) -> Option<(Direction, (usize, usize))> {
        // println!("Advancing {:?} from {:?}", direction, position);
        match (direction, position) {
            (&Direction::West, (row, col)) if col > 0 => Some((
                direction,
                &self.tiles[row * self.shape.1 + col - 1],
                (row, col - 1),
            )),
            (&Direction::East, (row, col)) if col < self.shape.1 => Some((
                direction,
                &self.tiles[row * self.shape.1 + col + 1],
                (row, col + 1),
            )),
            (&Direction::North, (row, col)) if row > 0 => Some((
                direction,
                &self.tiles[(row - 1) * self.shape.1 + col],
                (row - 1, col),
            )),
            (&Direction::South, (row, col)) if row < self.shape.0 => Some((
                direction,
                &self.tiles[(row + 1) * self.shape.1 + col],
                (row + 1, col),
            )),
            _ => None,
        }
        .and_then(|(direction, tile, position)| match (direction, tile) {
            (_, Tile::Ground) => None,
            (_, Tile::Start) => Some((Direction::East, position)),
            (Direction::East, Tile::EW) => Some((Direction::East, position)),
            (Direction::East, Tile::NW) => Some((Direction::North, position)),
            (Direction::East, Tile::SW) => Some((Direction::South, position)),
            (Direction::West, Tile::EW) => Some((Direction::West, position)),
            (Direction::West, Tile::NE) => Some((Direction::North, position)),
            (Direction::West, Tile::SE) => Some((Direction::South, position)),
            (Direction::North, Tile::SW) => Some((Direction::West, position)),
            (Direction::North, Tile::SE) => Some((Direction::East, position)),
            (Direction::North, Tile::NS) => Some((Direction::North, position)),
            (Direction::South, Tile::NW) => Some((Direction::West, position)),
            (Direction::South, Tile::NE) => Some((Direction::East, position)),
            (Direction::South, Tile::NS) => Some((Direction::South, position)),
            _ => None,
        })
    }
}

#[derive(Debug)]
enum State {
    Outside,
    Inside,
    InsideEdge(Tile),
    OutsideEdge(Tile),
}

pub fn part1(lines: Vec<String>) -> Option<i128> {
    let sewer = Sewer::new(lines);
    vec![Tile::EW, Tile::NE, Tile::NS]
        .iter()
        .filter_map(|x| sewer.solve(x))
        .map(|x| (x.len() / 2) as i128)
        .nth(0)

    // println!("west {:?}\n====\n", sewer.solve(Tile::EW));
    // println!("east {:?}\n====\n", sewer.solve(Tile::NE));
    // println!("north {:?}\n====\n", sewer.solve(Tile::NS));
    // None
}

pub fn part2(lines: Vec<String>) -> Option<i128> {
    let mut sewer = Sewer::new(lines);
    let (direction, solution) = vec![Tile::SW, Tile::NE, Tile::NS]
        .iter()
        .filter_map(|x| sewer.solve(x).map(|solution| (x.clone(), solution)))
        .nth(0)
        .unwrap();
    let solution_lookup: HashSet<_> = solution.iter().collect();
    println!("wtf {:?} {:?}", direction.clone(), solution[0]);

    sewer.tiles[solution[0].0 * sewer.shape.1 + solution[0].1] = direction;
    let mut inside_count = 0;
    for row_num in 0..sewer.shape.0 {
        let mut state = State::Outside;
        for col_num in 0..sewer.shape.1 {
            let pos = (row_num, col_num);
            // println!(
            //     "next {:?} {:?} {}",
            //     pos,
            //     &sewer.tiles[row_num * sewer.shape.1 + col_num],
            //     solution_lookup.contains(&pos),
            // );
            match (
                &sewer.tiles[row_num * sewer.shape.1 + col_num],
                &state,
                solution_lookup.contains(&pos),
            ) {
                (_, State::Inside, false) => {
                    println!("Counting tile at {pos:?}");
                    inside_count += 1;
                }
                (_, State::Outside, false) => {}
                (tile @ (Tile::SE | Tile::NE), State::Inside, true) => {
                    state = State::InsideEdge(tile.clone())
                }
                (tile @ (Tile::SE | Tile::NE), State::Outside, true) => {
                    state = State::OutsideEdge(tile.clone())
                }
                (Tile::NW, State::InsideEdge(Tile::NE), true) => state = State::Inside,
                (Tile::NW, State::OutsideEdge(Tile::NE), true) => state = State::Outside,

                (Tile::NW, State::InsideEdge(Tile::SE), true) => state = State::Outside,
                (Tile::NW, State::OutsideEdge(Tile::SE), true) => state = State::Inside,

                (Tile::SW, State::OutsideEdge(Tile::NE), true) => state = State::Inside,
                (Tile::SW, State::InsideEdge(Tile::NE), true) => state = State::Outside,

                (Tile::SW, State::OutsideEdge(Tile::SE), true) => state = State::Outside,
                (Tile::SW, State::InsideEdge(Tile::SE), true) => state = State::Inside,

                (Tile::EW, State::InsideEdge(_) | State::OutsideEdge(_), true) => {}
                (Tile::NS, State::Inside, true) => state = State::Outside,
                (Tile::NS, State::Outside, true) => state = State::Inside,
                _ => todo!(),
            }
            println!("{:?} {:?} {}", pos, state, solution_lookup.contains(&pos));
        }
    }
    Some(inside_count)
}

#[test]
fn test_part1() {
    {
        let lines = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part1(lines), Some(8));
    }
    {
        let lines = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part1(lines), Some(8));
    }
    {
        let lines = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part1(lines), Some(4));
    }
}

#[test]
fn test_part2() {
    {
        let lines = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part2(lines), Some(8));
    }
    {
        let lines = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            .lines()
            .map(String::from)
            .collect();
        assert_eq!(part2(lines), Some(10));
    }
}
