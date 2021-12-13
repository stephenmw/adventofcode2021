pub fn problem1(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;
    let common = most_common_per_digit(&nums);

    let gamma: Vec<_> = common.clone();
    let epsilon: Vec<_> = common.iter().map(|x| !x).collect();

    let ans = bools_to_int(&gamma) * bools_to_int(&epsilon);
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;

    let oxygen = filter_common(nums.clone(), false);
    let c02 = filter_common(nums.clone(), true);

    let ans = bools_to_int(&oxygen) * bools_to_int(&c02);
    format!("{}", ans)
}

fn filter_common(mut nums: Vec<Vec<bool>>, invert: bool) -> Vec<bool> {
    let l = nums.first().unwrap().len();
    for i in 0..l {
        if nums.len() == 1 {
            return nums[0].clone();
        }
        let common_bit = most_common_digit(&nums, i);
        let criteria = match invert {
            true => !common_bit,
            false => common_bit,
        };
        nums.retain(|num| num[i] == criteria);
    }
    nums[0].clone()
}

fn most_common_digit(nums: &[Vec<bool>], index: usize) -> bool {
    let ones = nums.iter().map(|digits| digits[index]).filter(|&x| x).count();
    let zeros = nums.len() - ones;
    ones >= zeros
}

fn most_common_per_digit(nums: &[Vec<bool>]) -> Vec<bool> {
    let l = nums.first().unwrap().len();
    (0..l).map(|i| most_common_digit(nums, i)).collect()
}

fn bools_to_int(xs: &[bool]) -> usize {
    let mut ret = 0;
    for &digit in xs {
        ret <<= 1;
        if digit {
            ret += 1;
        }
    }
    ret
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
        let bit = map(one_of("01"), |x| x == '1');
        let binary_number = many1(bit);
        let mut parser = separated_list1(line_ending, binary_number);
        parser(input)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "198")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "230")
    }
}
