use itertools::Itertools;

#[derive(Debug, PartialEq)]
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

pub fn part1(lines: Vec<String>) -> Option<i128> {
    let sewer = Sewer::new(lines);
    vec![Tile::EW, Tile::NE, Tile::NS]
        .iter()
        .filter_map(|x| sewer.solve(x))
        .map(|x| (x.len() / 2) as i128)
        .take(1)
        .nth(0)

    // println!("west {:?}\n====\n", sewer.solve(Tile::EW));
    // println!("east {:?}\n====\n", sewer.solve(Tile::NE));
    // println!("north {:?}\n====\n", sewer.solve(Tile::NS));
    // None
}

pub fn part2(lines: Vec<String>) -> Option<i128> {
    let sewer = Sewer::new(lines);
    let res = vec![Tile::EW, Tile::NE, Tile::NS]
        .iter()
        .filter_map(|x| sewer.solve(x))
        .take(1)
        .nth(0)
        .unwrap();
    println!(
        "bbox: ({}, {}), ({}, {})",
        res.iter().min_by_key(|v| v.0).unwrap().0,
        res.iter().min_by_key(|v| v.1).unwrap().1,
        res.iter().max_by_key(|v| v.0).unwrap().0,
        res.iter().max_by_key(|v| v.1).unwrap().1,
    );
    None
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
    let lines = "".lines().map(String::from).collect();
    assert_eq!(part2(lines), None);
}
