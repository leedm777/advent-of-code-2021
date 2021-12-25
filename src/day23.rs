enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Amphipod {
    fn parse(ch: char) -> Amphipod {
        match ch {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            'D' => Amphipod::Desert,
            _ => panic!("Invalid char {}", ch),
        }
    }

    fn energy(&self) -> usize {
        match self {
            Amphipod::Amber => 1,
            Amphipod::Bronze => 10,
            Amphipod::Copper => 100,
            Amphipod::Desert => 1000,
        }
    }
}

pub struct AmphipodBurrow {
    total_energy: usize,
}

pub fn parse(input: &str) -> AmphipodBurrow {
    // maybe the maze is a digraph, marking the goals for the different rooms?
    AmphipodBurrow { total_energy: 0 }
}

pub fn part1(_input: &AmphipodBurrow) -> i32 {
    0
}

pub fn part2(_input: &AmphipodBurrow) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "#############",
            "#...........#",
            "###B#C#B#D###",
            "  #A#D#C#A#",
            "  #########",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(23)
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 12521);
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
