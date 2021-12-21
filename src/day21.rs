#[derive(Debug, Eq, PartialEq)]
pub struct GameBoard {
    player1: u16,
    player2: u16,
}

pub fn parse(input: &str) -> GameBoard {
    let mut lines = input.lines();
    let p1 = lines.next().expect("Missing line 1");
    let p2 = lines.next().expect("Missing line 2");

    let re =
        regex::Regex::new(r"Player [12] starting position: (?P<pos>\d+)").expect("Invalid regex");

    let player1: u16 = re
        .captures(p1)
        .expect("Invalid line 1")
        .name("pos")
        .expect("Line 1 missing pos")
        .as_str()
        .parse()
        .expect("Invalid position 1");
    let player2: u16 = re
        .captures(p2)
        .expect("Invalid line 2")
        .name("pos")
        .expect("Line 2 missing pos")
        .as_str()
        .parse()
        .expect("Invalid position 2");

    GameBoard { player1, player2 }
}

pub fn part1(starting: &GameBoard) -> u32 {
    let mut pos1 = starting.player1;
    let mut pos2 = starting.player2;
    let mut next_roll = 1;
    let mut score1 = 0u16;
    let mut score2 = 0u16;
    let mut num_rolls = 0;

    loop {
        let roll = 3 * (next_roll + next_roll + 2) / 2;
        next_roll = (next_roll + 3 - 1) % 100 + 1;
        num_rolls += 3;
        // print!("p1 {} + {} -> ", pos1, roll);
        pos1 = (pos1 + roll - 1) % 10 + 1;
        score1 += pos1;
        // println!("{} => {}", pos1, score1);

        if score1 >= 1000 {
            break;
        }

        let roll = 3 * (next_roll + next_roll + 2) / 2;
        next_roll = (next_roll + 3 - 1) % 100 + 1;
        num_rolls += 3;
        // print!("p2 {} + {} -> ", pos2, roll);
        pos2 = (pos2 + roll - 1) % 10 + 1;
        score2 += pos2;
        // println!("{} => {}", pos2, score2);

        if score2 >= 1000 {
            break;
        }
    }

    score1.min(score2) as u32 * num_rolls
}

#[derive(Clone, Copy)]
struct Player {
    score: u16,
    position: u16,
}

impl Player {
    fn new(position: u16) -> Player {
        Player { score: 0, position }
    }

    fn mv(&self, spaces: u16) -> Player {
        let position = (self.position + spaces - 1) % 10 + 1;
        let score = self.score + self.position;

        Player { score, position }
    }
}

struct ParallelGame {
    player1: Player,
    player2: Player,
    num_universes: usize,
    p1turn: bool,
}

// after each turn there are:
//   1 universe where 3 total is rolled
//   3 universe where 4 total is rolled
//   6 universe where 5 total is rolled
//   7 universe where 6 total is rolled
//   6 universe where 7 total is rolled
//   3 universe where 8 total is rolled
//   1 universe where 9 total is rolled

// num_universes, num_spaces
const QUANTUM_DIE: [[u16; 2]; 7] = [[1, 3], [3, 4], [6, 5], [7, 6], [6, 7], [3, 8], [1, 9]];

impl ParallelGame {
    fn next(&self, num_universes: usize, num_spaces: u16) -> ParallelGame {
        if self.p1turn {
            ParallelGame {
                player1: self.player1.mv(num_spaces),
                player2: self.player2,
                num_universes: self.num_universes * num_universes,
                p1turn: false,
            }
        } else {
            ParallelGame {
                player1: self.player1,
                player2: self.player2.mv(num_spaces),
                num_universes: self.num_universes * num_universes,
                p1turn: true,
            }
        }
    }
    fn play(&self) -> (usize, usize) {
        if self.player1.score >= 21 {
            (self.num_universes, 0)
        } else if self.player2.score >= 21 {
            (0, self.num_universes)
        } else {
            QUANTUM_DIE
                .map(|[num_universes, num_spaces]| {
                    self.next(num_universes as usize, num_spaces).play()
                })
                .iter()
                .fold((0, 0), |(acc1, acc2), (p1, p2)| (acc1 + p1, acc2 + p2))
        }
    }
}

pub fn part2(starting: &GameBoard) -> usize {
    let g = ParallelGame {
        player1: Player::new(starting.player1),
        player2: Player::new(starting.player2),
        num_universes: 1,
        p1turn: true,
    };

    let (p1, p2) = g.play();

    println!("({}, {})", p1, p2);
    p1.max(p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "Player 1 starting position: 4",
            "Player 2 starting position: 8",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(21)
    }

    #[test]
    fn test_parse() {
        let actual = parse(&ex1());
        assert_eq!(
            actual,
            GameBoard {
                player1: 4,
                player2: 8
            }
        );
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 739785);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 1002474);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 444356092776315);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}
