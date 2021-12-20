pub struct MapImage {
    image_enhancement: Vec<bool>,
    image: std::collections::HashSet<(i32, i32)>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl MapImage {
    fn new(image_enhancement: Vec<bool>) -> MapImage {
        MapImage {
            image_enhancement: image_enhancement,
            image: std::collections::HashSet::new(),
            min_x: i32::MAX,
            max_x: i32::MIN,
            min_y: i32::MAX,
            max_y: i32::MIN,
        }
    }

    fn set_pixel(&mut self, x: i32, y: i32) {
        self.image.insert((x, y));

        if x < self.min_x {
            self.min_x = x;
        } else if x > self.max_x {
            self.max_x = x;
        }

        if y < self.min_y {
            self.min_y = y;
        } else if y > self.max_y {
            self.max_y = y;
        }
    }

    fn get_pixel(&self, x: i32, y: i32) -> bool {
        self.image.contains(&(x, y))
    }

    fn enhance(&self) -> MapImage {
        let mut enhanced = MapImage::new(self.image_enhancement.clone());

        for y in (self.min_y - 1)..=(self.max_y + 1) {
            for x in (self.min_x - 1)..=(self.max_x + 1) {
                let mut v = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        v <<= 1;
                        if self.get_pixel(x + dx, y + dy) {
                            v |= 1;
                        }
                    }
                }

                // println!("({},{}) -> {} == {}", x, y, v, self.image_enhancement[v]);
                if self.image_enhancement[v] {
                    enhanced.set_pixel(x, y);
                }
            }
        }

        enhanced
    }
}

impl ToString for MapImage {
    fn to_string(&self) -> String {
        let mut s = format!(
            "({}, {}) -> ({}, {})\n",
            self.min_x, self.min_y, self.max_y, self.max_y
        );
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                if self.get_pixel(x, y) {
                    s += "#";
                } else {
                    s += ".";
                }
            }
            s += "\n";
        }

        s
    }
}

pub fn parse_pixel(ch: char) -> bool {
    if ch == '#' {
        true
    } else if ch == '.' {
        false
    } else {
        panic!("Invalid map character {}", ch);
    }
}

pub fn parse(input: &str) -> MapImage {
    let mut lines = input.lines();
    let image_enhancement = lines
        .next()
        .expect("No image enhancement line")
        .chars()
        .map(parse_pixel)
        .collect();
    let blank = lines.next().expect("No blank");
    if !blank.is_empty() {
        panic!("Expected blank line");
    }
    let mut image = MapImage::new(image_enhancement);

    for (y, line) in lines.enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if parse_pixel(ch) {
                image.set_pixel(x as i32, y as i32);
            }
        }
    }

    image
}

pub fn part1(image: &MapImage) -> usize {
    println!("{}", image.to_string());
    let round1 = image.enhance();
    println!("{}", round1.to_string());
    let round2 = round1.enhance();
    println!("{}", round2.to_string());

    round2.image.len()
}

pub fn part2(_input: &MapImage) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#",
            "",
            "#..#.",
            "#....",
            "##..#",
            "..#..",
            "..###",            
        ].join("\n")
    }

    fn real() -> String {
        util::read_input(20)
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 35);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 0); // 5127 is too high
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}
