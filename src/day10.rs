pub fn parse(input: &str) -> Vec<&str> {
    input.lines().collect()
}

pub fn part1(lines: &Vec<&str>) -> i64 {
    let mut score = 0;
    for line in lines {
        let mut expected = vec![];
        for ch in line.chars() {
            match ch {
                '(' => expected.push(')'),
                '[' => expected.push(']'),
                '{' => expected.push('}'),
                '<' => expected.push('>'),
                _ => {
                    if ch != expected.pop().expect("Too many closing parens") {
                        match ch {
                            ')' => score += 3,
                            ']' => score += 57,
                            '}' => score += 1197,
                            '>' => score += 25137,
                            _ => panic!("Unexpected ch"),
                        }
                    }
                }
            }
        }
    }
    score
}

pub fn part2(lines: &Vec<&str>) -> i64 {
    let mut scores: Vec<i64> = lines
        .iter()
        .map(|line: &&str| {
            let mut expected = vec![];
            for ch in line.chars() {
                match ch {
                    '(' => expected.push(')'),
                    '[' => expected.push(']'),
                    '{' => expected.push('}'),
                    '<' => expected.push('>'),
                    _ => {
                        // ignore incomplete lines
                        if expected.is_empty() {
                            break;
                        }

                        if ch != expected.pop().expect("PANIC!!!!") {
                            return 0;
                        }
                    }
                }
            }
            expected.reverse();
            let score = expected
                .iter()
                .map(|ch| match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("PANIC!!!!"),
                })
                .fold(0 as i64, |acc, s| 5 * acc + s);

            // println!("{} => {}", expected.iter().collect::<String>(), score);
            score
        })
        .filter(|s| *s > 0)
        .collect();
    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "[({(<(())[]>[[{[]{<()<>>",
            "[(()[<>])]({[<{<<[]>>(",
            "{([(<{}[<>[]}>{[]{[(<()>",
            "(((({<>}<{<{<>}{[]{[]{}",
            "[[<[([]))<([[{}[[()]]]",
            "[{[{({}]{}}([{[{{{}}([]",
            "{<[[]]>}<{[{[{[]{()[[[]",
            "[<(<(<(<{}))><([]([]()",
            "<{([([[(<>()){}]>(<<{{",
            "<{([{{}}[<[[[<>{}]]]>[]]",
        ]
        .join("\n")
    }

    fn real() -> String {
        return util::read_input(10);
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 26397);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 366027);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 288957);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 1118645287);
    }
}
