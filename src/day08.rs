use std::collections::HashMap;

#[derive(Debug)]
pub struct Pattern {
    segments: u8,
    num_bits: u8,
}

fn encode(s: &str) -> u8 {
    s.chars()
        .map(|ch| ch as i32 - 'a' as i32)
        .fold(0, |a, b| a | (1 << b))
}

fn parse_pattern(s: &str) -> Pattern {
    let segments = encode(s);
    let bits = s.len() as u8;
    Pattern {
        segments,
        num_bits: bits,
    }
}

#[derive(Debug)]
pub struct Entry {
    patterns: [Pattern; 10],
    output: [Pattern; 4],
}

fn parse_entry(line: &str) -> Entry {
    let (patterns, output) = line.split_once(" | ").expect("Could not parse line");

    let mut patterns = patterns.split_whitespace().map(parse_pattern);
    let patterns = [
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
        patterns.next().expect("Could not parse pattern"),
    ];

    let mut output = output.split_whitespace().map(parse_pattern);
    let output = [
        output.next().expect("Could not parse output"),
        output.next().expect("Could not parse output"),
        output.next().expect("Could not parse output"),
        output.next().expect("Could not parse output"),
    ];

    return Entry { patterns, output };
}

pub fn parse(input: &str) -> Vec<Entry> {
    return input.lines().map(parse_entry).collect();
}

pub fn part1(input: &Vec<Entry>) -> usize {
    let matches = input.iter().flat_map(|e| {
        e.output
            .iter()
            .filter(|p| [2, 3, 4, 7].contains(&p.num_bits))
    });
    return matches.count();
}

fn solve(entry: &Entry) -> i32 {
    let one = entry
        .patterns
        .iter()
        .find(|p| p.num_bits == 2)
        .expect("Could not find 1");
    let seven = entry
        .patterns
        .iter()
        .find(|p| p.num_bits == 3)
        .expect("Could not find 7");
    let four = entry
        .patterns
        .iter()
        .find(|p| p.num_bits == 4)
        .expect("Could not find 4");
    let eight = entry
        .patterns
        .iter()
        .find(|p| p.num_bits == 7)
        .expect("Could not find 8");

    let a = seven.segments & !one.segments;

    let mut e = 0u8;
    let mut f = 0u8;
    let mut b = 0u8;

    for ch in 0..=7 {
        let ch = 1 << ch;
        let count = entry
            .patterns
            .iter()
            .filter(|p| (p.segments & ch) != 0u8)
            .count();
        if count == 4 {
            e = ch;
        } else if count == 6 {
            b = ch;
        } else if count == 9 {
            f = ch;
        }
    }

    let c = one.segments & !f;
    let d = four.segments & !(b | c | f);
    let g = 0b111_1111 & !(a | b | c | d | e | f);

    let mut m: HashMap<u8, i32> = HashMap::new();
    m.insert(a | b | c | e | f | g, 0);
    m.insert(one.segments, 1);
    m.insert(a | c | d | e | g, 2);
    m.insert(a | c | d | f | g, 3);
    m.insert(four.segments, 4);
    m.insert(a | b | d | f | g, 5);
    m.insert(a | b | d | e | f | g, 6);
    m.insert(seven.segments, 7);
    m.insert(eight.segments, 8);
    m.insert(a | b | c | d | f | g, 9);

    let mut r = 0;
    for i in 0..4 {
        let v = m
            .get(&entry.output[i].segments)
            .expect("Could not decode output");
        r = r * 10 + v;
    }
    return r;
}

pub fn part2(input: &Vec<Entry>) -> i32 {
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
