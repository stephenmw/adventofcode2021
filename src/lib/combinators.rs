use std::str::FromStr;

pub use nom::{
    bytes::complete::{is_a, tag, take_while},
    character::complete::line_ending,
    combinator::{map_res, verify},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn is_alphabetic(c: char) -> bool {
    c.is_ascii_alphabetic()
}

pub fn uint_parser<T: FromStr>(input: &str) -> IResult<&str, T> {
    let digits = is_a("0123456789");
    let mut parser = map_res(digits, |x: &str| x.parse());
    parser(input)
}