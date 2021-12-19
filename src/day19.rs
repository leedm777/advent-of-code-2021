pub struct Puzzle {
}

pub fn parse(input: &str) -> Puzzle {
    input.lines().collect()
}

pub fn part1(_input: &Puzzle) -> i32 {
    0
}

pub fn part2(_input: &Puzzle) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "TODO",
        ]
        .join("\n")
    }

    fn real() -> String {
       util::read_input(19)
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 0);
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
