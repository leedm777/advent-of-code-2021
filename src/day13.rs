use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub enum FoldDir {
    X,
    Y,
}

#[derive(Debug, Eq, PartialEq)]
pub struct TransparentPaper {
    dots: HashSet<(u32, u32)>,
    folds: Vec<(FoldDir, u32)>,
}

fn pu32(input: &str) -> u32 {
    input.parse().expect("Could not parse u32")
}

pub fn parse(input: &str) -> TransparentPaper {
    let dots = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(",").expect("Could not parse dot");
            (pu32(x), pu32(y))
        })
        .collect();

    let mut folds: Vec<(FoldDir, u32)> = input
        .lines()
        .skip_while(|s| !s.is_empty())
        .skip(1)
        .map(|line| {
            let (dir, val) = line.split_once("=").expect("Could not parse fold");
            let dir = if dir == "fold along x" {
                FoldDir::X
            } else {
                FoldDir::Y
            };
            let val = pu32(val);
            (dir, val)
        })
        .collect();

    folds.reverse();

    TransparentPaper { dots, folds }
}

pub fn part1(_input: &TransparentPaper) -> i32 {
    0
}

pub fn part2(_input: &TransparentPaper) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "6,10",
            "0,14",
            "9,10",
            "0,3",
            "10,4",
            "4,11",
            "6,0",
            "6,12",
            "4,1",
            "0,13",
            "10,12",
            "3,4",
            "3,0",
            "8,4",
            "1,10",
            "2,14",
            "8,10",
            "9,0",
            "",
            "fold along y=7",
            "fold along x=5",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(13)
    }

    #[test]
    fn test_parse_ex1() {
        let actual = parse(&ex1());
        let expected = TransparentPaper {
            dots: HashSet::from_iter(
                [
                    (6u32, 10u32),
                    (0u32, 14u32),
                    (9u32, 10u32),
                    (0u32, 3u32),
                    (10u32, 4u32),
                    (4u32, 11u32),
                    (6u32, 0u32),
                    (6u32, 12u32),
                    (4u32, 1u32),
                    (0u32, 13u32),
                    (10u32, 12u32),
                    (3u32, 4u32),
                    (3u32, 0u32),
                    (8u32, 4u32),
                    (1u32, 10u32),
                    (2u32, 14u32),
                    (8u32, 10u32),
                    (9u32, 0u32),
                ]
                .iter()
                .cloned(),
            ),
            folds: vec![(FoldDir::X, 5), (FoldDir::Y, 7)],
        };
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 17);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 0);
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
