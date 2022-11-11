use std::{collections::HashMap, str::FromStr};

pub fn problem1(input: &str) -> String {
    let displays = parser::parse(input).unwrap().1;
    let ans = displays
        .iter()
        .flat_map(|d| d.final_patterns.iter())
        .filter(|p| [2, 4, 3, 7].contains(&p.segments.len()))
        .count();

    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let displays = parser::parse(input).unwrap().1;
    let ans: usize = displays
        .iter()
        .map(|d| {
            let t = Translation::compute(&d.seen_patterns);
            d.final_value(&t)
        })
        .sum();

    format!("{}", ans)
}

lazy_static! {
    static ref DIGIT_PATTERN: HashMap<Pattern, usize> = {
        HashMap::from_iter([
            (0, Pattern::from_str("abcefg").unwrap()),
            (1, Pattern::from_str("cf").unwrap()),
            (2, Pattern::from_str("acdeg").unwrap()),
            (3, Pattern::from_str("acdfg").unwrap()),
            (4, Pattern::from_str("bcdf").unwrap()),
            (5, Pattern::from_str("abdfg").unwrap()),
            (6, Pattern::from_str("abdefg").unwrap()),
            (7, Pattern::from_str("acf").unwrap()),
            (8, Pattern::from_str("abcdefg").unwrap()),
            (9, Pattern::from_str("abcdfg").unwrap()),
        ].into_iter().map(|(a, b)| (b, a)))
    };

    static ref SEGMENT_FINGERPRINT: HashMap<Vec<usize>, Segment> = {
        let mut fps: HashMap<Segment, Vec<usize>> = HashMap::new();
        for (pattern, _) in DIGIT_PATTERN.iter() {
            for &s in &pattern.segments {
                fps.entry(s).or_default().push(pattern.segments.len());
            }
        }

        // fingerprints must be sorted.
        fps.values_mut().for_each(|x| x.sort());

        HashMap::from_iter(fps.into_iter().map(|(s, f)| (f, s)))
    };
}

type Segment = char;

#[derive(PartialEq, Eq, Hash)]
struct Pattern {
    segments: Vec<Segment>,
}

impl From<Vec<Segment>> for Pattern {
    fn from(mut s: Vec<Segment>) -> Self {
        s.sort();
        s.dedup();
        Pattern { segments: s }
    }
}

impl FromStr for Pattern {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Pattern::from(s.chars().collect::<Vec<_>>()))
    }
}

struct Translation {
    m: HashMap<Segment, Segment>,
}

impl Translation {
    fn compute(seen_patterns: &[Pattern]) -> Self {
        let mut fingerprints: HashMap<Segment, Vec<usize>> = HashMap::new();
        for p in seen_patterns {
            for s in &p.segments {
                fingerprints.entry(*s).or_default().push(p.segments.len())
            }
        }

        fingerprints.values_mut().for_each(|x| x.sort());

        let m = fingerprints
            .into_iter()
            .map(|(s, fp)| (s, *SEGMENT_FINGERPRINT.get(&fp).unwrap()));

        Translation {
            m: HashMap::from_iter(m),
        }
    }

    fn translate(&self, input: &Pattern) -> Pattern {
        input
            .segments
            .iter()
            .map(|s| *self.m.get(s).unwrap())
            .collect::<Vec<_>>()
            .into()
    }
}

pub struct Display {
    seen_patterns: Vec<Pattern>,
    final_patterns: Vec<Pattern>,
}

impl Display {
    fn new(seen_patterns: Vec<Pattern>, final_patterns: Vec<Pattern>) -> Self {
        Display {
            seen_patterns: seen_patterns,
            final_patterns: final_patterns,
        }
    }

    fn final_value(&self, t: &Translation) -> usize {
        self.final_patterns
            .iter()
            .map(|p| t.translate(p))
            .map(|p| DIGIT_PATTERN.get(&p).unwrap())
            .fold(0, |acc, d| acc * 10 + d)
    }
}

mod parser {
    use super::*;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Display>> {
        let patterns = || {
            let segment = one_of("abcdefg");
            let pattern = map(many1(segment), |x| Pattern::from(x));
            separated_list1(space1, pattern)
        };
        let display = map(
            separated_pair(patterns(), tag(" | "), patterns()),
            |(a, b)| Display::new(a, b),
        );
        let displays = separated_list1(line_ending, display);
        complete(displays)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "26")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "61229")
    }
}
