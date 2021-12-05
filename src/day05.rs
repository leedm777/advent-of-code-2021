use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{},{}", self.x, self.y);
    }
}

struct Line {
    begin: Pos,
    end: Pos,
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{} -> {}", self.begin, self.end);
    }
}

fn parse_pos(s: &str) -> Pos {
    let (x, y) = s.split_once(",").expect("Could not find , in x");
    let x = x.parse().expect("Could not parse x");
    let y = y.parse().expect("Could not parse y");
    return Pos { x, y };
}
fn parse_line(line: &String) -> Line {
    let (begin, end) = line.split_once(" -> ").expect("Could not find -> in line");
    let begin = parse_pos(begin);
    let end = parse_pos(end);
    return Line { begin, end };
}

pub fn part1(input: &Vec<String>) -> i32 {
    let mut count = HashMap::new();
    let lines = input.iter().map(parse_line);
    for line in lines {
        if line.begin.x == line.end.x {
            let x = line.begin.x;
            let y1 = std::cmp::min(line.begin.y, line.end.y);
            let y2 = std::cmp::max(line.begin.y, line.end.y);

            for y in (y1)..=(y2) {
                let pos = Pos { x, y };
                // from https://stackoverflow.com/a/41418147/115478
                *count.entry(pos).or_insert(0) += 1;
            }
        } else if line.begin.y == line.end.y {
            let y = line.begin.y;
            let x1 = std::cmp::min(line.begin.x, line.end.x);
            let x2 = std::cmp::max(line.begin.x, line.end.x);

            for x in (x1)..=(x2) {
                let pos = Pos { x, y };
                // from https://stackoverflow.com/a/41418147/115478
                *count.entry(pos).or_insert(0) += 1;
            }
        }
    }

    for y in 0..10 {
        let mut s = String::new();
        for x in 0..10 {
            s = format!(
                "{}{}",
                s,
                count.get(&Pos { x, y }).map(|c| *c).unwrap_or_default()
            );
        }
        println!("{}", s);
    }

    return count.iter().map(|(_, c)| c).filter(|c| **c > 1).count() as i32;
}

pub fn part2(_input: &Vec<String>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> Vec<String> {
        return vec![
            "0,9 -> 5,9",
            "8,0 -> 0,8",
            "9,4 -> 3,4",
            "2,2 -> 2,1",
            "7,0 -> 7,4",
            "6,4 -> 2,0",
            "0,9 -> 2,9",
            "3,4 -> 1,4",
            "0,0 -> 8,8",
            "5,5 -> 8,2",
        ]
        .into_iter()
        .map(str::to_string)
        .collect();
    }

    fn real() -> Vec<String> {
        return util::file_as_strings("./src/day05.txt");
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 0);
    }
}
