use std::collections::HashMap;

mod day01;
mod day12;

type Solution = &'static dyn Fn(&str) -> String;

fn init_solutions() -> HashMap<(usize, usize), Solution> {
    let mut ret = HashMap::new();
    ret.insert((1, 1), &day01::problem1 as Solution);
    ret.insert((1, 2), &day01::problem2 as Solution);
    ret.insert((12, 1), &day12::problem1 as Solution);
    ret.insert((12, 2), &day12::problem2 as Solution);
    ret
}

pub fn run(day: usize, problem: usize, input: &str) -> Option<String> {
    let solutions = init_solutions();
    solutions.get(&(day, problem)).map(|&f| f(input))
}
