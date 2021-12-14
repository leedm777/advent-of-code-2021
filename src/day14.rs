use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polymerization {
    polymer_template: String,
    pair_insertion_rules: HashMap<(char, char), char>,
}

impl Polymerization {
    fn step(&mut self) {
        let chars = self.polymer_template.chars().collect::<Vec<char>>();
        let insertions = chars
            .windows(2)
            .map(|chars| {
                let ch1 = chars[0];
                let ch2 = chars[1];
                let &insert = self
                    .pair_insertion_rules
                    .get(&(ch1, ch2))
                    .expect("Could not find insertion rule");
                insert
            })
            .collect::<Vec<char>>();

        let len = self.polymer_template.len();
        for (idx, &ch) in insertions.iter().rev().enumerate() {
            self.polymer_template.insert(len - idx - 1, ch);
        }
    }
}

pub fn parse(input: &str) -> Polymerization {
    let template = input.lines().next().expect("Could not find template");
    let rules = input
        .lines()
        .skip(2)
        .map(|line| {
            let (ab, c) = line.split_once(" -> ").expect("Could not split rule");
            let mut ab = ab.chars();
            let a = ab.next().expect("Could not find a");
            let b = ab.next().expect("Could not find b");
            let c = c.chars().next().expect("Could not find c");

            ((a, b), c)
        })
        .collect();

    Polymerization {
        polymer_template: template.to_string(),
        pair_insertion_rules: rules,
    }
}

pub fn part1(poly: &Polymerization) -> i32 {
    let mut poly = poly.clone();

    for _ in 0..10 {
        poly.step();
    }

    let mut char_counts = HashMap::<char, i32>::new();
    for ch in poly.polymer_template.chars() {
        *char_counts.entry(ch).or_insert(0) += 1;
    }

    let max = char_counts.values().max().expect("Could not find max");
    let min = char_counts.values().min().expect("Could not find min");

    max - min
}

pub fn part2(_input: &Polymerization) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "NNCB", "", "CH -> B", "HH -> N", "CB -> H", "NH -> C", "HB -> C", "HC -> B",
            "HN -> C", "NN -> C", "BH -> H", "NC -> B", "NB -> B", "BN -> B", "BB -> N", "BC -> B",
            "CC -> N", "CN -> C",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(14)
    }

    #[test]
    fn test_parse_ex1() {
        let actual = parse(&ex1());
        assert_eq!(
            actual,
            Polymerization {
                polymer_template: "NNCB".to_string(),
                pair_insertion_rules: [
                    (('C', 'H'), 'B'),
                    (('H', 'H'), 'N'),
                    (('C', 'B'), 'H'),
                    (('N', 'H'), 'C'),
                    (('H', 'B'), 'C'),
                    (('H', 'C'), 'B'),
                    (('H', 'N'), 'C'),
                    (('N', 'N'), 'C'),
                    (('B', 'H'), 'H'),
                    (('N', 'C'), 'B'),
                    (('N', 'B'), 'B'),
                    (('B', 'N'), 'B'),
                    (('B', 'B'), 'N'),
                    (('B', 'C'), 'B'),
                    (('C', 'C'), 'N'),
                    (('C', 'N'), 'C'),
                ]
                .into_iter()
                .collect()
            }
        )
    }
    #[test]
    fn test_step_ex1() {
        let mut actual = parse(&ex1());
        actual.step();
        assert_eq!(actual.polymer_template, "NCNBCHB");
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 1588);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 2768);
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
