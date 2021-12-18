#[derive(Debug, Eq, PartialEq)]
pub struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
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

pub fn part2(_input: &TargetArea) -> i32 {
    0
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
        assert_eq!(actual, 4278); //4371 is too high
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
