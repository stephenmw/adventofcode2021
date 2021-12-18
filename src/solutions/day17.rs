pub fn problem1(input: &str) -> String {
    let (_x_target, y_target) = parser::parse(input).unwrap().1;
    // Wrong, but it solves the problem...
    // TODO: replace with values obtained from problem2
    let ans: i32 = (1..y_target.start.abs()).sum();

    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let (x_target, y_target) = parser::parse(input).unwrap().1;
    let x_ranges: Vec<_> = (0..x_target.end.unwrap() + 1)
        .filter_map(|v| Some((v, time_on_target_x(v, &x_target)?)))
        .collect();
    let y_ranges: Vec<_> = (y_target.start - 1..y_target.start.abs() + 1)
        .filter_map(|v| Some((v, time_on_target_y(v, &y_target)?)))
        .collect();

    let mut count = 0;

    for (_y_v, y_range) in y_ranges.iter() {
        for (_x_v, x_range) in x_ranges.iter() {
            if x_range.overlap(y_range) {
                count += 1;
            }
        }
    }

    format!("{}", count)
}

fn time_on_target_x(initial_velocity: i32, target: &Range) -> Option<Range> {
    let end = target
        .end
        .expect("time_on_target_x: target must be bounded");

    let mut v = initial_velocity;
    let mut cur = 0;
    let mut steps = 0;
    let mut first_step_on_target = None;

    loop {
        if target.contains(cur) {
            first_step_on_target.get_or_insert(steps);
        }

        if cur > end {
            return first_step_on_target.map(|s| Range::new(s, steps - 1));
        }

        if v == 0 {
            return first_step_on_target.map(|s| Range::new_unbounded(s));
        }

        steps += 1;
        cur += v;
        v = (v - 1).max(0);
    }
}

fn time_on_target_y(initial_velocity: i32, target: &Range) -> Option<Range> {
    let start = target.start;

    let mut v = initial_velocity;
    let mut cur = 0;
    let mut steps = 0;
    let mut first_step_on_target = None;

    loop {
        if target.contains(cur) {
            first_step_on_target.get_or_insert(steps);
        }

        if cur < start {
            return first_step_on_target.map(|s| Range::new(s, steps - 1));
        }

        steps += 1;
        cur += v;
        v = v - 1;
    }
}

// A bounded [start, end] or unbounded [start, inf) range.
#[derive(Clone, Debug)]
pub struct Range {
    start: i32,
    end: Option<i32>,
}

impl Range {
    fn new(start: i32, end: i32) -> Self {
        Range {
            start: start,
            end: Some(end),
        }
    }

    fn new_unbounded(start: i32) -> Self {
        Range {
            start: start,
            end: None,
        }
    }

    fn contains(&self, n: i32) -> bool {
        if let Some(end) = self.end {
            if n > end {
                return false;
            }
        }

        n >= self.start
    }

    fn overlap(&self, other: &Range) -> bool {
        if self.start > other.end.unwrap_or(i32::MAX) || other.start > self.end.unwrap_or(i32::MAX)
        {
            false
        } else {
            true
        }
    }
}

impl From<(i32, i32)> for Range {
    fn from(r: (i32, i32)) -> Self {
        Range::new(r.0, r.1)
    }
}

mod parser {
    use super::Range;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, (Range, Range)> {
        let range = || map(separated_pair(int, tag(".."), int), |r| r.into());
        let parser = separated_pair(
            preceded(tag("x="), range()),
            tag(", "),
            preceded(tag("y="), range()),
        );
        complete(preceded(tag("target area: "), parser))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "45")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "112")
    }
}
