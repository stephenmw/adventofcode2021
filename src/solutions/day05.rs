use crate::lib::grid::Point;

pub fn problem1(input: &str) -> String {
    let data = parser::parse(input).unwrap().1;
    let non_diag = data.iter().filter(|(a, b)| a.x == b.x || a.y == b.y);
    let ans = find_overlap(non_diag);

    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let data = parser::parse(input).unwrap().1;
    let ans = find_overlap(&data);

    format!("{}", ans)
}

fn find_overlap<'a, I>(data: I) -> usize
where
    I: IntoIterator<Item = &'a (Point, Point)>,
{
    let mut grid = GrowableGrid::default();
    data.into_iter().for_each(|&(a, b)| grid.apply_line(a, b));

    grid.cells
        .iter()
        .flat_map(|r| r.iter())
        .filter(|&&c| c >= 2)
        .count()
}

#[derive(Clone, Default)]
struct GrowableGrid {
    cells: Vec<Vec<u32>>,
}

impl GrowableGrid {
    fn get_mut_or_default(&mut self, p: Point) -> &mut u32 {
        if p.y >= self.cells.len() {
            self.cells.resize(p.y + 1, Vec::new());
        }

        let row = &mut self.cells[p.y];

        if p.x >= row.len() {
            row.resize(p.x + 1, 0);
        }

        &mut row[p.x]
    }

    fn apply_line(&mut self, a: Point, b: Point) {
        let (dx, dy) = reduce_fraction(b.x as isize - a.x as isize, b.y as isize - a.y as isize);

        let mut p = a;
        while p != b {
            *self.get_mut_or_default(p) += 1;
            p = Point::new(
                (p.x as isize + dx).try_into().unwrap(),
                (p.y as isize + dy).try_into().unwrap(),
            );
        }
        *self.get_mut_or_default(p) += 1;
    }
}

// Reduces the fraction while keeping all negatives.
fn reduce_fraction(a: isize, b: isize) -> (isize, isize) {
    fn gcd(a: usize, b: usize) -> usize {
        let (a, b) = (a.max(b), a.min(b));
        if b == 0 {
            return a;
        }

        gcd(b, a % b)
    }

    let g = gcd(a.abs() as usize, b.abs() as usize);
    (a / g as isize, b / g as isize)
}

mod parser {
    use super::*;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<(Point, Point)>> {
        let point = || {
            map(separated_pair(uint, tag(","), uint), |(x, y)| {
                Point::new(x, y)
            })
        };

        let line = separated_pair(point(), tag(" -> "), point());
        let parser = separated_list1(line_ending, line);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "5")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "12")
    }
}
