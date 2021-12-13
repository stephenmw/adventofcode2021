pub fn problem1(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;
    let ans = count_increase(&nums);

    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;
    let ans = count_increase(nums.windows(3).map(|x| x.iter().sum::<usize>()));

    format!("{}", ans)
}

fn count_increase<I, T>(xs: I) -> usize
where
    I: IntoIterator<Item = T>,
    T: std::cmp::Ord,
{
    let mut iter = xs.into_iter();
    let mut last = match iter.next() {
        Some(x) => x,
        None => return 0,
    };

    let mut ret = 0;

    for n in iter {
        if n > last {
            ret += 1;
        }
        last = n;
    }

    ret
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<usize>> {
        let mut parser = separated_list1(line_ending, uint);
        parser(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "7")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "5")
    }
}
