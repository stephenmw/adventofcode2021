pub fn problem1(input: &str) -> String {
    let (p1, p2) = parser::parse(input).unwrap().1;
    let mut game = Game::new(p1, p2);
    while !game.is_winner() {
        game.play_turn();
    }

    let non_winner_score = game
        .players
        .iter()
        .map(|p| p.score)
        .find(|&s| s < 1000)
        .unwrap();
    let die_rolls = game.die.cur;
    let ans = non_winner_score * die_rolls;
    format!("{}", ans)
}

#[derive(Clone, Debug, Default)]
struct Game {
    die: DeterministicDie,
    players: [PlayerState; 2],
    cur_player: usize,
}

impl Game {
    fn new(p1: u32, p2: u32) -> Self {
        Game {
            die: DeterministicDie::default(),
            players: [PlayerState::new(p1), PlayerState::new(p2)],
            cur_player: 0,
        }
    }

    // Returns winning player, if theree is one
    fn play_turn(&mut self) {
        let n = self.die.roll(3);
        let player = &mut self.players[self.cur_player];
        player.forward(n);
        self.cur_player = (self.cur_player + 1) % self.players.len();
    }

    fn is_winner(&self) -> bool {
        self.players.iter().any(|x| x.score >= 1000)
    }
}

#[derive(Clone, Debug, Default)]
struct DeterministicDie {
    cur: u32,
}

impl DeterministicDie {
    fn roll(&mut self, n: u32) -> u32 {
        (0..n).map(|_| self.roll_once()).sum()
    }

    fn roll_once(&mut self) -> u32 {
        let ret = self.cur % 100 + 1;
        self.cur += 1;
        ret
    }
}

#[derive(Clone, Debug, Default)]
struct PlayerState {
    position: u32,
    score: u32,
}

impl PlayerState {
    fn new(starting_position: u32) -> Self {
        PlayerState {
            position: starting_position,
            score: 0,
        }
    }

    fn forward(&mut self, n: u32) {
        self.position += n;
        while self.position > 10 {
            self.position -= 10;
        }
        self.score += self.position;
    }
}

pub fn problem2(input: &str) -> String {
    let (p1, p2) = parser::parse(input).unwrap().1;
    format!("{}", problem2_mod::worlds_player_wins(p1, p2))
}

mod problem2_mod {
    #[derive(Clone, Debug, Default)]
    struct Turn {
        // worlds[position][score] = world_count
        worlds: [[u64; 21]; 10],
        wins: u64,
    }

    impl Turn {
        fn games_in_progress(&self) -> u64 {
            self.worlds.iter().flat_map(|x| x.iter()).sum()
        }

        fn all_worlds_ended(&self) -> bool {
            self.worlds.iter().flat_map(|x| x.iter()).all(|&x| x == 0)
        }
    }

    // Number of worlds where values [3, 9] are rolled per turn.
    const ROLL_FACTOR: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

    pub fn worlds_player_wins(p1: u32, p2: u32) -> u64 {
        let p1_turns = player_worlds(p1 as usize);
        let p2_turns = player_worlds(p2 as usize);

        let mut p1_wins = 0;
        let mut p2_wins = 0;

        let mut games_in_progress = 0;

        for (t1, t2) in p1_turns.iter().zip(p2_turns.iter()) {
            p1_wins += t1.wins * games_in_progress;
            games_in_progress = t1.games_in_progress();
            p2_wins += t2.wins * games_in_progress;
            games_in_progress = t2.games_in_progress();
        }

        p1_wins.max(p2_wins)
    }

    fn player_worlds(starting_position: usize) -> Vec<Turn> {
        let mut turns = Vec::new();

        let mut last_turn = {
            let mut turn = Turn::default();
            turn.worlds[(starting_position - 1) % 10][0] = 1;
            turn
        };
        turns.push(last_turn.clone());

        while !last_turn.all_worlds_ended() {
            last_turn = next_turn(&last_turn);
            turns.push(last_turn.clone());
        }

        turns
    }

    fn next_turn(prev: &Turn) -> Turn {
        let mut next = Turn::default();

        for roll in 3..=9 {
            for (pos, scores) in prev.worlds.iter().enumerate() {
                let new_pos = (pos + roll) % 10;
                let d_score = new_pos + 1;

                for (score, count) in scores.iter().enumerate() {
                    let new_score = score + d_score;
                    if new_score >= 21 {
                        next.wins += count * ROLL_FACTOR[roll - 3];
                    } else {
                        next.worlds[new_pos][new_score] += count * ROLL_FACTOR[roll - 3];
                    }
                }
            }
        }

        next
    }
}

mod parser {
    use crate::lib::combinators::*;

    pub fn parse(input: &str) -> IResult<&str, (u32, u32)> {
        let p1 = preceded(tag("Player 1 starting position: "), uint);
        let p2 = preceded(tag("Player 2 starting position: "), uint);
        let parser = separated_pair(p1, line_ending, p2);
        complete(parser)(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn problem1_test() {
        assert_eq!(problem1(EXAMPLE_INPUT), "739785")
    }
    #[test]
    fn problem2_test() {
        assert_eq!(problem2(EXAMPLE_INPUT), "444356092776315")
    }
}
