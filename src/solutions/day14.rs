use std::collections::HashMap;

type Element = char;
type ElementPair = (Element, Element);
type Rules = HashMap<ElementPair, Element>;

pub fn problem1(input: &str) -> String {
    let (template, rules) = parser::parse(input).unwrap().1;
    let mut polymer = Polymer::new(&template);
    (0..10).for_each(|_| polymer.grow(&rules));
    let ans = max_minus_min(polymer.element_freq());
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let (template, rules) = parser::parse(input).unwrap().1;
    let mut polymer = Polymer::new(&template);
    (0..40).for_each(|_| polymer.grow(&rules));
    let ans = max_minus_min(polymer.element_freq());
    format!("{}", ans)
}

fn max_minus_min(freq: HashMap<Element, usize>) -> usize {
    let max = freq.values().cloned().max().unwrap();
    let min = freq.values().cloned().min().unwrap();
    max - min
}

#[derive(Clone, Debug)]
struct Polymer {
    freq: HashMap<ElementPair, usize>,
    left: Element,
    right: Element,
}

impl Polymer {
    fn new(template: &[Element]) -> Self {
        assert!(template.len() > 1);

        let mut m = HashMap::new();
        for pair in template.windows(2).map(|xs| (xs[0], xs[1])) {
            *m.entry(pair).or_insert(0) += 1;
        }

        let left = template.first().cloned().unwrap();
        let right = template.last().cloned().unwrap();

        Polymer {
            freq: m,
            left: left,
            right: right,
        }
    }

    fn element_freq(&self) -> HashMap<Element, usize> {
        let mut m = HashMap::new();

        for (pair, count) in self.freq.iter() {
            *m.entry(pair.0).or_insert(0) += count;
            *m.entry(pair.1).or_insert(0) += count;
        }

        *m.get_mut(&self.left).unwrap() += 1;
        *m.get_mut(&self.right).unwrap() += 1;
        m.values_mut().for_each(|x| *x /= 2);

        m
    }

    fn grow(&mut self, rules: &Rules) {
        let mut new_freq = HashMap::new();
        for (&pair, &count) in self.freq.iter() {
            if let Some(e) = rules.get(&pair).cloned() {
                *new_freq.entry((pair.0, e)).or_insert(0) += count;
                *new_freq.entry((e, pair.1)).or_insert(0) += count;
            } else {
                *new_freq.entry(pair).or_insert(0) += count;
            }
        }
        self.freq = new_freq;
    }
}

mod parser {
    use super::{Element, Rules};
    use crate::lib::combinators::*;
    use std::collections::HashMap;

    pub fn parse(input: &str) -> IResult<&str, (Vec<Element>, Rules)> {
        let template = map(take_while(is_alphabetic), |x: &str| x.chars().collect());
        let rule = separated_pair(tuple((anychar, anychar)), tag(" -> "), anychar);
        let rules = map(separated_list1(line_ending, rule), |rules| {
            HashMap::from_iter(rules)
        });
        let parser = separated_pair(template, tuple((line_ending, line_ending)), rules);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "1588")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "2188189693529")
    }
}
