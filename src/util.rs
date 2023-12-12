pub fn parse_space_separated_str(s: &str) -> Vec<i128> {
    s.split(' ').flat_map(|s| s.parse().ok()).collect()
}

pub struct Vec2D<T> {
    pub tiles: Vec<T>,
    pub shape: (usize, usize),
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Column<T> {
    tiles: Vec<T>,
    index: usize,
}

struct Row<T> {
    tiles: Vec<T>,
    index: usize,
}

impl<T> Vec2D<T> {
    pub fn get(&self, position: &(usize, usize)) -> Option<&T> {
        if position.0 < self.shape.0 && position.1 < self.shape.1 {
            Some(&self.tiles[position.0 * self.shape.1 + position.1])
        } else {
            None
        }
    }

    pub fn rows(&self) -> Vec<Vec<(usize, usize)>> {
        let mut res = vec![];
        for x in 0..self.shape.0 {
            let mut row = vec![];
            for y in 0..self.shape.1 {
                row.push((x, y))
            }
            res.push(row)
        }
        res
    }

    pub fn columns(&self) -> Vec<Vec<(usize, usize)>> {
        let mut res = vec![];
        for x in 0..self.shape.1 {
            let mut column = vec![];
            for y in 0..self.shape.0 {
                column.push((y, x));
            }
            res.push(column);
        }
        res
    }

    pub fn advance(
        &self,
        position: (usize, usize),
        direction: Direction,
    ) -> Option<(usize, usize)> {
        match direction {
            Direction::Up if position.0 > 0 => Some((position.0 - 1, position.1)),
            Direction::Down if position.0 < self.shape.0 - 1 => Some((position.0 + 1, position.1)),
            Direction::Left if position.1 > 0 => Some((position.0, position.1 - 1)),
            Direction::Right if position.1 < self.shape.1 - 1 => Some((position.0, position.1 + 1)),
            _ => None,
        }
    }
}
