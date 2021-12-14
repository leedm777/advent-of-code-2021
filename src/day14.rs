use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Polymerization {
    polymer_template: String,
    pair_insertion_rules: HashMap<(char, char), char>,
}

impl Polymerization {
    // First non-scalable solution
    // fn step(&mut self) {
    //     let chars = self.polymer_template.chars().collect::<Vec<char>>();
    //     let insertions = chars
    //         .windows(2)
    //         .map(|chars| {
    //             let ch1 = chars[0];
    //             let ch2 = chars[1];
    //             let &insert = self
    //                 .pair_insertion_rules
    //                 .get(&(ch1, ch2))
    //                 .expect("Could not find insertion rule");
    //             insert
    //         })
    //         .collect::<Vec<char>>();
    //
    //     let len = self.polymer_template.len();
    //     for (idx, &ch) in insertions.iter().rev().enumerate() {
    //         self.polymer_template.insert(len - idx - 1, ch);
    //     }
    // }

    fn solve(&self, steps: i32) -> usize {
        let mut pair_counts = HashMap::<(char, char), usize>::new();
        let chars = self.polymer_template.chars().collect::<Vec<char>>();

        chars.windows(2).for_each(|ch| {
            *pair_counts.entry((ch[0], ch[1])).or_insert(0) += 1;
        });

        for _ in 0..steps {
            let mut next_pair_counts = HashMap::<(char, char), usize>::new();

            for ((ch1, ch2), count) in pair_counts {
                let &insert = self
                    .pair_insertion_rules
                    .get(&(ch1, ch2))
                    .expect("Could not find pair");

                *next_pair_counts.entry((ch1, insert)).or_insert(0) += count;
                *next_pair_counts.entry((insert, ch2)).or_insert(0) += count;
            }

            pair_counts = next_pair_counts;
        }

        // count the 2nd char in each pair to count all the characters
        let mut char_counter = HashMap::new();
        pair_counts.iter().for_each(|(&(_, ch2), &count)| {
            *char_counter.entry(ch2).or_insert(0) += count;
        });

        // plus one for the first character
        let first_char = self
            .polymer_template
            .chars()
            .next()
            .expect("Could not find first char");
        *char_counter.entry(first_char).or_insert(0) += 1;

        let &max = char_counter.values().max().expect("Could not find max");
        let &min = char_counter.values().min().expect("Could not find min");

        max - min
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

pub fn part1(poly: &Polymerization) -> usize {
    poly.solve(10)
}

pub fn part2(poly: &Polymerization) -> usize {
    poly.solve(40)
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

    // #[test]
    // fn test_step_ex1() {
    //     let mut actual = parse(&ex1());
    //     actual.step();
    //     assert_eq!(actual.polymer_template, "NCNBCHB");
    // }

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
        assert_eq!(actual, 2188189693529);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 2914365137499);
    }
}
