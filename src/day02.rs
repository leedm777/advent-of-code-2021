pub fn parse(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

pub fn part1(input: &Vec<String>) -> i32 {
    let (x, y) = input
        .iter()
        .map(|s| {
            let mut split = s.split_whitespace();
            let dir = split.next().unwrap();
            let dist = split.next().unwrap().parse::<i32>().unwrap();
            return match dir {
                "forward" => (dist, 0),
                "down" => (0, dist),
                "up" => (0, -dist),
                _ => panic!("Invalid direction {}", dir),
            };
        })
        .fold((0, 0), |(dx, dy), (x, y)| (x + dx, y + dy));
    x * y
}

pub fn part2(input: &Vec<String>) -> i32 {
    let (x, y, _) = input
        .iter()
        .map(|s| {
            let mut split = s.split_whitespace();
            let dir = split.next().unwrap();
            let dist = split.next().unwrap().parse::<i32>().unwrap();
            (dir, dist)
        })
        .fold((0, 0, 0), |(x, y, aim), (dir, dist)| match dir {
            "forward" => (x + dist, y + dist * aim, aim),
            "down" => (x, y, aim + dist),
            "up" => (x, y, aim - dist),
            _ => panic!("Invalid direction {}", dir),
        });
    x * y
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> Vec<String> {
        return vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
    }

    fn real() -> Vec<String> {
        parse(&util::read_input(2))
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 150);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 1882980);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 900);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 1971232560);
    }
}
