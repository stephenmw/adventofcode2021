use crate::lib::grid::Grid;

pub fn problem1(input: &str) -> String {
    let mut grid = parser::parse(input).unwrap().1;
    let ans: usize = (0..100).map(|_| step(&mut grid)).sum();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let mut grid = parser::parse(input).unwrap().1;
    let (lx, ly) = grid.size();
    let num_octopuses = lx * ly;

    let mut count = 0;
    while step(&mut grid) != num_octopuses {
        count += 1;
    }
    count += 1;
    format!("{}", count)
}

fn step(grid: &mut Grid<u32>) -> usize {
    let mut stack = Vec::new();
    let mut flashes = 0;

    for p in grid.iter() {
        if let Some(x) = grid.get_mut(p) {
            *x += 1;
            if *x >= 10 {
                *x = 0;
                stack.extend(p.neighbors8());
                flashes += 1;
            }
        }
    }

    while let Some(p) = stack.pop() {
        if let Some(x) = grid.get_mut(p) {
            if *x == 0 {
                // already flashed
                continue;
            }
            *x += 1;
            if *x >= 10 {
                *x = 0;
                stack.extend(p.neighbors8());
                flashes += 1;
            }
        }
    }

    flashes
}

mod parser {
    use crate::lib::combinators::*;
    use crate::lib::grid::Grid;

    pub fn parse(input: &str) -> IResult<&str, Grid<u32>> {
        let cell = map_res(one_of("123456789"), |c| c.to_digit(10).ok_or(()));
        let row = many1(cell);
        let rows = separated_list1(line_ending, row);
        let grid = map(rows, |x| x.into());
        complete(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "1656")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "195")
    }
}
