#[derive(Debug, Eq, PartialEq)]
pub struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

impl TargetArea {
    fn hit(&self, mut v_x: i32, mut v_y: i32) -> bool {
        // There's probably a math way to do this, but simulation is fast enough
        let mut x = 0;
        let mut y = 0;
        while y >= self.y_min {
            if self.x_min <= x && x <= self.x_max && self.y_min <= y && y <= self.y_max {
                return true;
            }

            x += v_x;
            y += v_y;

            v_x = 0.max(v_x - 1);
            v_y = v_y - 1;
        }

        false
    }
}

pub fn parse(input: &str) -> TargetArea {
    let re = regex::Regex::new(
        // r"target area: x=(?P<min_x>\d+)..(?P<max_x>\d+), y(?P<min_y>-?\d+)..(?P<max_y>-?\d+)",
        r"target area: x=(?P<min_x>\d+)\.\.(?P<max_x>\d+), y=(?P<min_y>-\d+)\.\.(?P<max_y>-\d+)",
    )
    .expect("Invalid regex");
    let caps = re.captures(input).expect("Invalid input");

    TargetArea {
        x_min: caps
            .name("min_x")
            .expect("Invalid input (min_x)")
            .as_str()
            .parse()
            .expect("Invalid number (min_x)"),
        x_max: caps
            .name("max_x")
            .expect("Invalid input (max_x)")
            .as_str()
            .parse()
            .expect("Invalid number (max_x)"),
        y_min: caps
            .name("min_y")
            .expect("Invalid input (min_y)")
            .as_str()
            .parse()
            .expect("Invalid number (min_y)"),
        y_max: caps
            .name("max_y")
            .expect("Invalid input (max_y)")
            .as_str()
            .parse()
            .expect("Invalid number (max_y)"),
    }
}

pub fn part1(target: &TargetArea) -> i32 {
    // t    0   1   2   3   4   5   6   7   8   9  10  11  12
    // y    0   5   9  12  14  15  15  14  12   9   5   0  -6
    // v_y  5   4   3   2   1   0  -1  -2  -3  -4  -5  -6  -7

    // assuming y_min is negative, the v_y will land directly on -(v_y + 1)
    // we can use that to maximize v_y where it hits the very bottom of the
    // range, assuming that we can pick whatever v_x is necessary to fulfill
    // that range
    let v_y = -target.y_min - 1;

    // the height is just the summation formula
    // s = n * (a + l) / 2
    v_y * (v_y + 1) / 2
}

pub fn part2(target: &TargetArea) -> i32 {
    let mut num_velocities = 0;

    // figuring out x_min
    //   s = n * (a + l) / 2
    //   s = x_min
    //   a = x_min
    //   l = 1
    //   n^2 + n - 2 * x_min
    // Then solve with quadratic
    //   (-b +- sqrt(b^2 - 4ac)) / 2*a
    //   a = 1
    //   b = 1
    //   c = -2*x_min
    //   (-1 +- sqrt(1 +8*x_min)) / 2
    let v_x_min = (-1 + ((1 + 8 * target.x_min) as f64).sqrt().ceil() as i32) / 2;
    let v_x_max = target.x_max; // single step

    let v_y_min = target.y_min; // single step
    let v_y_max = -target.y_min - 1; // highest arch

    for v_x in v_x_min..=v_x_max {
        for v_y in v_y_min..=v_y_max {
            if target.hit(v_x, v_y) {
                // println!("{}, {}", v_x, v_y);
                num_velocities += 1;
            }
        }
    }

    num_velocities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        "target area: x=20..30, y=-10..-5".to_string()
    }

    fn real() -> String {
        util::read_input(17)
    }

    #[test]
    fn test_parse_ex1() {
        let actual = parse(&ex1());
        assert_eq!(
            actual,
            TargetArea {
                x_min: 20,
                x_max: 30,
                y_min: -10,
                y_max: -5,
            }
        )
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 45);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 4278);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 112);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 1994);
    }
}
