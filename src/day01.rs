use crate::util;

fn part1(input: Vec<i32>) -> i32 {
    let mut increases = 0;
    let mut prior = i32::MAX;
    for v in input {
        if v > prior {
            increases += 1;
        }
        prior = v;
    }
    return increases;
}

fn part2(input: Vec<i32>) -> i32 {
    let avg = input.windows(3).map(|w| w.iter().sum()).collect();
    return part1(avg);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn ex1() -> Vec<i32> {
        return vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
    }

    fn real() -> Vec<i32> {
        return util::file_as_numbers("./src/day01.txt");
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(ex1());
        assert_eq!(actual, 7);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(real());
        assert_eq!(actual, 1688);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(ex1());
        assert_eq!(actual, 5);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(real());
        assert_eq!(actual, 1728);
    }
}
