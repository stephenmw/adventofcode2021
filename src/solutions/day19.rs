use std::collections::HashMap;
use std::fmt;

pub fn problem1(input: &str) -> String {
    let unsolved_scanners = parser::parse(input).unwrap().1;
    let solved_scanners = solve_scanners(unsolved_scanners);

    let mut beacons: Vec<_> = solved_scanners.iter()
        .flat_map(|(s, _)| s.beacons.iter())
        .collect();
    
    beacons.sort();
    beacons.dedup();

    format!("{}", beacons.len())
}

pub fn problem2(input: &str) -> String {
    let unsolved_scanners = parser::parse(input).unwrap().1;
    let solved_scanners = solve_scanners(unsolved_scanners);

    let scanner_coordinates: Vec<_> = solved_scanners.iter()
        .map(|(_, t)| t.translation)
        .collect();
    
    let mut max = 0;

    for a in scanner_coordinates.iter() {
        for b in scanner_coordinates.iter() {
            max = max.max(a.distance(b));
        }
    }

    format!("{}", max)
}

fn solve_scanners(mut scanners: Vec<Scanner>) -> Vec<(Scanner, Transformation)> {
    let mut useful_scanners = Vec::new();
    let mut done_scanners = Vec::new();

    useful_scanners.push((scanners.swap_remove(0), Transformation::default()));
    while let Some(cur) = useful_scanners.pop() {
        let mut i = 0;
        while i < scanners.len() {
            if let Some(transform) = cur.0.find_overlap(&scanners[i]) {
                let mut new_scanner = scanners.swap_remove(i);
                new_scanner.apply_transformation(&transform);
                useful_scanners.push((new_scanner, transform));
            } else {
                i += 1;
            }
        }
        done_scanners.push(cur);
    }

    assert!(scanners.is_empty());
    done_scanners
}

pub struct Scanner {
    _id: i32,
    beacons: Vec<Point>,
}

impl Scanner {
    fn find_overlap(&self, other: &Scanner) -> Option<Transformation> {
        for sp in self.beacons.iter() {
            for op in other.beacons.iter() {
                if let Some(t) = self.overlap(other, sp, op) {
                    return Some(t);
                }
            }
        }

        None
    }

    fn overlap(
        &self,
        other: &Scanner,
        self_point: &Point,
        other_point: &Point,
    ) -> Option<Transformation> {
        fn distance_diff_hashmap(
            beacons: &[Point],
            p: &Point,
        ) -> HashMap<i32, Vec<(Point, Point)>> {
            let mut m = HashMap::new();

            for b in beacons {
                m.entry(p.distance(b))
                    .or_insert_with(Vec::new)
                    .push((*b, p.diff(b)));
            }

            m
        }

        let self_dists = distance_diff_hashmap(&self.beacons, self_point);
        let other_dists = distance_diff_hashmap(&other.beacons, other_point);

        let mut candidates = Vec::new();
        for (dist, diffs) in self_dists.iter() {
            for diff in diffs.iter() {
                if let Some(o_diffs) = other_dists.get(dist) {
                    for other_diff in o_diffs {
                        candidates.push((diff, other_diff));
                    }
                }
            }
        }

        if candidates.len() < 12 {
            return None;
        }

        let mut rotations = HashMap::new();
        for (a, b) in candidates.iter() {
            for rotation in Rotation::rotations_to_equal(&a.1, &b.1) {
                rotations
                    .entry(rotation)
                    .or_insert_with(Vec::new)
                    .push((a.0, b.0));
            }
        }

        for (rotation, pairs) in rotations {
            if pairs.len() < 12 {
                continue;
            }

            let mut by_translation = HashMap::new();
            for (a, b) in pairs {
                let diff = a.diff(&rotation.apply(&b));
                by_translation
                    .entry(diff)
                    .or_insert_with(Vec::new)
                    .push((a, b))
            }

            for (translation, pairs) in by_translation {
                if pairs.len() >= 12 {
                    return Some(Transformation {
                        rotation: rotation,
                        translation: translation,
                    });
                }
            }
        }

        None
    }

    fn apply_transformation(&mut self, t: &Transformation) {
        self.beacons.iter_mut().for_each(|x| *x = t.apply(x));
    }
}

