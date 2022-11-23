use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::lib::heapentry::MinHeapEntry;

pub fn problem1(input: &str) -> String {
    let data = parser::parse(input).unwrap().1;
    let burrow = Burrow::new(data);
    let ans = bfs(burrow).unwrap();
    format!("{}", ans)
}

pub fn problem2(input: &str) -> String {
    let mut lines: Vec<_> = input.lines().collect();
    lines.splice(3..3, ["#D#C#B#A#", "#D#B#A#C#"]);
    let spliced_input = lines.iter().fold(String::new(), |mut acc, x| {
        acc.push_str(x);
        acc.push('\n');
        acc
    });
    let data = parser::parse(&spliced_input).unwrap().1;
    let burrow = Burrow::new(data);
    let ans = bfs(burrow).unwrap();
    format!("{}", ans)
}

fn bfs(initial: Burrow) -> Option<usize> {
    let mut frontier = BinaryHeap::new();
    frontier.push(MinHeapEntry::new(0, initial));

    while let Some(MinHeapEntry {
        key: mut cost,
        value: mut state,
    }) = frontier.pop()
    {
        if state.is_complete() {
            return Some(cost);
        }

        // Attempt to move any possible to siderooms.
        loop {
            let mut moved = false;

            // Move from hall to sideroom
            let hall_occupants: Vec<_> = state
                .hall
                .iter()
                .enumerate()
                .filter_map(|(i, &a)| Some((i, a?)))
                .collect();

            for (i, a) in hall_occupants {
                if let Some(c) = state.move_to_room(a, i) {
                    moved = true;
                    cost += c;
                    state.hall[i] = None;
                }
            }

            // move from sideroom to sideroom
            let top_occupants: Vec<_> = state
                .iter_rooms()
                .filter_map(|(room_id, room)| Some((room_id, room.peak()?)))
                .collect();

            for (room_id, (leave_room_cost, a)) in top_occupants {
                if let Some(to_room_cost) = state.move_to_room(a, room_id.room_entrance()) {
                    state
                        .get_room_mut(room_id)
                        .pop()
                        .expect("peak worked so pop should also");
                    cost += leave_room_cost + to_room_cost;
                    moved = true;
                }
            }

            if !moved {
                break;
            }
        }

        if state.is_complete() {
            frontier.push(MinHeapEntry::new(cost, state));
            continue;
        }

        // move every top occupant to every possible hall location.
        for i in 0..state.rooms.len() {
            let mut pop_state = state.clone();
            let Some((to_hall_cost, a)) = pop_state.rooms[i].pop() else {continue;};
            let entrance = pop_state.rooms[i].wanted.room_entrance();
            let open_hall_locs = pop_state.open_hall_range(entrance);
            for hall_loc in open_hall_locs {
                let hall_cost = hall_loc.abs_diff(entrance) * a.energy();
                let mut s = pop_state.clone();
                s.hall[hall_loc] = Some(a);
                frontier.push(MinHeapEntry::new(cost + to_hall_cost + hall_cost, s));
            }
        }
    }

    None
}

const HALLWAY_LENGTH: usize = 11;

#[derive(Clone, PartialEq, Eq, Debug)]
struct Room {
    // Amphipods wanted in the room
    wanted: Amphipod,
    // total size of room
    size: usize,

    // number of spaces with the wanted Amphipod in front
    filled: usize,
    // stack of Amphipods that need to leave the room.
    cells: Vec<Amphipod>,
}

impl Room {
    fn new(wanted: Amphipod, mut cells: Vec<Amphipod>) -> Self {
        let size = cells.len();
        let mut filled = 0;

        cells.reverse();

        while cells.first().map(|&x| x == wanted).unwrap_or(false) {
            cells.remove(0);
            filled += 1;
        }

        Self {
            wanted,
            size,
            filled,
            cells,
        }
    }

    // Returns energy to push if push was successful. It is assumed you are
    // placing the only allowed Amphipod.
    fn push(&mut self) -> Option<usize> {
        if self.cells.len() != 0 {
            // Cannot put one in if you haven't removed all the unwanted ones.
            return None;
        }

        if self.filled >= self.size {
            panic!("overfilled sideroom");
        }

        let steps = self.size - self.filled;
        self.filled += 1;

        Some(steps * self.wanted.energy())
    }

    // Returns energy to leave room and value removed.
    fn pop(&mut self) -> Option<(usize, Amphipod)> {
        let ret = self.cells.pop()?;
        let steps = self.size - self.spaces_taken();
        Some((steps * ret.energy(), ret))
    }

    fn peak(&self) -> Option<(usize, Amphipod)> {
        let ret = self.cells.last()?;
        let steps = self.size - self.spaces_taken() + 1;
        Some((steps * ret.energy(), *ret))
    }

    fn is_complete(&self) -> bool {
        self.filled == self.size
    }

