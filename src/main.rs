mod lib;
mod solutions;

use lib::io::load_puzzle_input;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();
    let day: usize = args
        .next()
        .expect("not enough args")
        .parse()
        .expect("failed to parse arg");
    let problem: usize = args
        .next()
        .expect("not enough args")
        .parse()
        .expect("failed to parse arg");

    let input = load_puzzle_input(day).expect("failed to load puzzle input");

    let ans = solutions::run(day, problem, &input).expect("no solution found");
    println!("{}", ans.trim_end());
}
