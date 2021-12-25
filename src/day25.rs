#[derive(Clone, Copy, PartialEq)]
enum Location {
    Empty,
    EastFacing,
    SouthFacing,
}

impl Location {
    fn to_char(&self) -> &'static str {
        match self {
            Location::Empty => ".",
            Location::EastFacing => ">",
            Location::SouthFacing => "v",
        }
    }
}

#[derive(Clone)]
pub struct SeaCucumbers {
    map: Vec<Vec<Location>>,
}

impl ToString for SeaCucumbers {
    fn to_string(&self) -> String {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|loc| loc.to_char())
                    .collect::<Vec<&str>>()
                    .join("")
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl SeaCucumbers {
    fn step(&self) -> (Self, bool) {
        let mut half_step_map = self.map.clone();
        let mut moved = false;

        // move east facing sea cucumbers
        for y in 0..self.map.len() {
            let row = &self.map[y];
            let row_len = row.len();
            for x in 0..row_len {
                let loc = self.map[y][x];

                let west_x = if x == 0 { row_len - 1 } else { x - 1 };
                let west = self.map[y][west_x];

                if loc == Location::Empty && west == Location::EastFacing {
                    moved = true;
                    half_step_map[y][west_x] = Location::Empty;
                    half_step_map[y][x] = Location::EastFacing;
                }
            }
        }

        // move south facing sea cucumbers
        let mut map = half_step_map.clone();
        for y in 0..half_step_map.len() {
            let row = &half_step_map[y];
            for x in 0..row.len() {
                let loc = half_step_map[y][x];

                let north_y = if y == 0 {
                    half_step_map.len() - 1
                } else {
                    y - 1
                };
                let north = half_step_map[north_y][x];

                if loc == Location::Empty && north == Location::SouthFacing {
                    moved = true;
                    map[north_y][x] = Location::Empty;
                    map[y][x] = Location::SouthFacing;
                }
            }
        }

        (SeaCucumbers { map }, moved)
    }
}

pub fn parse(input: &str) -> SeaCucumbers {
    let mut map = vec![];

    for line in input.lines() {
        let mut row = vec![];
        for ch in line.chars() {
            let location = match ch {
                'v' => Location::SouthFacing,
                '>' => Location::EastFacing,
                '.' => Location::Empty,
                _ => panic!("Invalid location"),
            };
            row.push(location);
        }
        map.push(row);
    }

    SeaCucumbers { map }
}

pub fn part1(input: &SeaCucumbers) -> i32 {
    let mut counter = 0;
    let mut input = input.clone();
    let mut moved = true;

    while moved {
        // println!("After {} steps:\n{}\n", counter, input.to_string());
        let step = input.step();
        input = step.0;
        moved = step.1;
        counter += 1;
    }

    counter
}

pub fn part2(_input: &SeaCucumbers) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "v...>>.vv>",
            ".vv>>.vv..",
            ">>.>v>...v",
            ">>v>>.>.v.",
            "v>v.vv.v..",
            ">.>>..v...",
            ".vv..>.>v.",
            "v.v..>>v.v",
            "....v..v.>",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(25)
    }

    fn ex2() -> String {
        vec![
            "...>...", ".......", "......>", "v.....>", "......>", ".......", "..vvv..",
        ]
        .join("\n")
    }

    #[test]
    fn test_parse_ex1() {
        let v = parse(&ex1());
        let actual = v.to_string();
        assert_eq!(actual, ex1());
    }

    #[test]
    fn test_parse_ex2() {
        let v = parse(&ex2());
        let actual = v.to_string();
        assert_eq!(actual, ex2());
    }

    #[test]
    fn test_step_ex2() {
        let v = parse(&ex2());
        let (actual, moved) = v.step();
        let expected = vec![
            "..vv>..", ".......", ">......", "v.....>", ">......", ".......", "....v..",
        ]
        .join("\n");
        assert_eq!(actual.to_string(), expected);
        assert!(moved);
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 58);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 557);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}
