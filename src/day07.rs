pub fn part1(input: &Vec<i32>) -> i32 {
    let min_depth = *input.iter().min().expect("Could not find min");
    let max_depth = *input.iter().max().expect("Could not find max");

    let mut min_cost = i32::MAX;

    for depth in min_depth..=max_depth {
        let cost = input.iter().map(|d| (d - depth).abs()).sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}

pub fn part2(_input: &Vec<i32>) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> Vec<i32> {
        return vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    }

    fn real() -> Vec<i32> {
        return util::file_as_numbers("./src/day07.txt");
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&ex1());
        assert_eq!(actual, 37);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&real());
        assert_eq!(actual, 347011);
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
