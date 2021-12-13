pub fn problem1(input: &str) -> String {
    let (drawn, mut boards) = parser::parse(input).unwrap().1;

    for &num in drawn.iter() {
        for board in boards.iter_mut() {
            if board.mark(num) {
                let ans = sum_board(board) * num;
                return format!("{}", ans);
            }
        }
    }

    "no answer".to_owned()
}

pub fn problem2(input: &str) -> String {
    let (drawn, mut boards) = parser::parse(input).unwrap().1;

    for &num in drawn.iter() {
        let mut i = 0;
        while i < boards.len() {
            if boards[i].mark(num) {
                if boards.len() == 1 {
                    let ans = sum_board(&boards[0]) * num;
                    return format!("{}", ans);
                }
                boards.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    "no answer".to_owned()
}

fn sum_board(board: &Board) -> i32 {
    board
        .values
        .iter()
        .flat_map(|xs| xs.iter())
        .filter(|x| **x != -1)
        .sum::<i32>()
}

#[derive(Debug)]
pub struct Board {
    values: Vec<Vec<i32>>,
}

impl Board {
    pub fn new(values: Vec<Vec<i32>>) -> Self {
        Board { values: values }
    }

    // returns if bingo is reached during marking
    fn mark(&mut self, num: i32) -> bool {
        for row in 0..self.values.len() {
            for column in 0..self.values[0].len() {
                if self.values[row][column] == num {
                    self.values[row][column] = -1;
                    if self.check_row(row) || self.check_column(column) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_row(&self, row: usize) -> bool {
        self.values[row].iter().all(|&x| x == -1)
    }

    fn check_column(&self, column: usize) -> bool {
        self.values.iter().map(|xs| xs[column]).all(|x| x == -1)
    }
}

mod parser {
    use super::Board;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, (Vec<i32>, Vec<Board>)> {
        let drawn = separated_list1(tag(","), uint::<i32>);
        let row = preceded(space0, separated_list1(space1, uint::<i32>));
        let board = map(separated_list1(line_ending, row), |vs| Board::new(vs));
        let boards = separated_list1(tuple((line_ending, line_ending)), board);
        let parser = separated_pair(drawn, tuple((line_ending, line_ending)), boards);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "4512")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "1924")
    }
}
