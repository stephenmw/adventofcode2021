pub fn problem1(input: &str) -> String {
    let fish = parser::parse(input).unwrap().1;
    let mut school = LanternfishSchool::new(&fish);
    (0..80).for_each(|_| school.tick());
    let ans = school.count();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let fish = parser::parse(input).unwrap().1;
    let mut school = LanternfishSchool::new(&fish);
    (0..256).for_each(|_| school.tick());
    let ans = school.count();
    format!("{}", ans)
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<usize>> {
        let starting = separated_list1(tag(","), uint);
        complete(starting)(input)
    }
}

#[derive(Clone, Debug, Default)]
struct LanternfishSchool {
    ages: [usize; 9],
}

impl LanternfishSchool {
    fn new(fish: &[usize]) -> Self {
        let mut ret = Self::default();

        for &n in fish.iter() {
            ret.ages[n] += 1;
        }

        ret
    }

    fn tick(&mut self) {
        self.ages.rotate_left(1);
        self.ages[6] += self.ages[8];
    }

    fn count(&self) -> usize {
        self.ages.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "5934")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "26984457539")
    }
}
