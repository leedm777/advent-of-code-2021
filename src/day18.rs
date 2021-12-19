#[derive(Clone)]
enum SnailfishElement {
    Number(u32),
    Pair(Box<SnailfishNumber>),
}

impl SnailfishElement {
    fn parse(input: &str) -> (SnailfishElement, &str) {
        let mut iter = input.chars();
        match iter.next() {
            Some('[') => {
                let (left, rem) = SnailfishElement::parse(&input[1..]);
                if rem.chars().next() != Some(',') {
                    panic!("Invalid pair; missing comma");
                }
                let (right, rem) = SnailfishElement::parse(&rem[1..]);
                if rem.chars().next() != Some(']') {
                    panic!("Invalid pair; missing bracket");
                }
                let number = SnailfishNumber { left, right };
                (SnailfishElement::Pair(Box::new(number)), &rem[1..])
            }
            Some(n) => {
                let number = SnailfishElement::Number(n.to_digit(10).expect("Invalid number"));
                (number, &input[1..])
            }
            None => panic!("Invalid element"),
        }
    }
}
impl ToString for SnailfishElement {
    fn to_string(&self) -> String {
        match self {
            SnailfishElement::Number(n) => n.to_string(),
            SnailfishElement::Pair(p) => p.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct SnailfishNumber {
    left: SnailfishElement,
    right: SnailfishElement,
}

impl SnailfishNumber {
    fn parse(input: &str) -> Self {
        match SnailfishElement::parse(input) {
            (SnailfishElement::Pair(p), rem) => {
                if !rem.is_empty() {
                    panic!("Invalid number (trailing data)")
                }

                *p
            }
            _ => panic!("Invalid number"),
        }
    }

    fn sum(&self, rhs: &Self) -> Self {
        let mut r = Self {
            left: SnailfishElement::Pair(Box::new(self.clone())),
            right: SnailfishElement::Pair(Box::new(rhs.clone())),
        };

        r.reduce();

        r
    }

    fn reduce(&mut self) {
        // TODO
    }
}

impl ToString for SnailfishNumber {
    fn to_string(&self) -> String {
        format!("[{},{}]", self.left.to_string(), self.right.to_string())
    }
}

pub fn parse(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(SnailfishNumber::parse).collect()
}

pub fn part1(_input: &Vec<SnailfishNumber>) -> i32 {
    0
}

pub fn part2(_input: &Vec<SnailfishNumber>) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
            "[[[5,[2,8]],4],[5,[[9,9],0]]]",
            "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
            "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
            "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
            "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
            "[[[[5,4],[7,7]],8],[[8,3],8]]",
            "[[9,3],[[9,9],[6,[4,9]]]]",
            "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
            "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(18)
    }

    #[test]
    fn test_parse_1() {
        let input = "[1,2]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_2() {
        let input = "[[1,2],3]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_3() {
        let input = "[9,[8,7]]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_4() {
        let input = "[[1,9],[8,5]]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_5() {
        let input = "[[[[1,2],[3,4]],[[5,6],[7,8]]],9]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_6() {
        let input = "[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }
    #[test]
    fn test_parse_7() {
        let input = "[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]";
        let actual = SnailfishNumber::parse(input);
        assert_eq!(actual.to_string(), input);
    }

    #[test]
    fn test_sum_1() {
        // [1,2] + [[3,4],5]
        let lhs = SnailfishNumber::parse("[1,2]");
        let rhs = SnailfishNumber::parse("[[3,4],5]");

        let actual = lhs.sum(&rhs);
        assert_eq!(actual.to_string(), "[[1,2],[[3,4],5]]");
    }

    #[test]
    fn test_sum_2() {
        let actual = ["[1,1]", "[2,2]", "[3,3]", "[4,4]"]
            .iter()
            .map(|&s| SnailfishNumber::parse(s))
            .reduce(|n1, n2| n1.sum(&n2))
            .unwrap();

        assert_eq!(actual.to_string(), "[[[[1,1],[2,2]],[3,3]],[4,4]]");
    }

    #[test]
    fn test_sum_3() {
        let actual = ["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"]
            .iter()
            .map(|&s| SnailfishNumber::parse(s))
            .reduce(|n1, n2| n1.sum(&n2))
            .unwrap();

        assert_eq!(actual.to_string(), "[[[[3,0],[5,3]],[4,4]],[5,5]]")
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 4140);
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
