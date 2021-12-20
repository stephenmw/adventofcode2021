type Bit = bool;

pub fn problem1(input: &str) -> String {
    let (algorithm, mut image) = parser::parse(input).unwrap().1;
    for _ in 0..2 {
        image = enhance(&image, &algorithm);
    }

    let ans = image
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|x| **x)
        .count();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let (algorithm, mut image) = parser::parse(input).unwrap().1;
    for _ in 0..50 {
        image = enhance(&image, &algorithm);
    }

    let ans = image
        .grid
        .iter()
        .flat_map(|row| row.iter())
        .filter(|x| **x)
        .count();
    format!("{}", ans)
}

fn enhance(image: &Image, algorithm: &[Bit]) -> Image {
    let at_index = |x: i32, y: i32| {
        let bit_offsets = (-1..=1).flat_map(|y| (-1..=1).map(move |x| (x, y)));
        let mut index = 0;
        for (dx, dy) in bit_offsets {
            let bit = image.get_bit(x + dx, y + dy);
            index = (index << 1) + bit as usize;
        }
        algorithm[index]
    };

    let default = match image.default_bit {
        true => algorithm[511], // 2^9 - 1
        false => algorithm[0],
    };

    let mut grid = {
        let x_len = image.grid[0].len() + 4;
        let y_len = image.grid.len() + 4;
        let row = vec![false; x_len];
        vec![row; y_len]
    };

    for (y, row) in grid.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            *cell = at_index(x as i32 - 2, y as i32 - 2);
        }
    }

    Image {
        default_bit: default,
        grid: grid,
    }
}

pub struct Image {
    default_bit: Bit,
    grid: Vec<Vec<Bit>>,
}

impl Image {
    fn new(grid: Vec<Vec<Bit>>) -> Self {
        Image {
            default_bit: false,
            grid: grid,
        }
    }

    fn get_bit(&self, x: i32, y: i32) -> bool {
        let x: usize = match x.try_into() {
            Ok(x) => x,
            Err(_) => return self.default_bit,
        };
        let y: usize = match y.try_into() {
            Ok(y) => y,
            Err(_) => return self.default_bit,
        };

        self.grid
            .get(y)
            .and_then(|row| row.get(x))
            .cloned()
            .unwrap_or(self.default_bit)
    }
}

mod parser {
    use super::{Bit, Image};
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Bit>, Image)> {
        let bit = || {
            map(one_of(".#"), |b| match b {
                '.' => false,
                '#' => true,
                _ => panic!("not one_of '.#'"),
            })
        };
        let algorithm = count(bit(), 512);
        let row = many1(bit());
        let grid = separated_list1(line_ending, row);
        let image = map(grid, |x| Image::new(x));
        let parser = separated_pair(algorithm, tuple((line_ending, line_ending)), image);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "35")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "3351")
    }
}
