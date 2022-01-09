use std::fmt;
use std::fmt::Write;
use tree_walker::TreeWalker;

pub fn problem1(input: &str) -> String {
    format!("{}", add_nums(input).magnitude())
}

pub fn problem2(input: &str) -> String {
    let nums = parser::parse(input).unwrap().1;
    let ans = (0..nums.len())
        .flat_map(|i| (0..nums.len()).map(move |j| (i, j)))
        .filter(|(i, j)| i != j)
        .map(|(i, j)| {
            let mut a = nums[i].clone();
            a.add(&nums[j]);
            a.magnitude()
        })
        .max()
        .unwrap();
    format!("{}", ans)
}

fn add_nums(input: &str) -> SnailfishNum {
    let nums = parser::parse(input).unwrap().1;
    let ans = nums
        .into_iter()
        .reduce(|mut a, b| {
            a.add(&b);
            a
        })
        .unwrap();
    ans
}

mod tree_walker {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct TreeWalker {
        stack: Vec<StackEntry>,
    }

    impl TreeWalker {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn cursor(&self, num: &SnailfishNum) -> usize {
            let entry = match self.stack.last() {
                Some(x) => x,
                None => return num.root,
            };

            match entry.dir {
                Direction::Left => num.nodes[entry.node].as_pair().unwrap().left,
                Direction::Right => num.nodes[entry.node].as_pair().unwrap().right,
            }
        }

        pub fn depth(&self) -> usize {
            self.stack.len() + 1
        }

        pub fn left(&mut self, num: &SnailfishNum) -> Option<usize> {
            let cur = self.cursor(num);
            let next = num.nodes[cur].as_pair()?.left;
            self.stack.push(StackEntry::new(cur, Direction::Left));
            Some(next)
        }

        pub fn right(&mut self, num: &SnailfishNum) -> Option<usize> {
            let cur = self.cursor(num);
            let next = num.nodes[cur].as_pair()?.right;
            self.stack.push(StackEntry::new(cur, Direction::Right));
            Some(next)
        }

        pub fn up(&mut self, num: &SnailfishNum) -> Option<usize> {
            if self.stack.pop().is_none() {
                return None;
            }
            Some(self.cursor(num))
        }

        pub fn next(&mut self, num: &SnailfishNum) -> Option<usize> {
            if self.left(num).is_some() {
                while let Some(_) = self.left(num) {}
                return Some(self.cursor(num));
            }

            loop {
                let entry = self.stack.pop()?;
                if entry.dir == Direction::Left {
                    self.right(num);
                    while let Some(_) = self.left(num) {}
                    return Some(self.cursor(num));
                }
            }
        }