impl From<(i32, Vec<Point>)> for Scanner {
    fn from(s: (i32, Vec<Point>)) -> Self {
        Scanner {
            _id: s.0,
            beacons: s.1,
        }
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn translate(&self, diff: &Point) -> Point {
        Point {
            x: self.x + diff.x,
            y: self.y + diff.y,
            z: self.z + diff.z,
        }
    }

    fn diff(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }

    // Computes manhattan distance.
    fn distance(&self, other: &Point) -> i32 {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        let z_diff = (self.z - other.z).abs();

        x_diff + y_diff + z_diff
    }
}

impl From<(i32, i32, i32)> for Point {
    fn from(p: (i32, i32, i32)) -> Self {
        Point {
            x: p.0,
            y: p.1,
            z: p.2,
        }
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Rotation {
    swaps: (Axis, Axis, Axis),
    negations: (bool, bool, bool),
}

impl Rotation {
    fn rotations_to_equal(a: &Point, b: &Point) -> Vec<Rotation> {
        fn sorted_abs_values(p: &Point) -> [i32; 3] {
            let mut ret = [p.x.abs(), p.y.abs(), p.z.abs()];
            ret.sort();
            ret
        }

        if sorted_abs_values(a) != sorted_abs_values(b) {
            return Vec::new();
        }

        let target = (a.x.abs(), a.y.abs(), a.z.abs());

        let permutations = [
            (Axis::X, Axis::Y, Axis::Z),
            (Axis::X, Axis::Z, Axis::Y),
            (Axis::Y, Axis::X, Axis::Z),
            (Axis::Y, Axis::Z, Axis::X),
            (Axis::Z, Axis::Y, Axis::X),
            (Axis::Z, Axis::X, Axis::Y),
        ];

        let swaps = permutations.iter().filter(|swap| {
            (
                swap.0.value(b).abs(),
                swap.1.value(b).abs(),
                swap.2.value(b).abs(),
            ) == target
        });

        if a.x == 0 || a.y == 0 || a.z == 0 {
            swaps
                .flat_map(|swap| {
                    let mut options = Vec::new();
                    for l in [true, false] {
                        for j in [true, false] {
                            for k in [true, false] {
                                options.push((l, j, k));
                            }
                        }
                    }
                    options.into_iter().map(|neg| Rotation {
                        swaps: *swap,
                        negations: neg,
                    })
                })
                .filter(|x| x.valid())
                .collect()
        } else {
            swaps
                .filter_map(|swap| {
                    let negations = (
                        a.x == -1 * swap.0.value(b),
                        a.y == -1 * swap.1.value(b),
                        a.z == -1 * swap.2.value(b),
                    );
                    Some(Rotation {
                        swaps: *swap,
                        negations: negations,
                    })
                    .filter(|x| x.valid())
                })
                .collect()
        }
    }

    fn apply(&self, p: &Point) -> Point {
        let neg = |x| if x { -1 } else { 1 };
        Point {
            x: self.swaps.0.value(p) * neg(self.negations.0),
            y: self.swaps.1.value(p) * neg(self.negations.1),
            z: self.swaps.2.value(p) * neg(self.negations.2),
        }
    }

    fn valid(&self) -> bool {
        // all Axes must be unique
        if self.swaps.0 == self.swaps.1
            || self.swaps.0 == self.swaps.2
            || self.swaps.1 == self.swaps.2
        {
            return false;
        }

        let num_diffs = {
            (self.swaps.0 != Axis::X) as u8
                + (self.swaps.1 != Axis::Y) as u8
                + (self.swaps.2 != Axis::Z) as u8
        };
        let num_swaps = match num_diffs {
            0 => 0,
            2 => 1,
            3 => 2,
            _ => panic!("impossible"),
        };

        // Number of negations must be odd iff number of swaps is odd.
        let negative_parity = num_swaps % 2;
        let negatives =
            { self.negations.0 as u8 + self.negations.1 as u8 + self.negations.2 as u8 };

        if negatives % 2 != negative_parity {
            return false;
        }

        true
    }
}

impl Default for Rotation {
    // The identity rotation.
    fn default() -> Self {
        Rotation {
            swaps: (Axis::X, Axis::Y, Axis::Z),
            negations: (false, false, false),
        }
    }
}

// First applies a rotation, then translation
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash)]
struct Transformation {
    rotation: Rotation,
    translation: Point,
}

impl Transformation {
    fn apply(&self, p: &Point) -> Point {
        self.rotation.apply(&p).translate(&self.translation)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash)]
enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn value(&self, p: &Point) -> i32 {
        match self {
            Axis::X => p.x,
            Axis::Y => p.y,
            Axis::Z => p.z,
        }
    }
}

mod parser {
    use super::Scanner;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, Vec<Scanner>> {
        let point = map(
            tuple((int, tag(","), int, tag(","), int)),
            |(x, _, y, _, z)| (x, y, z).into(),
        );
        let points = separated_list1(line_ending, point);
        let header = delimited(tag("--- scanner "), uint, tag(" ---"));
        let scanner = map(separated_pair(header, line_ending, points), |x| x.into());
        let parser = separated_list1(tuple((line_ending, line_ending)), scanner);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "79")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "3621")
    }

    #[test]
    fn overlap_test() {
        let scanners = parser::parse(EXAMPLE_INPUT).unwrap().1;
        assert_eq!(scanners[0]._id, 0);
        assert_eq!(scanners[1]._id, 1);
        let ans = scanners[0].overlap(
            &scanners[1],
            &Point::from((-618, -824, -621)),
            &Point::from((686, 422, 578)),
        );
        assert_eq!(
            ans,
            Some(Transformation {
                rotation: Rotation {
                    swaps: (Axis::X, Axis::Y, Axis::Z),
                    negations: (true, false, true)
                },
                translation: (68, -1246, -43).into()
            })
        );
    }

    #[test]
    fn find_overlap_test() {
        let scanners = parser::parse(EXAMPLE_INPUT).unwrap().1;
        assert_eq!(scanners[0]._id, 0);
        assert_eq!(scanners[1]._id, 1);
        let ans = scanners[0].find_overlap(
            &scanners[1],
        );
        assert_eq!(
            ans,
            Some(Transformation {
                rotation: Rotation {
                    swaps: (Axis::X, Axis::Y, Axis::Z),
                    negations: (true, false, true)
                },
                translation: (68, -1246, -43).into()
            })
        );
    }

    #[test]
    fn rotations_to_equal_test() {
        let a = Point::from((-618, -824, -621)).diff(&(-537, -823, -458).into());
        let b = Point::from((686, 422, 578)).diff(&(605, 423, 415).into());

        assert_eq!(
            Rotation::rotations_to_equal(&a, &b),
            vec![Rotation {
                swaps: (Axis::X, Axis::Y, Axis::Z),
                negations: (true, false, true)
            }]
        );
    }
}
