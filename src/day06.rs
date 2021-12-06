use std::collections::HashMap;

fn next(fish: &mut Vec<i32>) {
    for idx in 0..fish.len() {
        if fish[idx] == 0 {
            fish[idx] = 6;
            fish.push(8);
        } else {
            fish[idx] -= 1;
        }
    }
}

fn grow(input: &Vec<i32>, days: i32) -> usize {
    let mut growth = HashMap::<i32, usize>::new();

    let mut fish_n = vec![8];
    for d in 0..days {
        // println!("day {}", d);
        next(&mut fish_n);
    }
    growth.insert(8, fish_n.len());

    for n in (0..=7).rev() {
        // println!("fish {}", n);
        next(&mut fish_n);
        growth.insert(n, fish_n.len());
    }

    return input
        .iter()
        .map(|n| growth.get(n).expect(&format!("No growth for {}", n)))
        .sum();
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
    fn test_next_1() {
        let mut fish = vec![3, 4, 3, 1, 2];
        next(&mut fish);
        assert_eq!(fish, vec![2, 3, 2, 0, 1]);
    }

    #[test]
    fn test_next_2() {
        let mut fish = vec![2, 3, 2, 0, 1];
        next(&mut fish);
        assert_eq!(fish, vec![1, 2, 1, 6, 0, 8]);
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
