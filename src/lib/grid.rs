#[derive(Clone, Debug)]
pub struct Grid<T> {
    cells: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        assert!(!data.is_empty() && !data[0].is_empty());
        Grid { cells: data }
    }

    pub fn get(&self, p: Point) -> Option<&T> {
        self.cells.get(p.y)?.get(p.x)
    }

    pub fn get_mut(&mut self, p: Point) -> Option<&mut T> {
        self.cells.get_mut(p.y)?.get_mut(p.x)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.cells[0].len(), self.cells.len())
    }

    pub fn iter(&self) -> impl Iterator<Item = Point> {
        let (x_len, y_len) = self.size();
        (0..y_len).flat_map(move |y| (0..x_len).map(move |x| Point::new(x, y)))
    }

    pub fn neighbor_wrapping(&self, p: Point, d: Direction) -> Point {
        let (x_len, y_len) = self.size();

        match d {
            Direction::Up => Point::new(p.x, (p.y + 1) % y_len),
            Direction::Down => Point::new(p.x, p.y.checked_sub(1).unwrap_or(y_len - 1)),
            Direction::Left => Point::new(p.x.checked_sub(1).unwrap_or(x_len - 1), p.y),
            Direction::Right => Point::new((p.x + 1) % x_len, p.y),
        }
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(cells: Vec<Vec<T>>) -> Self {
        Self::new(cells)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Point { x: x, y: y }
    }

    pub fn next(&self, d: Direction) -> Option<Point> {
        let p = match d {
            Direction::Up => Point::new(self.x, self.y.checked_add(1)?),
            Direction::Down => Point::new(self.x, self.y.checked_sub(1)?),
            Direction::Left => Point::new(self.x.checked_sub(1)?, self.y),
            Direction::Right => Point::new(self.x.checked_add(1)?, self.y),
        };

        Some(p)
    }

    pub fn neighbors<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        Direction::iter().filter_map(|d| self.next(d))
    }

    pub fn neighbors8<'a>(&'a self) -> impl Iterator<Item = Point> + 'a {
        fn apply_offset(p: &Point, dx: isize, dy: isize) -> Option<Point> {
            Some(Point {
                x: (p.x as isize + dx).try_into().ok()?,
                y: (p.y as isize + dy).try_into().ok()?,
            })
        }

        let offsets = (-1..=1).flat_map(|y| (-1..=1).map(move |x| (x, y)));
        offsets.filter_map(|(dx, dy)| apply_offset(self, dx, dy))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}
