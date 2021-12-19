use std::cell::RefCell;
use std::fmt;
use std::fmt::Write;
use std::mem;
use std::rc::Rc;
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
            let b = nums[j].clone();
            a.add(b);
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
            a.add(b);
            a
        })
        .unwrap();
    ans
}

mod tree_walker {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct TreeWalker {
        root: Rc<RefCell<SnailfishNum>>,
        stack: Vec<StackEntry>,
    }

    impl TreeWalker {
        pub fn new(root: Rc<RefCell<SnailfishNum>>) -> Self {
            TreeWalker {
                root: root,
                stack: Vec::new(),
            }
        }

        pub fn cursor(&self) -> Rc<RefCell<SnailfishNum>> {
            let entry = match self.stack.last() {
                Some(x) => x,
                None => return self.root.clone(),
            };

            match entry.dir {
                Direction::Left => entry.node.borrow().as_pair().unwrap().left.clone(),
                Direction::Right => entry.node.borrow().as_pair().unwrap().right.clone(),
            }
        }

        pub fn depth(&self) -> usize {
            self.stack.len() + 1
        }

        pub fn left(&mut self) -> Option<Rc<RefCell<SnailfishNum>>> {
            let cur = self.cursor();
            let next = cur.borrow().as_pair()?.left.clone();
            self.stack.push(StackEntry::new(cur, Direction::Left));
            Some(next)
        }

        pub fn right(&mut self) -> Option<Rc<RefCell<SnailfishNum>>> {
            let cur = self.cursor();
            let next = cur.borrow().as_pair()?.right.clone();
            self.stack.push(StackEntry::new(cur, Direction::Right));
            Some(next)
        }

        pub fn up(&mut self) -> Option<Rc<RefCell<SnailfishNum>>> {
            if self.stack.pop().is_none() {
                return None;
            }
            Some(self.cursor())
        }

        pub fn next(&mut self) -> Option<Rc<RefCell<SnailfishNum>>> {
            if self.left().is_some() {
                while let Some(_) = self.left() {}
                return Some(self.cursor());
            }

            loop {
                let entry = self.stack.pop()?;
                if entry.dir == Direction::Left {
                    self.right();
                    while let Some(_) = self.left() {}
                    return Some(self.cursor());
                }
            }
        }

        pub fn prev(&mut self) -> Option<Rc<RefCell<SnailfishNum>>> {
            if self.right().is_some() {
                while let Some(_) = self.right() {}
                return Some(self.cursor());
            }

            loop {
                let entry = self.stack.pop()?;
                if entry.dir == Direction::Right {
                    self.left();
                    while let Some(_) = self.right() {}
                    return Some(self.cursor());
                }
            }
        }
    }

    #[derive(Clone, Debug)]
    struct StackEntry {
        node: Rc<RefCell<SnailfishNum>>,
        dir: Direction,
    }

