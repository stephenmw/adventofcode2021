use std::collections::HashMap;

mod day01;
mod day02;
mod day03;
mod day04;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day25;

type Solution = &'static dyn Fn(&str) -> String;

fn init_solutions() -> HashMap<(usize, usize), Solution> {
    let mut ret = HashMap::new();

    macro_rules! day {
        ($x:expr,$y:tt) => {
            ret.insert(($x, 1), &$y::problem1 as Solution);
            ret.insert(($x, 2), &$y::problem2 as Solution);
        };
    }

    day!(1, day01);
    day!(2, day02);
    day!(3, day03);
    day!(4, day04);
    day!(6, day06);
    day!(7, day07);
    day!(8, day08);
    day!(9, day09);
    day!(10, day10);
    day!(11, day11);
    day!(12, day12);
    day!(13, day13);
    day!(14, day14);
    day!(15, day15);
    day!(16, day16);
    day!(17, day17);
    day!(18, day18);
    day!(19, day19);
    day!(20, day20);
    day!(21, day21);
    day!(22, day22);
    day!(25, day25);

    ret
}

pub fn run(day: usize, problem: usize, input: &str) -> Option<String> {
    let solutions = init_solutions();
    solutions.get(&(day, problem)).map(|&f| f(input))
}
