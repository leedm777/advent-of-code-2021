use std::collections::HashMap;

pub struct OceanFloor {
    heights: Vec<Vec<i32>>,
    max_x: usize,
    max_y: usize,
}

pub fn parse(input: &str) -> OceanFloor {
    let mut heights = vec![];
    let mut y: usize = 0;
    let mut max_x: usize = 0;
    for line in input.lines() {
        let mut x = 0;
        let mut row = vec![0; line.len()];
        for ch in line.chars() {
            let height = ch as i32 - '0' as i32;
            row[x] = height;
            x += 1;
        }
        heights.push(row);
        max_x = x;
        y += 1;
    }
    return OceanFloor {
        heights,
        max_y: y,
        max_x,
    };
}

pub fn part1(floor: &OceanFloor) -> i32 {
    let get = |x: i32, y: i32| {
        if x < 0 || y < 0 || x >= floor.heights[0].len() as i32 || y >= floor.heights.len() as i32 {
            return &i32::MAX;
        }
        return &floor.heights[y as usize][x as usize];
    };

    let mut risk = 0;
    for x in 0..floor.max_x {
        for y in 0..floor.max_y {
            let x = x as i32;
            let y = y as i32;

            let n = get(x, y);
            let north = get(x, y - 1);
            let south = get(x, y + 1);
            let west = get(x - 1, y);
            let east = get(x + 1, y);

            if north > n && south > n && east > n && west > n {
                risk += n + 1;
            }
        }
    }
    return risk;
}

pub fn part2(input: &OceanFloor) -> i32 {
    // let mut basins = vec![vec![i32::MIN; input.max_x as usize]; input.max_y as usize];
    // let mut next_basic = 1;
    //
    // for x in 0..floor.max_x {
    //     for y in 0..floor.max_y {
    //
    //     }
    // }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        return vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .join("\n");
    }

    fn real() -> String {
        return util::read_input(9);
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 15);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 423);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 1134);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}