    impl StackEntry {
        fn new(node: Rc<RefCell<SnailfishNum>>, dir: Direction) -> Self {
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

    #[cfg(test)]
    mod tests {
        use super::*;

        fn test_num(s: &str) -> Rc<RefCell<SnailfishNum>> {
            let num = parser::num(s).unwrap().1;
            Rc::new(RefCell::new(num))
        }

        #[test]
        fn left_test() {
            let num = test_num("[7,[6,[5,[4,[3,2]]]]]]");
            let mut walker = TreeWalker::new(num);
            assert_eq!(walker.left(), Some(test_num("7")));

            let num = test_num("[[[[[9,8],1],2],3],4]");
            let mut walker = TreeWalker::new(num);
            assert_eq!(walker.left(), Some(test_num("[[[[9,8],1],2],3]")));
            assert_eq!(walker.left(), Some(test_num("[[[9,8],1],2]")));
            assert_eq!(walker.left(), Some(test_num("[[9,8],1]")));
        }

        #[test]
        fn next_test() {
            let num = test_num("[7,[6,[5,[4,[3,2]]]]]]");
            let mut walker = TreeWalker::new(num);
            assert_eq!(walker.next(), Some(test_num("7")));
            assert_eq!(walker.next(), Some(test_num("6")));
            assert_eq!(walker.next(), Some(test_num("5")));
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum SnailfishNum {
    Pair(SnailfishPair),
    Literal(u64),
}

impl SnailfishNum {
    fn add(&mut self, other: SnailfishNum) {
        let lhs = mem::take(self);
        let mut new_root = SnailfishNum::from((lhs, other));
        new_root.reduce();
        *self = new_root.into();
    }

    fn magnitude(&self) -> u64 {
        match self {
            SnailfishNum::Literal(l) => *l,
            SnailfishNum::Pair(p) => {
                p.left.borrow().magnitude() * 3 + p.right.borrow().magnitude() * 2
            }
        }
    }

    fn as_pair(&self) -> Option<&SnailfishPair> {
        match self {
            SnailfishNum::Pair(p) => Some(p),
            SnailfishNum::Literal(_) => None,
        }
    }

    fn unwrap_literal(&self) -> u64 {
        match self {
            SnailfishNum::Literal(l) => *l,
            SnailfishNum::Pair(_) => panic!("unwrap_literal: called on pair"),
        }
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
        let root = Rc::new(RefCell::new(mem::take(self)));
        let mut walker = TreeWalker::new(root.clone());

        // find pair to explode
        loop {
            if walker.next().is_none() {
                drop(walker);
                *self = Rc::try_unwrap(root).unwrap().into_inner();
                return false;
            }

            if walker.depth() > 5 {
                walker.up();
                break;
            }
        }

        let exploding_pair = {
            let node = mem::take(&mut *walker.cursor().borrow_mut());
            let pair = node.as_pair().unwrap();
            let left = pair.left.borrow().unwrap_literal();
            let right = pair.right.borrow().unwrap_literal();
            (left, right)
        };

        let mut walker2 = walker.clone();

        if let Some(left_num) = walker.prev() {
            let v = left_num.borrow().unwrap_literal();
            mem::swap(
                &mut *left_num.borrow_mut(),
                &mut (v + exploding_pair.0).into(),
            )
        }

        if let Some(right_num) = walker2.next() {
            let v = right_num.borrow().unwrap_literal();
            mem::swap(
                &mut *right_num.borrow_mut(),
                &mut (v + exploding_pair.1).into(),
            )
        }

        drop(walker);
        drop(walker2);
        *self = Rc::try_unwrap(root).unwrap().into_inner();
        true
    }

    fn reduce_split(&mut self) -> bool {
        let root = Rc::new(RefCell::new(mem::take(self)));
        let mut walker = TreeWalker::new(root.clone());
        while let Some(node) = walker.next() {
            let n = node.borrow().unwrap_literal();
            if n > 9 {
                let left = SnailfishNum::from(n / 2);
                let right = SnailfishNum::from((n + 1) / 2);
                *node.borrow_mut() = SnailfishNum::from((left, right));
                drop(walker);
                *self = Rc::try_unwrap(root).unwrap().into_inner();
                return true;
            }
        }
        drop(walker);
        *self = Rc::try_unwrap(root).unwrap().into_inner();
        false
    }
}

impl Default for SnailfishNum {
    fn default() -> Self {
        SnailfishNum::Literal(0)
    }
}

impl From<u64> for SnailfishNum {
    fn from(i: u64) -> Self {
        SnailfishNum::Literal(i)
    }
}

impl From<SnailfishPair> for SnailfishNum {
    fn from(p: SnailfishPair) -> Self {
        SnailfishNum::Pair(p)
    }
}

impl From<(SnailfishNum, SnailfishNum)> for SnailfishNum {
    fn from(p: (SnailfishNum, SnailfishNum)) -> Self {
        SnailfishPair::from(p).into()
    }
}

impl fmt::Display for SnailfishNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SnailfishNum::Literal(l) => write!(f, "{}", l),
            SnailfishNum::Pair(p) => {
                f.write_char('[')?;
                p.left.borrow().fmt(f)?;
                f.write_char(',')?;
                p.right.borrow().fmt(f)?;
                f.write_char(']')?;
                Ok(())
            }
        }
    }
}

impl fmt::Debug for SnailfishNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct SnailfishPair {
    left: Rc<RefCell<SnailfishNum>>,
    right: Rc<RefCell<SnailfishNum>>,
}

impl From<(SnailfishNum, SnailfishNum)> for SnailfishPair {
    fn from(p: (SnailfishNum, SnailfishNum)) -> Self {
        SnailfishPair {
            left: Rc::new(RefCell::new(p.0)),
            right: Rc::new(RefCell::new(p.1)),
        }
    }
}

impl Clone for SnailfishPair {
    fn clone(&self) -> Self {
        SnailfishPair {
            left: Rc::new((*self.left).clone()),
            right: Rc::new((*self.right).clone()),
        }
    }
}

mod parser {
    use super::SnailfishNum;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<SnailfishNum>> {
        let parser = separated_list1(line_ending, num);
        complete(parser)(input)
    }

    pub fn num(input: &str) -> IResult<&str, SnailfishNum> {
        alt((literal, pair))(input)
    }

    fn literal(input: &str) -> IResult<&str, SnailfishNum> {
        map(uint::<u64>, |x| x.into())(input)
    }

    fn pair(input: &str) -> IResult<&str, SnailfishNum> {
        let p = delimited(tag("["), separated_pair(num, tag(","), num), tag("]"));
        let mut parser = map(p, |x| x.into());
        parser(input)
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
            let num = parser::num(expected).unwrap().1;
            assert_eq!(add_nums(input), num);
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
