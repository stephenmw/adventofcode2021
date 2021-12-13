pub fn problem1(input: &str) -> String {
    let (dots, folds) = parser::parse(input).unwrap().1;
    let mut paper = Paper::new(dots);
    paper.fold(folds[0]);
    format!("{}", paper.dots.len())
}

pub fn problem2(input: &str) -> String {
    let (dots, folds) = parser::parse(input).unwrap().1;
    let mut paper = Paper::new(dots);

    for fold in folds.iter().cloned() {
        paper.fold(fold);
    }

    paper.render()
}

struct Paper {
    dots: Vec<Coordinate>,
    max_x: usize,
    max_y: usize,
}

impl Paper {
    fn new(dots: Vec<Coordinate>) -> Self {
        let max_x = dots.iter().map(|c| c.x).max().unwrap();
        let max_y = dots.iter().map(|c| c.y).max().unwrap();

        Paper {
            dots: dots,
            max_x: max_x,
            max_y: max_y,
        }
    }

    fn fold(&mut self, f: Fold) {
        match f.axis {
            'x' => {
                for c in self.dots.iter_mut() {
                    if c.x > f.loc {
                        c.x = self.max_x - c.x;
                    }
                }
                self.max_x = f.loc - 1;
            }
            'y' => {
                for c in self.dots.iter_mut() {
                    if c.y > f.loc {
                        c.y = self.max_y - c.y;
                    }
                }
                self.max_y = f.loc - 1;
            }
            _ => panic!("bad axis"),
        }

        self.dots.sort();
        self.dots.dedup();
    }

    fn render(&self) -> String {
        let mut table = Vec::new();
        table.resize(self.max_y + 1, vec!['.'; self.max_x + 1]);

        for c in self.dots.iter() {
            table[c.y][c.x] = '#'
        }

        let mut ret = String::new();
        for row in table.iter() {
            for &ch in row {
                ret.push(ch);
            }
            ret.push('\n');
        }

        ret
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct Fold {
    pub axis: char,
    pub loc: usize,
}

mod parser {
    use super::{Coordinate, Fold};
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Coordinate>, Vec<Fold>)> {
        let coordinate = map(
            separated_pair(uint_parser::<usize>, tag(","), uint_parser::<usize>),
            |(x, y)| Coordinate { x: x, y: y },
        );
        let fold = map(
            preceded(
                tag("fold along "),
                separated_pair(one_of("xy"), tag("="), uint_parser::<usize>),
            ),
            |(a, l)| Fold { axis: a, loc: l },
        );
        let coordinate_list = separated_list1(line_ending, coordinate);
        let fold_list = separated_list1(line_ending, fold);
        let mut parser = separated_pair(
            coordinate_list,
            tuple((line_ending, line_ending)),
            fold_list,
        );
        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "17")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(
            problem2(EXAMPLE_INPUT),
            "#####\n#...#\n#...#\n#...#\n#####\n.....\n.....\n"
        )
    }
}