        pub fn prev(&mut self, num: &SnailfishNum) -> Option<usize> {
            if self.right(num).is_some() {
                while let Some(_) = self.right(num) {}
                return Some(self.cursor(num));
            }

            loop {
                let entry = self.stack.pop()?;
                if entry.dir == Direction::Right {
                    self.left(num);
                    while let Some(_) = self.right(num) {}
                    return Some(self.cursor(num));
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    struct StackEntry {
        node: usize,
        dir: Direction,
    }

    impl StackEntry {
        fn new(node: usize, dir: Direction) -> Self {
            StackEntry {
                node: node,
                dir: dir,
            }
        }
    }

    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    enum Direction {
        Left,
        Right,
    }
}

#[derive(Clone, Debug)]
pub struct SnailfishNum {
    root: usize,
    nodes: Vec<SnailfishNode>,
}

impl SnailfishNum {
    fn from_nodes(nodes: Vec<SnailfishNode>, root: usize) -> Self {
        Self {
            root: root,
            nodes: nodes,
        }
    }

    fn add(&mut self, other: &SnailfishNum) {
        let offset = self.nodes.len();
        self.nodes.extend_from_slice(&other.nodes);

        // point new nodes at the new locations.
        for n in &mut self.nodes[offset..] {
            if let SnailfishNode::Pair(ref mut pair) = n {
                pair.left += offset;
                pair.right += offset;
            }
        }

        // add new root node
        let new_root = SnailfishNode::from((self.root, other.root + offset));
        self.root = self.add_node(new_root);

        self.reduce();
    }

    fn magnitude(&self) -> u64 {
        fn m_rec(num: &SnailfishNum, index: usize) -> u64 {
            match &num.nodes[index] {
                SnailfishNode::Literal(l) => *l,
                SnailfishNode::Pair(p) => m_rec(num, p.left) * 3 + m_rec(num, p.right) * 2,
            }
        }

        m_rec(self, self.root)
    }

    fn reduce(&mut self) {
        loop {
            if self.reduce_explode() {
                continue;
            }

            if self.reduce_split() {
                continue;
            }

            break;
        }
    }

    fn reduce_explode(&mut self) -> bool {
        let mut walker = TreeWalker::new();

        // find pair to explode
        loop {
            if walker.next(self).is_none() {
                return false;
            }

            if walker.depth() > 5 {
                walker.up(self);
                break;
            }
        }

        let exploding_pair = {
            let pair = self.nodes[walker.cursor(self)].as_pair().unwrap();
            let left = self.nodes[pair.left].unwrap_literal();
            let right = self.nodes[pair.right].unwrap_literal();
            (left, right)
        };

        // replace exploding pair with 0
        let cur = walker.cursor(self);
        self.nodes[cur] = SnailfishNode::default();

        let mut walker2 = walker.clone();

        if let Some(left_num) = walker.prev(self) {
            let v = self.nodes[left_num].unwrap_literal();
            self.nodes[left_num] = (v + exploding_pair.0).into();
        }

        if let Some(right_num) = walker2.next(self) {
            let v = self.nodes[right_num].unwrap_literal();
            self.nodes[right_num] = (v + exploding_pair.1).into();
        }

        true
    }

    fn reduce_split(&mut self) -> bool {
        let mut walker = TreeWalker::new();
        while let Some(node) = walker.next(self) {
            let n = self.nodes[node].unwrap_literal();
            if n > 9 {
                let left = self.add_node(SnailfishNode::from(n / 2));
                let right = self.add_node(SnailfishNode::from((n + 1) / 2));

                self.nodes[node] = SnailfishNode::from((left, right));
                return true;
            }
        }

        false
    }

    fn add_node(&mut self, node: SnailfishNode) -> usize {
        let id = self.nodes.len();
        self.nodes.push(node);
        id
    }
}

impl fmt::Display for SnailfishNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn rec_fmt(f: &mut fmt::Formatter<'_>, nodes: &[SnailfishNode], i: usize) -> fmt::Result {
            match &nodes[i] {
                SnailfishNode::Literal(l) => write!(f, "{}", l),
                SnailfishNode::Pair(p) => {
                    f.write_char('[')?;
                    rec_fmt(f, nodes, p.left)?;
                    f.write_char(',')?;
                    rec_fmt(f, nodes, p.right)?;
                    f.write_char(']')?;
                    Ok(())
                }
            }
        }

        rec_fmt(f, &self.nodes, self.root)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SnailfishNode {
    Pair(SnailfishPair),
    Literal(u64),
}

impl SnailfishNode {
    fn as_pair(&self) -> Option<&SnailfishPair> {
        match self {
            SnailfishNode::Pair(p) => Some(p),
            SnailfishNode::Literal(_) => None,
        }
    }

    fn unwrap_literal(&self) -> u64 {
        match self {
            SnailfishNode::Literal(l) => *l,
            SnailfishNode::Pair(_) => panic!("unwrap_literal: called on pair"),
        }
    }
}

impl Default for SnailfishNode {
    fn default() -> Self {
        SnailfishNode::Literal(0)
    }
}

impl From<u64> for SnailfishNode {
    fn from(i: u64) -> Self {
        SnailfishNode::Literal(i)
    }
}

impl From<SnailfishPair> for SnailfishNode {
    fn from(p: SnailfishPair) -> Self {
        SnailfishNode::Pair(p)
    }
}

impl From<(usize, usize)> for SnailfishNode {
    fn from(p: (usize, usize)) -> Self {
        SnailfishPair::from(p).into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SnailfishPair {
    left: usize,
    right: usize,
}

impl From<(usize, usize)> for SnailfishPair {
    fn from(p: (usize, usize)) -> Self {
        Self {
            left: p.0,
            right: p.1,
        }
    }
}

mod parser {
    use super::{SnailfishNode, SnailfishNum};
    use crate::lib::combinators::*;
    use std::cell::RefCell;

    #[derive(Clone, Debug, Default)]
    struct SnailfishNumParser {
        nodes: RefCell<Vec<SnailfishNode>>,
    }

    impl SnailfishNumParser {
        fn new() -> Self {
            Self::default()
        }

        fn add_node(&self, node: SnailfishNode) -> usize {
            let mut nodes = self.nodes.borrow_mut();
            let id = nodes.len();
            nodes.push(node);
            id
        }

        fn into_inner(self) -> Vec<SnailfishNode> {
            self.nodes.into_inner()
        }

        fn node<'a>(&self, input: &'a str) -> IResult<&'a str, usize> {
            alt((|i| self.literal(i), |i| self.pair(i)))(input)
        }

        fn literal<'a>(&self, input: &'a str) -> IResult<&'a str, usize> {
            let (rest, node) = into(uint::<u64>)(input)?;
            Ok((rest, self.add_node(node)))
        }

        fn pair<'a>(&self, input: &'a str) -> IResult<&'a str, usize> {
            let p = delimited(
                tag("["),
                separated_pair(|i| self.node(i), tag(","), |i| self.node(i)),
                tag("]"),
            );

            let (rest, node) = into(p)(input)?;
            Ok((rest, self.add_node(node)))
        }
    }

