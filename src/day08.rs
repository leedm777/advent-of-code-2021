#[derive(Eq, PartialEq, Debug)]
pub struct Entry {
    patterns: Vec<String>,
    output: [String; 4],
}

fn parse_entry(line: &str) -> Entry {
    let (patterns, output) = line.split_once(" | ").expect("Could not parse line");
    let patterns = patterns.split_whitespace().map(|s| s.to_string()).collect();
    let mut output = output.split_whitespace();
    let output = [
        output.next().expect("Could not parse output").to_string(),
        output.next().expect("Could not parse output").to_string(),
        output.next().expect("Could not parse output").to_string(),
        output.next().expect("Could not parse output").to_string(),
    ];

    return Entry { patterns, output };
}

pub fn parse(input: &str) -> Vec<Entry> {
    return input.lines().map(parse_entry).collect();
}

pub fn part1(input: &Vec<Entry>) -> i32 {
    let matches = input
        .iter()
        .flat_map(|e| e.output.to_vec())
        .filter(|s| [2, 3, 4, 7].contains(&s.len()));
    // matches.clone().for_each(|s| println!("{}", s));
    return matches.count() as i32;
}

pub fn part2(_input: &Vec<Entry>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        return [
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
            "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
            "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
            "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
            "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
            "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
            "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
            "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
        ].join("\n");
    }

    fn real() -> String {
        return util::read_input(8);
    }

    #[test]
    fn test_parse_entry() {
        let actual = parse_entry("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert_eq!(
            actual,
            Entry {
                patterns: vec![
                    "be", "cfbegad", "cbdgef", "fgaecd", "cgeb", "fdcge", "agebfd", "fecdb",
                    "fabcd", "edb"
                ]
                .iter()
                .map(|s| s.to_string())
                .collect(),
                output: ["fdgacbe", "cefdb", "cefbgd", "gcbe"].map(|s| s.to_string()),
            }
        )
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 26);
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
