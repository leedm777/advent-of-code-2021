use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum FoldDir {
    X,
    Y,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TransparentPaper {
    dots: HashSet<(i32, i32)>,
    folds: Vec<(FoldDir, i32)>,
}

impl TransparentPaper {
    fn fold(&mut self) {
        let mut moves: Vec<(i32, i32)> = vec![];
        let (fold_dir, fold_val) = self.folds.pop().expect("Ran out of folds");

        self.dots.iter().for_each(|(dot_x, dot_y)| {
            if fold_dir == FoldDir::X && dot_x >= &fold_val {
                moves.push((*dot_x, *dot_y));
            } else if fold_dir == FoldDir::Y && dot_y >= &fold_val {
                moves.push((*dot_x, *dot_y));
            }
        });

        moves.iter().for_each(|(old_x, old_y)| {
            if fold_dir == FoldDir::X {
                let new_x = 2 * fold_val - old_x;
                self.dots.remove(&(*old_x, *old_y));
                self.dots.insert((new_x, *old_y));
            } else if fold_dir == FoldDir::Y {
                let new_y = 2 * fold_val - old_y;
                self.dots.remove(&(*old_x, *old_y));
                self.dots.insert((*old_x, new_y));
            }
        })
    }
}

fn pi32(input: &str) -> i32 {
    input.parse().expect("Could not parse i32")
}

pub fn parse(input: &str) -> TransparentPaper {
    let dots = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|line| {
            let (x, y) = line.split_once(",").expect("Could not parse dot");
            (pi32(x), pi32(y))
        })
        .collect();

    let mut folds: Vec<(FoldDir, i32)> = input
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
            let val = pi32(val);
            (dir, val)
        })
        .collect();

    folds.reverse();

    TransparentPaper { dots, folds }
}

pub fn part1(paper: &TransparentPaper) -> usize {
    let mut paper = paper.clone();
    paper.fold();

    // for y in 0..=14 {
    //     for x in 0..=10 {
    //         let dot = paper.dots.contains(&(x, y));
    //         if dot {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }

    paper.dots.len()
}

pub fn part2(paper: &TransparentPaper) -> String {
    let mut paper = paper.clone();
    while !paper.folds.is_empty() {
        paper.fold();
    }

    // The actual challenge was to read the characters that are generated. I'm not going to do
    // that sort of thing, so instead we'll just print it when we need it and stick to counting
    // dots

    let &max_x = paper.dots.iter().map(|(x, _)| x).max().expect("No dots?");
    let &max_y = paper.dots.iter().map(|(_, y)| y).max().expect("No dots?");

    let mut r = "\n".to_string();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let dot = paper.dots.contains(&(x, y));
            if dot {
                r += "#";
            } else {
                r += " ";
            }
        }
        r += "\n";
    }

    r
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
                    (6, 10),
                    (0, 14),
                    (9, 10),
                    (0, 3),
                    (10, 4),
                    (4, 11),
                    (6, 0),
                    (6, 12),
                    (4, 1),
                    (0, 13),
                    (10, 12),
                    (3, 4),
                    (3, 0),
                    (8, 4),
                    (1, 10),
                    (2, 14),
                    (8, 10),
                    (9, 0),
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
        assert_eq!(actual, 942);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(
            actual,
            vec!["", "#####", "#   #", "#   #", "#   #", "#####", ""].join("\n")
        );
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(
            actual,
            vec![
                "",
                "  ## ####  ##  #  #  ##  ###  ###  ### ",
                "   #    # #  # #  # #  # #  # #  # #  #",
                "   #   #  #    #  # #  # #  # #  # ### ",
                "   #  #   # ## #  # #### ###  ###  #  #",
                "#  # #    #  # #  # #  # #    # #  #  #",
                " ##  ####  ###  ##  #  # #    #  # ### ",
                ""
            ]
            .join("\n")
        );
    }
}