    pub fn parse(input: &str) -> IResult<&str, Vec<SnailfishNum>> {
        let parser = separated_list1(line_ending, num);
        complete(parser)(input)
    }

    pub fn num(input: &str) -> IResult<&str, SnailfishNum> {
        let parser = SnailfishNumParser::new();
        let (rest, root) = parser.node(input)?;
        let ret = SnailfishNum::from_nodes(parser.into_inner(), root);
        Ok((rest, ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "4140")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "3993")
    }

    #[test]
    fn add_test() {
        let tests = [
            (
                "[[[[1,1],[2,2]],[3,3]],[4,4]]",
                "[1,1]
[2,2]
[3,3]
[4,4]",
            ),
            (
                "[[[[3,0],[5,3]],[4,4]],[5,5]]",
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]",
            ),
            (
                "[[[[5,0],[7,4]],[5,5]],[6,6]]",
                "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]",
            ),
            (
                "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
                "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]",
            ),
            (
                "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
                "[[[[4,3],4],4],[7,[[8,4],9]]]
[1,1]",
            ),
        ];

        for (expected, input) in tests {
            assert_eq!(format!("{}", add_nums(input)), expected);
        }
    }

    #[test]
    fn explode_test() {
        let tests = [
            ("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]"),
            ("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]"),
            ("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]"),
            (
                "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
            ),
            (
                "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]",
                "[[3,[2,[8,0]]],[9,[5,[7,0]]]]",
            ),
            (
                "[[[[4,0],[5,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
                "[[[[4,0],[5,4]],[[0,[7,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]",
            ),
        ];

        for (input, expected) in tests {
            let mut num = parser::num(input).unwrap().1;
            let ret = num.reduce_explode();
            assert_eq!(ret, true);
            assert_eq!(format!("{}", num), expected);
        }
    }
}
