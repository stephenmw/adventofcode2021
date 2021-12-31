pub fn problem1(input: &str) -> String {
    let lines = parser::parse(input).unwrap().1;
    let ans: usize = lines
        .iter()
        .filter_map(|l| check_syntax(l).and_then(|x| x.unexpected_char()))
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("unknown closing char"),
        })
        .sum();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let lines = parser::parse(input).unwrap().1;
    let mut scores: Vec<_> = lines
        .iter()
        .filter_map(|l| check_syntax(l).and_then(|x| x.unexpected_eof()))
        .map(|expected| score_expected(&expected))
        .collect();
    scores.sort();
    let median_score = scores[scores.len() / 2];
    format!("{}", median_score)
}

fn score_expected(line: &[char]) -> usize {
    line.iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("bad expected char"),
        })
        .fold(0, |acc, x| acc * 5 + x)
}

fn check_syntax(line: &[char]) -> Option<SyntaxError> {
    let mut stack = Vec::new();
    for &ch in line.iter() {
        let expected_open = match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
                continue;
            }
            ')' => '(',
            ']' => '[',
            '}' => '{',
            '>' => '<',
            _ => panic!("unknown char"),
        };

        if stack.pop() != Some(expected_open) {
            return Some(SyntaxError::UnexpectedChar(ch));
        }
    }

    if stack.is_empty() {
        None
    } else {
        let expected = stack
            .into_iter()
            .rev()
            .map(|c| match c {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                '<' => '>',
                _ => unreachable!(), // stack only contains open brackets.
            })
            .collect();

        Some(SyntaxError::UnexpectedEOF(expected))
    }
}

enum SyntaxError {
    UnexpectedChar(char),
    UnexpectedEOF(Vec<char>),
}

impl SyntaxError {
    fn unexpected_char(&self) -> Option<char> {
        match self {
            Self::UnexpectedChar(c) => Some(*c),
            _ => None,
        }
    }

    fn unexpected_eof(&self) -> Option<Vec<char>> {
        match self {
            Self::UnexpectedEOF(expected) => Some(expected.clone()),
            _ => None,
        }
    }
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
        let ch = one_of("()[]{}<>");
        let row = many1(ch);
        let rows = separated_list1(line_ending, row);
        complete(rows)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "26397")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "288957")
    }
}