    fn spaces_taken(&self) -> usize {
        self.filled + self.cells.len()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Burrow {
    hall: [Option<Amphipod>; HALLWAY_LENGTH],
    rooms: [Room; 4],
}

impl Burrow {
    fn new(room_occupants: [Vec<Amphipod>; 4]) -> Self {
        let [a, b, c, d] = room_occupants;
        let rooms = [
            Room::new(Amphipod::A, a),
            Room::new(Amphipod::B, b),
            Room::new(Amphipod::C, c),
            Room::new(Amphipod::D, d),
        ];

        Self {
            hall: [None; HALLWAY_LENGTH],
            rooms,
        }
    }

    fn is_complete(&self) -> bool {
        self.rooms.iter().all(|x| x.is_complete())
    }

    fn get_room_mut(&mut self, a: Amphipod) -> &mut Room {
        match a {
            Amphipod::A => &mut self.rooms[0],
            Amphipod::B => &mut self.rooms[1],
            Amphipod::C => &mut self.rooms[2],
            Amphipod::D => &mut self.rooms[3],
        }
    }

    // Returns the steps of moving a value from one location to another in the
    // hall. Returns None if the move is blocked.
    fn hall_move_steps(&self, from: usize, to: usize) -> Option<usize> {
        let section = match from.cmp(&to) {
            Ordering::Equal => return Some(0),
            Ordering::Greater => &self.hall[to..from],
            Ordering::Less => &self.hall[from + 1..to + 1],
        };

        if section.iter().all(|&x| x == None) {
            Some(from.abs_diff(to))
        } else {
            None
        }
    }

    // Returns the energy used to move if successful.
    fn move_to_room(&mut self, a: Amphipod, from_hall: usize) -> Option<usize> {
        let hall_steps = self.hall_move_steps(from_hall, a.room_entrance())?;
        let room_cost = self.get_room_mut(a).push()?;
        Some(hall_steps * a.energy() + room_cost)
    }

    fn iter_rooms(&self) -> impl Iterator<Item = (Amphipod, &Room)> {
        [
            (Amphipod::A, &self.rooms[0]),
            (Amphipod::B, &self.rooms[1]),
            (Amphipod::C, &self.rooms[2]),
            (Amphipod::D, &self.rooms[3]),
        ]
        .into_iter()
    }

    // Returns the largest unoccupied range containing mid in the form of [i, j].
    // mid may be occupied.
    fn open_hall_range(&self, mid: usize) -> impl Iterator<Item = usize> {
        let start = (&self.hall[..mid])
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, &x)| x != None)
            .next()
            .map(|(i, _)| i + 1)
            .unwrap_or(0);

        let end = (&self.hall[mid + 1..])
            .iter()
            .enumerate()
            .filter(|(_, &x)| x != None)
            .next()
            .map(|(i, _)| i + mid + 1)
            .unwrap_or(self.hall.len());

        (start..end).filter(|x| ![2, 4, 6, 8].contains(x))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn energy(&self) -> usize {
        match &self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn room_entrance(&self) -> usize {
        match &self {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        }
    }
}

impl TryFrom<char> for Amphipod {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Amphipod::A),
            'B' => Ok(Amphipod::B),
            'C' => Ok(Amphipod::C),
            'D' => Ok(Amphipod::D),
            _ => Err(()),
        }
    }
}

mod parser {
    use super::*;
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, [Vec<Amphipod>; 4], &'static str> {
        let amphipods: Vec<_> = input
            .chars()
            .filter_map(|c| Amphipod::try_from(c).ok())
            .collect();

        if amphipods.len() % 4 != 0 {
            return IResult::Err(nom::Err::Error("unexpected number of amphipods"));
        }

        let mut ret = [vec![], vec![], vec![], vec![]];
        for (i, a) in amphipods.into_iter().enumerate() {
            ret[i % 4].push(a);
        }

        IResult::Ok((&input[input.len()..], ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "#############
#...........#
###B#C#B#D###
    #A#D#C#A#
    #########";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "12521")
    }

    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "44169")
    }

    #[test]
    fn test_open_hall_range() {
        let mut b = Burrow::new([vec![], vec![], vec![], vec![]]);
        assert_eq!(
            b.open_hall_range(2).collect::<Vec<_>>(),
            vec![0, 1, 3, 5, 7, 9, 10],
        );

        b.hall[4] = Some(Amphipod::A);
        b.hall[8] = Some(Amphipod::A);

        assert_eq!(b.open_hall_range(1).collect::<Vec<_>>(), vec![0, 1, 3]);
        assert_eq!(
            b.open_hall_range(4).collect::<Vec<_>>(),
            vec![0, 1, 3, 5, 7]
        );
        assert_eq!(b.open_hall_range(5).collect::<Vec<_>>(), vec![5, 7]);
        assert_eq!(b.open_hall_range(10).collect::<Vec<_>>(), vec![9, 10]);
    }
}
