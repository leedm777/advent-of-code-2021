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
    OceanFloor {
        heights,
        max_y: y,
        max_x,
    }
}

pub fn part1(floor: &OceanFloor) -> i32 {
    let get = |x: i32, y: i32| {
        if x < 0 || y < 0 || x >= floor.max_x as i32 || y >= floor.max_y as i32 {
            return &i32::MAX;
        }
        &floor.heights[y as usize][x as usize]
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
    risk
}

fn merge_basins(
    floor: &OceanFloor,
    basins: &mut Vec<Vec<usize>>,
    old_basin: usize,
    new_basin: usize,
    x: usize,
    y: usize,
) {
    if basins[y][x] == old_basin {
        basins[y][x] = new_basin;
        if x > 0 {
            merge_basins(floor, basins, old_basin, new_basin, x - 1, y);
        }
        if x < floor.max_x - 1 {
            merge_basins(floor, basins, old_basin, new_basin, x + 1, y);
        }
        if y > 0 {
            merge_basins(floor, basins, old_basin, new_basin, x, y - 1);
        }
        if y < floor.max_y - 1 {
            merge_basins(floor, basins, old_basin, new_basin, x, y + 1);
        }
    }
}
pub fn part2(floor: &OceanFloor) -> i32 {
    let mut basins = vec![vec![0; floor.max_x]; floor.max_y];
    let mut next_basin: usize = 1;

    for y in 0..floor.max_y {
        for x in 0..floor.max_x {
            let height = floor.heights[y][x];
            if height == 9 {
                basins[y][x] = 0;
                continue;
            }

            let west_basin = if x == 0 { 0 } else { basins[y][x - 1] };
            let north_basin = if y == 0 { 0 } else { basins[y - 1][x] };

            if west_basin == 0 && north_basin == 0 {
                basins[y][x] = next_basin;
                next_basin += 1;
            } else if west_basin == north_basin {
                basins[y][x] = west_basin;
            } else if west_basin == 0 {
                basins[y][x] = north_basin;
            } else if north_basin == 0 {
                basins[y][x] = west_basin;
            } else {
                // join two basins
                basins[y][x] = north_basin;
                merge_basins(floor, &mut basins, west_basin, north_basin, x - 1, y);
            }
        }
    }

    let mut basin_sizes = vec![0; next_basin];

    for y in 0..floor.max_y {
        for x in 0..floor.max_x {
            // print!("{}", basins[y][x]);
            let basin = basins[y][x];
            basin_sizes[basin] += 1;
        }
        // println!();
    }
    basin_sizes[0] = 0;
    basin_sizes.sort_unstable();
    let n = basin_sizes.len();
    // for f in &basin_sizes {
    //     println!("{}", f);
    // }
    basin_sizes[n - 1] * basin_sizes[n - 2] * basin_sizes[n - 3]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(9)
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
        assert_eq!(actual, 1198704);
    }
}
