use std::str::FromStr;

pub fn problem1(input: &str) -> String {
    let commands = parser::parse(input).unwrap().1;
    let mut x = 0;
    let mut y = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Forward => x += command.distance,
            Direction::Down => y += command.distance,
            Direction::Up => y -= command.distance,
        };
    }

    format!("{}", x * y)
}

pub fn problem2(input: &str) -> String {
    let commands = parser::parse(input).unwrap().1;
    let mut x = 0;
    let mut y = 0;
    let mut aim = 0;
    for command in commands.iter() {
        match command.direction {
            Direction::Forward => {
                x += command.distance;
                y += command.distance * aim;
            }
            Direction::Down => aim += command.distance,
            Direction::Up => aim -= command.distance,
        };
    }

    format!("{}", x * y)
}

#[derive(Clone, Copy, Debug)]
pub struct Command {
    pub direction: Direction,
    pub distance: i32,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Forward,
    Down,
    Up,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "forward" => Ok(Self::Forward),
            "down" => Ok(Self::Down),
            "up" => Ok(Self::Up),
            _ => Err(()),
        }
    }
}

mod parser {
    use super::Command;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Command>> {
        let distance = uint::<i32>;
        let direction = map_res(take_while(is_alphabetic), |x: &str| x.parse());
        let command = map(
            separated_pair(direction, tag(" "), distance),
            |(dir, dist)| Command {
                direction: dir,
                distance: dist,
            },
        );
        let mut parser = separated_list1(line_ending, command);
        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "150")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "900")
    }
}
