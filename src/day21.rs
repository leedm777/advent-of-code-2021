#[derive(Debug, Eq, PartialEq)]
pub struct GameBoard {
    player1: u32,
    player2: u32,
}

pub fn parse(input: &str) -> GameBoard {
    let mut lines = input.lines();
    let p1 = lines.next().expect("Missing line 1");
    let p2 = lines.next().expect("Missing line 2");

    let re =
        regex::Regex::new(r"Player [12] starting position: (?P<pos>\d+)").expect("Invalid regex");

    let player1: u32 = re
        .captures(p1)
        .expect("Invalid line 1")
        .name("pos")
        .expect("Line 1 missing pos")
        .as_str()
        .parse()
        .expect("Invalid position 1");
    let player2: u32 = re
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
    let mut score1 = 0u32;
    let mut score2 = 0u32;
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

    score1.min(score2) * num_rolls
}

pub fn part2(starting: &GameBoard) -> usize {
    0
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
