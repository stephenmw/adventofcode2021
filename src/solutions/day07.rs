pub fn problem1(input: &str) -> String {
    let mut nums = parser::parse(input).unwrap().1;
    nums.sort();
    let median = nums[nums.len() / 2];
    let sum_dist: i32 = nums.iter().copied().map(|x| (median - x).abs()).sum();
    format!("{}", sum_dist)
}

pub fn problem2(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;
    let max = nums.iter().copied().max().unwrap();

    fn cost(nums: &[i32], i: i32) -> usize {
        nums.iter()
            .map(|n| (n - i).abs()) // distance
            .map(|n| n * (n + 1) / 2) // sum of 1 to n
            .sum::<i32>() as usize
    }

    let ans = (0..=max).map(|i| cost(&nums, i)).min().unwrap();
    format!("{}", ans)
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<i32>> {
        complete(separated_list1(tag(","), uint))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "37")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "168")
    }
}
