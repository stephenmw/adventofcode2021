use crate::lib::grid::{Grid, Point};

use std::collections::HashSet;

pub fn problem1(input: &str) -> String {
    let grid = parser::parse(input).unwrap().1;
    let ans: u32 = grid
        .iter()
        .filter(|x| is_local_minima(&grid, *x))
        .filter_map(|p| grid.get(p).copied().map(|x| x + 1))
        .sum();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let grid = parser::parse(input).unwrap().1;

    let mut seen = HashSet::new();
    let mut basin_sizes: Vec<_> = grid
        .iter()
        .map(|p| basin_size(&grid, p, &mut seen))
        .filter(|&x| x != 0)
        .collect();
    basin_sizes.sort();

    let ans: usize = basin_sizes.iter().rev().take(3).product();
    format!("{}", ans)
}

fn is_local_minima(grid: &Grid<u32>, p: Point) -> bool {
    let val = match grid.get(p).copied() {
        Some(x) => x,
        None => return false,
    };
    p.neighbors()
        .filter_map(|x| grid.get(x).copied())
        .all(|x| x > val)
}

fn basin_size(grid: &Grid<u32>, p: Point, seen: &mut HashSet<Point>) -> usize {
    let mut count = 0;
    let mut frontier = Vec::new();

    if !seen.contains(&p) {
        frontier.push(p);
    }

    while let Some(cur) = frontier.pop() {
        if let Some(val) = grid.get(cur).copied() {
            if seen.insert(cur) {
                if val == 9 {
                    continue;
                }
                count += 1;
                frontier.extend(cur.neighbors());
            }
        }
    }

    count
}

mod parser {
    use crate::lib::combinators::*;
    use crate::lib::grid::Grid;

    pub fn parse(input: &str) -> IResult<&str, Grid<u32>> {
        let cell = map_res(one_of("1234567890"), |c| c.to_digit(10).ok_or(()));
        let row = many1(cell);
        let rows = separated_list1(line_ending, row);
        let grid = map(rows, |x| x.into());
        complete(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "15")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "1134")
    }
}
