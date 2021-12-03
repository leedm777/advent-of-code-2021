use crate::util;

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
        .fold((0, 0), |(dx, dy), (x, y)| {
            return (x + dx, y + dy);
        });
    return x * y;
}

pub fn part2(input: &Vec<String>) -> i32 {
    let (x, y, _) = input
        .iter()
        .map(|s| {
            let mut split = s.split_whitespace();
            let dir = split.next().unwrap();
            let dist = split.next().unwrap().parse::<i32>().unwrap();
            return (dir, dist);
        })
        .fold((0, 0, 0), |(x, y, aim), (dir, dist)| {
            return match dir {
                "forward" => (x + dist, y + dist * aim, aim),
                "down" => (x, y, aim + dist),
                "up" => (x, y, aim - dist),
                _ => panic!("Invalid direction {}", dir),
            };
        });
    return x * y;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ex1() -> Vec<String> {
        return (vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ])
        .into_iter()
        .map(str::to_string)
        .collect();
    }

    fn real() -> Vec<String> {
        return util::file_as_strings("./src/day02.txt");
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
