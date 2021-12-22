#[derive(Debug, Eq, PartialEq)]
pub struct Cuboid {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
    z_min: i32,
    z_max: i32,

    on: bool,
}

impl Cuboid {
    fn parse(s: &str) -> Cuboid {
        // on x=10..12,y=10..12,z=10..12
        //         regex::Regex::new(r"Player [12] starting position: (?P<pos>\d+)").expect("Invalid regex");
        let re = regex::Regex::new(r"^(?P<on_off>[a-z]+) x=(?P<x_min>-?\d+)\.\.(?P<x_max>-?\d+),y=(?P<y_min>-?\d+)\.\.(?P<y_max>-?\d+),z=(?P<z_min>-?\d+)\.\.(?P<z_max>-?\d+)$")
            .expect("Invalid regex");
        let caps = re.captures(s).expect("Invalid Cuboid string");

        Cuboid {
            x_min: caps
                .name("x_min")
                .expect("Missing x_min")
                .as_str()
                .parse()
                .expect("Invalid x_min"),
            x_max: caps
                .name("x_max")
                .expect("Missing x_max")
                .as_str()
                .parse()
                .expect("Invalid x_max"),
            y_min: caps
                .name("y_min")
                .expect("Missing y_min")
                .as_str()
                .parse()
                .expect("Invalid y_min"),
            y_max: caps
                .name("y_max")
                .expect("Missing y_max")
                .as_str()
                .parse()
                .expect("Invalid y_max"),
            z_min: caps
                .name("z_min")
                .expect("Missing z_min")
                .as_str()
                .parse()
                .expect("Invalid z_min"),
            z_max: caps
                .name("z_max")
                .expect("Missing z_max")
                .as_str()
                .parse()
                .expect("Invalid z_max"),
            on: caps.name("on_off").expect("Missing on_off").as_str() == "on",
        }
    }

    fn get(&self, x: i32, y: i32, z: i32) -> Option<bool> {
        if x < self.x_min || x > self.x_max {
            return None;
        }
        if y < self.y_min || y > self.y_max {
            return None;
        }
        if z < self.z_min || z > self.z_max {
            return None;
        }
        Some(self.on)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ReactorCore {
    cuboids: Vec<Cuboid>,
}

impl ReactorCore {
    fn get(&self, x: i32, y: i32, z: i32) -> bool {
        let cuboid_value = self
            .cuboids
            .iter()
            .rev()
            .fold(None, |acc: Option<bool>, cuboid| match acc {
                None => cuboid.get(x, y, z),
                x => x,
            });

        match cuboid_value {
            None => false,
            Some(v) => v,
        }
    }
}

pub fn parse(input: &str) -> ReactorCore {
    let cuboids = input.lines().map(Cuboid::parse).collect();

    ReactorCore { cuboids }
}

pub fn part1(core: &ReactorCore) -> usize {
    let mut count: usize = 0;

    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                if core.get(x, y, z) {
                    count += 1;
                }
            }
        }
    }

    count
}

pub fn part2(_input: &ReactorCore) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "on x=-20..26,y=-36..17,z=-47..7",
            "on x=-20..33,y=-21..23,z=-26..28",
            "on x=-22..28,y=-29..23,z=-38..16",
            "on x=-46..7,y=-6..46,z=-50..-1",
            "on x=-49..1,y=-3..46,z=-24..28",
            "on x=2..47,y=-22..22,z=-23..27",
            "on x=-27..23,y=-28..26,z=-21..29",
            "on x=-39..5,y=-6..47,z=-3..44",
            "on x=-30..21,y=-8..43,z=-13..34",
            "on x=-22..26,y=-27..20,z=-29..19",
            "off x=-48..-32,y=26..41,z=-47..-37",
            "on x=-12..35,y=6..50,z=-50..-2",
            "off x=-48..-32,y=-32..-16,z=-15..-5",
            "on x=-18..26,y=-33..15,z=-7..46",
            "off x=-40..-22,y=-38..-28,z=23..41",
            "on x=-16..35,y=-41..10,z=-47..6",
            "off x=-32..-23,y=11..30,z=-14..3",
            "on x=-49..-5,y=-3..45,z=-29..18",
            "off x=18..30,y=-20..-8,z=-3..13",
            "on x=-41..9,y=-7..43,z=-33..15",
            "on x=-54112..-39298,y=-85059..-49293,z=-27449..7877",
            "on x=967..23432,y=45373..81175,z=27513..53682",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(22)
    }

    //
    // off x=9..11,y=9..11,z=9..11
    #[test]
    fn test_parse1() {
        let actual = Cuboid::parse("on x=11..13,y=11..13,z=11..13");
        assert_eq!(
            actual,
            Cuboid {
                x_min: 11,
                x_max: 13,
                y_min: 11,
                y_max: 13,
                z_min: 11,
                z_max: 13,
                on: true,
            }
        );
    }

    #[test]
    fn test_parse2() {
        let actual = Cuboid::parse("off x=-11..-13,y=-11..-13,z=-11..-13");
        assert_eq!(
            actual,
            Cuboid {
                x_min: -11,
                x_max: -13,
                y_min: -11,
                y_max: -13,
                z_min: -11,
                z_max: -13,
                on: false,
            }
        );
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 590784);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 582644);
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
