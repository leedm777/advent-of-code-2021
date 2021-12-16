// use std::fmt;

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Octopuses {
    energy_level: Vec<Vec<u8>>,
    flashed: Vec<Vec<bool>>,
}

impl Octopuses {
    fn parse(input: &str) -> Octopuses {
        let mut r = Octopuses {
            energy_level: vec![],
            flashed: vec![],
        };

        for line in input.lines() {
            let mut row = vec![];
            let mut flashed_row = vec![];
            for ch in line.chars() {
                let v = ch.to_digit(10).expect("Could not part digit") as u8;
                row.push(v);
                flashed_row.push(v == 0);
            }
            r.energy_level.push(row);
            r.flashed.push(flashed_row);
        }
        r
    }

    fn next(&self) -> Octopuses {
        let mut r = self.clone();
        let mut just_flashed = vec![];

        let max_y = r.energy_level.len();
        let max_x = r.energy_level[0].len();

        for (y, row) in self.energy_level.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                r.energy_level[y][x] = v + 1;
                if *v == 9u8 {
                    r.flashed[y][x] = true;
                    just_flashed.push((x, y));
                } else {
                    r.flashed[y][x] = false;
                }
            }
        }

        while !just_flashed.is_empty() {
            let (x, y) = just_flashed.pop().expect("It said it wasn't empty");

            crate::util::neighbors(x, y, max_x, max_y)
                .iter()
                .for_each(|(x, y)| {
                    let x = *x;
                    let y = *y;
                    r.energy_level[y][x] += 1;
                    if r.energy_level[y][x] == 10 {
                        r.flashed[y][x] = true;
                        just_flashed.push((x, y));
                    }
                });
        }

        for y in 0..max_y {
            for x in 0..max_x {
                if r.flashed[y][x] {
                    r.energy_level[y][x] = 0;
                }
            }
        }

        r
    }

    // fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    //     let mut s = String::new();
    //     for row in &self.energy_level {
    //         for v in row {
    //             s += &v.to_string();
    //         }
    //         s += "\n";
    //     }
    //
    //     f.debug_struct("Octopuses")
    //         .field("energy_level", &s)
    //         .finish()
    // }
}

pub fn parse(input: &str) -> Octopuses {
    Octopuses::parse(input)
}

pub fn _part1(o: &Octopuses, num_rounds: i32) -> usize {
    let mut num_flashes = 0;
    let mut octopuses = o.clone();
    for _ in 0..num_rounds {
        octopuses = octopuses.next();
        num_flashes += octopuses.flashed.iter().flatten().filter(|e| **e).count();

        // println!("After step {} ({})", step + 1, num_flashes);
        // octopuses.energy_level.iter().for_each(|row| {
        //     row.iter().for_each(|v| {
        //         print!("{}", v);
        //     });
        //     println!();
        // });
        // println!();
    }

    num_flashes
}

pub fn part1(o: &Octopuses) -> usize {
    _part1(o, 100)
}
pub fn part2(o: &Octopuses) -> i32 {
    let mut octopuses = o.clone();
    let mut round = 0;

    while !octopuses.flashed.iter().flatten().all(|f| *f) {
        octopuses = octopuses.next();
        round += 1;
    }
    round
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(11)
    }

    #[test]
    fn test_parse() {
        let actual = parse(&ex1());
        assert_eq!(
            actual,
            Octopuses {
                energy_level: [
                    [5, 4, 8, 3, 1, 4, 3, 2, 2, 3],
                    [2, 7, 4, 5, 8, 5, 4, 7, 1, 1],
                    [5, 2, 6, 4, 5, 5, 6, 1, 7, 3],
                    [6, 1, 4, 1, 3, 3, 6, 1, 4, 6],
                    [6, 3, 5, 7, 3, 8, 5, 4, 7, 8],
                    [4, 1, 6, 7, 5, 2, 4, 6, 4, 5],
                    [2, 1, 7, 6, 8, 4, 1, 7, 2, 1],
                    [6, 8, 8, 2, 8, 8, 1, 1, 3, 4],
                    [4, 8, 4, 6, 8, 4, 8, 5, 5, 4],
                    [5, 2, 8, 3, 7, 5, 1, 5, 2, 6],
                ]
                .iter()
                .map(|a| a.to_vec())
                .collect(),
                flashed: vec![vec![false; 10]; 10],
            },
        )
    }

    #[test]
    fn test_next() {
        let before = parse(&["11111", "19991", "19191", "19991", "11111"].join("\n"));
        let expected = parse(&["34543", "40004", "50005", "40004", "34543"].join("\n"));
        let actual = before.next();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_ex1_small() {
        let actual = _part1(&parse(&ex1()), 10);
        assert_eq!(actual, 204);
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 1656);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 1661);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 195);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 334);
    }
}
