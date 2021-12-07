use std::collections::HashMap;

pub fn parse(input: &str) -> Vec<i32> {
    return crate::util::as_ints(input);
}

fn next(fish_counts: &HashMap<i32, usize>) -> HashMap<i32, usize> {
    let mut r = HashMap::<i32, usize>::new();

    for (&n, &c) in fish_counts.iter() {
        if n == 0 {
            r.insert(8, c);
            *r.entry(6).or_insert(0) += c;
        } else if n == 7 {
            *r.entry(6).or_insert(0) += c;
        } else {
            r.insert(n - 1, c);
        }
    }

    return r;
}

fn grow(input: &Vec<i32>, days: i32) -> usize {
    let mut fish_counts = HashMap::<i32, usize>::new();
    for fish in input {
        *fish_counts.entry(*fish).or_insert(0) += 1;
    }

    for _ in 0..days {
        fish_counts = next(&fish_counts);
    }

    return fish_counts.values().sum();
}

pub fn part1(input: &Vec<i32>) -> usize {
    return grow(input, 80);
}

pub fn part2(input: &Vec<i32>) -> usize {
    return grow(input, 256);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> Vec<i32> {
        return vec![3, 4, 3, 1, 2];
    }

    fn real() -> Vec<i32> {
        return util::file_as_numbers("./src/day06.txt");
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 5934);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 356190);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&ex1());
        assert_eq!(actual, 26984457539);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 1617359101538);
    }
}
