pub fn parse(input: &str) -> Vec<i32> {
    crate::util::as_ints(input)
}

pub fn part1(input: &Vec<i32>) -> i32 {
    // improvement inspired by https://github.com/jeremylt/advent2021/blob/main/src/day01.rs
    return input.windows(2).map(|p| (p[0] < p[1]) as i32).sum();
}

pub fn part2(input: &Vec<i32>) -> i32 {
    // let avg = input.windows(3).map(|w| w[0] + w[1] + w[2]).collect();
    // return part1(&avg);
    return input
        .windows(4)
        .map(|p| (p[0] + p[1] + p[2] < p[1] + p[2] + p[3]) as i32)
        .sum();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use crate::util;

    fn ex1() -> Vec<i32> {
        return vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    }

    fn real() -> Vec<i32> {
        parse(&util::read_input(1))
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 1688);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 1728);
    }
}
