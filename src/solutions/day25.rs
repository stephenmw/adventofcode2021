use crate::lib::grid::{Direction, Grid, Point};

type SCGrid = Grid<Option<Herd>>;

pub fn problem1(input: &str) -> String {
    let mut grid = parser::parse(input).unwrap().1;

    let (i, _) = (1..)
        .map(|i| (i, step(&mut grid)))
        .find(|(_, moves)| *moves == 0)
        .unwrap();
    format!("{}", i)
}

pub fn problem2(_input: &str) -> String {
    unimplemented!();
}

fn step(g: &mut SCGrid) -> usize {
    step_herd(g, Herd::East) + step_herd(g, Herd::South)
}

fn step_herd(g: &mut SCGrid, herd: Herd) -> usize {
    let moves = find_moves(g, herd);
    let l = moves.len();
    for p in moves {
        let new_p = g.neighbor_wrapping(p, herd.direction());
        *g.get_mut(p).unwrap() = None;
        *g.get_mut(new_p).unwrap() = Some(herd);
    }
    l
}

fn find_moves(g: &SCGrid, h: Herd) -> Vec<Point> {
    g.iter()
        .filter(|&p| g.get(p) == Some(&Some(h))) // correct herd
        .filter(|&p| g.get(g.neighbor_wrapping(p, h.direction())) == Some(&None)) // next is empty
        .collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Herd {
    East,
    South,
}

impl Herd {
    fn direction(&self) -> Direction {
        match self {
            Self::East => Direction::Right,
            Self::South => Direction::Up,
        }
    }
}

mod parser {
    use super::Herd;
    use super::SCGrid;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, SCGrid> {
        let east = map(tag(">"), |_| Some(Herd::East));
        let south = map(tag("v"), |_| Some(Herd::South));
        let empty_space = map(tag("."), |_| None);
        let cell = alt((east, south, empty_space));
        let row = many1(cell);
        let rows = separated_list1(line_ending, row);
        let grid = map(rows, |x| x.into());
        complete(grid)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "58")
    }
    //#[test]
    //fn problem2_test() {
        //assert_eq!(problem2(EXAMPLE_INPUT), "")
    //}
}
