#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl std::fmt::Display for Pos {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return write!(f, "{},{}", self.x, self.y);
    }
}

pub struct Line {
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
fn parse_line(line: &str) -> Line {
    let (begin, end) = line.split_once(" -> ").expect("Could not find -> in line");
    let begin = parse_pos(begin);
    let end = parse_pos(end);
    return Line { begin, end };
}

pub fn parse(input: &str) -> Vec<Line> {
    return input.lines().map(parse_line).collect();
}

fn score_count(count: &Vec<Vec<i32>>) -> i32 {
    count.iter().flat_map(|v| v).fold(
        0,
        |sum, &num_overlaps| {
            if num_overlaps > 1 {
                sum + 1
            } else {
                sum
            }
        },
    )
}

pub fn part1(input: &Vec<Line>) -> i32 {
    let mut count = vec![vec![0; 1000]; 1000];
    for line in input {
        if line.begin.x == line.end.x || line.begin.y == line.end.y {
            let x_dir = (line.end.x - line.begin.x).signum();
            let y_dir = (line.end.y - line.begin.y).signum();

            let mut pos = line.begin;
            while pos != line.end {
                count[pos.y as usize][pos.x as usize] += 1;
                pos.x += x_dir;
                pos.y += y_dir;
            }
            count[pos.y as usize][pos.x as usize] += 1;
        }
    }

    score_count(&count)
}

pub fn part2(input: &Vec<Line>) -> i32 {
    let mut count = vec![vec![0; 1000]; 1000];
    for line in input {
        let x_dir = (line.end.x - line.begin.x).signum();
        let y_dir = (line.end.y - line.begin.y).signum();

        let mut pos = line.begin;
        while pos != line.end {
            count[pos.y as usize][pos.x as usize] += 1;
            pos.x += x_dir;
            pos.y += y_dir;
        }
        count[pos.y as usize][pos.x as usize] += 1;
    }

    // for y in 0..1000 {
    //     for x in 0..1000 {
    //         let v = count[y][x];
    //         if v == 0 {
    //             print!(" ");
    //         } else {
    //             print!("{}", v % 10);
    //         }
    //     }
    //     println!();
    // }

    score_count(&count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> Vec<Line> {
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
        .iter()
        .map(|&s| parse_line(s))
        .collect();
    }

    fn real() -> Vec<Line> {
        return util::read_input(5).lines().map(parse_line).collect();
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 6841);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 12);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 19258); // 19236 too low
    }
}
