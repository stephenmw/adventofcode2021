use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

pub fn problem1(input: &str) -> String {
    let data = parser::parse(input).unwrap().1;
    let grid = Grid::new(data);
    let start = Point::new(0, 0);
    let (x_len, y_len) = grid.size();
    let end = Point::new(x_len - 1, y_len - 1);
    let ans = least_cost_path(start, end, |p| grid.get(p));
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let data = parser::parse(input).unwrap().1;
    let grid = Grid::new(data);
    let start = Point::new(0, 0);
    let (x_len, y_len) = grid.size();
    let end = Point::new(x_len * 5 - 1, y_len * 5 - 1);

    let ans = least_cost_path(start, end, |p| {
        let ref_p = Point::new(p.x % x_len, p.y % y_len);
        let x_offset = p.x / x_len;
        let y_offset = p.y / y_len;
        if x_offset >= 5 || y_offset >= 5 {
            return None;
        }
        let raw = grid.get(ref_p).unwrap(); // ref_p is always in bounds
        let mut ret = raw + y_offset as u32 + x_offset as u32;
        if ret > 9 {
            ret -= 9;
        }

        Some(ret)
    });

    format!("{}", ans)
}

fn least_cost_path<F>(start: Point, end: Point, costfn: F) -> u32
where
    F: Fn(Point) -> Option<u32>,
{
    let mut seen = HashSet::new();
    seen.insert(start);
    let mut frontier = BinaryHeap::new();

    for p in Direction::iter().filter_map(|d| start.next(d)) {
        frontier.push(Reverse((0, p)));
    }

    while let Some(Reverse((cost, cur))) = frontier.pop() {
        if !seen.insert(cur) {
            // Don't process a node twice
            continue;
        }

        let new_cost = {
            let v = match costfn(cur) {
                Some(x) => x,
                None => continue, // invalid point
            };
            cost + v
        };

        if cur == end {
            return new_cost;
        }

        for p in Direction::iter().filter_map(|d| cur.next(d)) {
            frontier.push(Reverse((new_cost, p)));
        }
    }

    panic!("least_cost_path: no valid path");
}

struct Grid {
    data: Vec<Vec<u32>>,
}

impl Grid {
    fn new(data: Vec<Vec<u32>>) -> Self {
        assert!(!data.is_empty() && !data[0].is_empty());
        Grid { data: data }
    }

    fn get(&self, p: Point) -> Option<u32> {
        self.data.get(p.y)?.get(p.x).copied()
    }

    fn size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x: x, y: y }
    }

    fn next(&self, d: Direction) -> Option<Point> {
        let p = match d {
            Direction::Up => Point::new(self.x, self.y.checked_sub(1)?),
            Direction::Down => Point::new(self.x, self.y.checked_add(1)?),
            Direction::Left => Point::new(self.x.checked_sub(1)?, self.y),
            Direction::Right => Point::new(self.x.checked_add(1)?, self.y),
        };
        Some(p)
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .into_iter()
    }
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
        let num = map_res(one_of("0123456789"), |x: char| x.to_digit(10).ok_or(()));
        let row = many1(num);
        let matrix = separated_list1(line_ending, row);
        complete(matrix)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "40")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "315")
    }
}
