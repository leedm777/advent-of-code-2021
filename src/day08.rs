use std::collections::HashMap;

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

// const BASE_PATTERNS: [&str; 10] = [
//     "abcefg",  // 0
//     "cf",      // 1
//     "acdeg",   // 2
//     "acdfg",   // 3
//     "bcdf",    // 4
//     "abdfg",   // 5
//     "abdefg",  // 6
//     "acf",     // 7
//     "abcdefg", // 8
//     "abcdfg",  // 9
// ];

/*
 * Segment counts:
 *  e: 4 - unique
 *  b: 6 - unique
 *  d: 7
 *  g: 7
 *  a: 8
 *  c: 8
 *  f: 9 - unique
 */

fn solve(entry: &Entry) -> i32 {
    let one = entry
        .patterns
        .iter()
        .find(|s| s.len() == 2)
        .expect("Could not find 1");
    let seven = entry
        .patterns
        .iter()
        .find(|s| s.len() == 3)
        .expect("Could not find 7");
    let four = entry
        .patterns
        .iter()
        .find(|s| s.len() == 4)
        .expect("Could not find 4");
    let eight = entry
        .patterns
        .iter()
        .find(|s| s.len() == 7)
        .expect("Could not find 8");

    let a = seven
        .chars()
        .find(|ch| one.chars().nth(0).unwrap() != *ch && one.chars().nth(1).unwrap() != *ch)
        .expect("Could not find a");

    let mut e = '\0';
    let mut f = '\0';
    let mut b = '\0';

    for ch in 'a'..='g' {
        let count = entry.patterns.iter().filter(|p| p.contains(ch)).count();
        if count == 4 {
            e = ch;
        } else if count == 6 {
            b = ch;
        } else if count == 9 {
            f = ch;
        }
    }

    let c = one.chars().find(|c| *c != f).expect("Could not find c");
    let d = four
        .chars()
        .find(|ch| ![b, c, f].contains(ch))
        .expect("Could not find d");
    let g = ('a'..='g')
        .find(|ch| ![a, b, c, d, e, f].contains(ch))
        .expect("Could not find g");

    fn encode(s: &str) -> i32 {
        s.chars()
            .map(|ch| ch as i32 - 'a' as i32)
            .fold(0, |a, b| a | (1 << b))
    }
    let zero = encode(&[a, b, c, e, f, g].iter().collect::<String>());
    let one = encode(one);
    let two = encode(&[a, c, d, e, g].iter().collect::<String>());
    let three = encode(&[a, c, d, f, g].iter().collect::<String>());
    let four = encode(four);
    let five = encode(&[a, b, d, f, g].iter().collect::<String>());
    let six = encode(&[a, b, d, e, f, g].iter().collect::<String>());
    let seven = encode(seven);
    let eight = encode(eight);
    let nine = encode(&[a, b, c, d, f, g].iter().collect::<String>());

    let mut m = HashMap::new();
    m.insert(zero, 0);
    m.insert(one, 1);
    m.insert(two, 2);
    m.insert(three, 3);
    m.insert(four, 4);
    m.insert(five, 5);
    m.insert(six, 6);
    m.insert(seven, 7);
    m.insert(eight, 8);
    m.insert(nine, 9);

    let mut r = 0;
    for i in 0..4 {
        let v = m
            .get(&encode(&entry.output[i]))
            .expect("Could not decode output");
        r = r * 10 + v;
    }
    return r;
}

pub fn part2(input: &Vec<Entry>) -> i32 {
    // for ch in 'a'..='g' {
    //     let count = BASE_PATTERNS
    //         .to_vec()
    //         .iter()
    //         .filter(|p| p.contains(ch))
    //         .count();
    //     println!("{}: {}", ch, count);
    // }
    input.iter().map(solve).sum()
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
        assert_eq!(actual, 495);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 61229);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 1055164);
    }
}
