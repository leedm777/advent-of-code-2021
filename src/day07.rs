pub struct CrabDepths(i32, i32, Vec<i32>);

pub fn parse(input: &str) -> CrabDepths {
    let input: Vec<i32> = crate::util::as_ints(input);

    let min_depth = *input.iter().min().expect("Could not find min");
    let max_depth = *input.iter().max().expect("Could not find max");

    return CrabDepths(min_depth, max_depth, input);
}

pub fn part1(CrabDepths(min_depth, max_depth, crab_depths): &CrabDepths) -> i32 {
    let mut min_cost = i32::MAX;

    for depth in *min_depth..=*max_depth {
        let cost = crab_depths.iter().map(|d| (d - depth).abs()).sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}

pub fn part2(CrabDepths(min_depth, max_depth, crab_depths): &CrabDepths) -> i32 {
    let mut min_cost = i32::MAX;

    for depth in *min_depth..=*max_depth {
        let cost = crab_depths
            .iter()
            .map(|d| (d - depth).abs())
            .map(|d| d * (d + 1) / 2)
            .sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }
    return min_cost;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> CrabDepths {
        return parse("16,1,2,0,4,2,7,1,2,14");
    }

    fn real() -> CrabDepths {
        return parse(&util::read_input(7));
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
        assert_eq!(actual, 168);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&real());
        assert_eq!(actual, 98363777);
    }
}
