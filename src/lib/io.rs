use std::fmt;
use std::fs;
use std::io;


#[allow(dead_code)]
pub fn write_sep_ln<I, O, T, U>(mut output: O, sep: T, mut input: I) -> Result<(), fmt::Error>
where
    I: Iterator<Item = U>,
    O: fmt::Write,
    T: fmt::Display,
    U: fmt::Display,
{
    if let Some(v) = input.next() {
        write!(output, "{}", v)?;
    }

    for v in input {
        write!(output, "{}{}", sep, v)?;
    }

    output.write_char('\n')
}

pub fn load_puzzle_input(day: usize) -> io::Result<String> {
    let filename = format!("puzzle-inputs/day{:02}.txt", day);
    fs::read_to_string(filename)
}
