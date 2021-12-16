#[derive(Clone)]
struct Board {
    grid: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl Board {
    fn play(&mut self, n: i32) {
        for row_number in 0..5 {
            for col_number in 0..5 {
                if self.grid[row_number][col_number] == n {
                    self.marked[row_number][col_number] = true;
                }
            }
        }
    }

    fn is_winner(&self) -> bool {
        for row_number in 0..5 {
            if self.marked[row_number] == [true; 5] {
                return true;
            }
        }

        'outer: for col_number in 0..5 {
            for row_number in 0..5 {
                if !self.marked[row_number][col_number] {
                    continue 'outer;
                }
            }
            return true;
        }

        false
    }

    fn score(&self) -> i32 {
        let mut r = 0;

        for col_number in 0..5 {
            for row_number in 0..5 {
                if !self.marked[row_number][col_number] {
                    r += self.grid[row_number][col_number]
                }
            }
        }

        r
    }
}

#[derive(Clone)]
pub struct Game {
    moves: Vec<i32>,
    boards: Vec<Board>,
}

impl Game {
    fn play(&mut self) -> i32 {
        let n = self.moves.pop().unwrap();
        for board in &mut self.boards {
            board.play(n);
        }
        n
    }

    fn find_winner(&self) -> Option<&Board> {
        return self.boards.iter().find(|board| board.is_winner());
    }
}

fn parse_board(input: &[&str]) -> Board {
    let mut r = Board {
        grid: [[0; 5]; 5],
        marked: [[false; 5]; 5],
    };

    for row_number in 0..5 {
        let mut row = input[row_number].split_whitespace();
        for col_number in 0..5 {
            let cell = row.next().unwrap().parse::<i32>().unwrap();
            r.grid[row_number][col_number] = cell;
        }
    }

    r
}

fn parse_game(input: &Vec<&str>) -> Game {
    let mut splits = input.split(|line| line.is_empty());
    let mut moves: Vec<i32> = splits
        .next()
        .unwrap()
        .first()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    moves.reverse();
    let boards = splits;
    Game {
        moves,
        boards: boards.map(parse_board).collect(),
    }
}

pub fn parse(input: &str) -> Game {
    let lines: Vec<&str> = input.lines().collect();
    parse_game(&lines)
}

pub fn part1(game: &Game) -> i32 {
    let mut game = game.clone();

    let mut n = game.play();
    let mut winner = game.find_winner();

    loop {
        match winner {
            Some(w) => {
                return n * w.score();
            }
            None => {
                n = game.play();
                winner = game.find_winner();
            }
        }
    }
}

pub fn part2(game: &Game) -> i32 {
    let mut game = game.clone();

    let mut n = game.play();
    let mut winner = game.find_winner();

    loop {
        match winner {
            Some(w) => {
                if game.boards.len() == 1 {
                    return n * w.score();
                } else {
                    game.boards.retain(|b| !b.is_winner());
                    winner = None;
                }
            }
            None => {
                n = game.play();
                winner = game.find_winner();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex0() -> String {
        [
            "7,4,9,5",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
        ]
        .join("\n")
    }

    fn ex1() -> Game {
        parse_game(&vec![
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1",
            "",
            "22 13 17 11  0",
            " 8  2 23  4 24",
            "21  9 14 16  7",
            " 6 10  3 18  5",
            " 1 12 20 15 19",
            "",
            " 3 15  0  2 22",
            " 9 18 13 17  5",
            "19  8  7 25 23",
            "20 11 10 24  4",
            "14 21 16 12  6",
            "",
            "14 21 17 24  4",
            "10 16 15  9 19",
            "18  8 23 26 20",
            "22 11 13  6  5",
            " 2  0 12  3  7",
        ])
    }

    fn real() -> Game {
        parse(&util::read_input(4))
    }

    #[test]
    fn test_parse_game() {
        let actual = parse(&ex0());
        let expected = Game {
            moves: vec![5, 9, 4, 7], //[7, 4, 9, 5],
            boards: vec![Board {
                grid: [
                    [22, 13, 17, 11, 0],
                    [8, 2, 23, 4, 24],
                    [21, 9, 14, 16, 7],
                    [6, 10, 3, 18, 5],
                    [1, 12, 20, 15, 19],
                ],
                marked: [
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                    [false, false, false, false, false],
                ],
            }],
        };
        assert_eq!(actual.moves, expected.moves);
        assert_eq!(actual.boards[0].grid, expected.boards[0].grid);
        assert_eq!(actual.boards[0].marked, expected.boards[0].marked);
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 4512);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 55770);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 1924);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 2980);
    }
}
