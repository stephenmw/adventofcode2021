use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

type Solution = &'static dyn Fn(&str) -> String;

fn init_solutions() -> HashMap<(usize, usize), Solution> {
    let mut ret = HashMap::new();
    ret.insert((1, 1), &day01::problem1 as Solution);
    ret.insert((1, 2), &day01::problem2 as Solution);
    ret.insert((2, 1), &day02::problem1 as Solution);
    ret.insert((2, 2), &day02::problem2 as Solution);
    ret.insert((3, 1), &day03::problem1 as Solution);
    ret.insert((3, 2), &day03::problem2 as Solution);
    ret.insert((4, 1), &day04::problem1 as Solution);
    ret.insert((4, 2), &day04::problem2 as Solution);
    ret.insert((6, 1), &day06::problem1 as Solution);
    ret.insert((6, 2), &day06::problem2 as Solution);
    ret.insert((12, 1), &day12::problem1 as Solution);
    ret.insert((12, 2), &day12::problem2 as Solution);
    ret.insert((13, 1), &day13::problem1 as Solution);
    ret.insert((13, 2), &day13::problem2 as Solution);
    ret.insert((14, 1), &day14::problem1 as Solution);
    ret.insert((14, 2), &day14::problem2 as Solution);
    ret.insert((15, 1), &day15::problem1 as Solution);
    ret.insert((15, 2), &day15::problem2 as Solution);
    ret.insert((16, 1), &day16::problem1 as Solution);
    ret.insert((16, 2), &day16::problem2 as Solution);
    ret
}

pub fn run(day: usize, problem: usize, input: &str) -> Option<String> {
    let solutions = init_solutions();
    solutions.get(&(day, problem)).map(|&f| f(input))
}
