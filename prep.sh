#!/bin/bash

#
# Prep for today's challenge. You'll need to grab the session cookie from the
# browser and put it into .cookies.txt for authentication.
#

# Puzzles are released at midnight Eastern time
export TZ=America/New_York

year=$(date +%Y)
day=$(date +%d)
input=./src/day${day}.txt

if test -e src/day${day}.rs; then
  echo "day${day}.rs already exists" >&2
  exit 1
fi

set -ex
curl \
  --output ${input} \
  --fail \
  --cookie .cookies.txt \
  https://adventofcode.com/${year}/day/$((10#${day}))/input

head ${input}

cat <<EOF > src/day${day}.rs
pub struct Puzzle {
}

pub fn parse(input: &str) -> Puzzle {
    input.lines().collect()
}

pub fn part1(_input: &Puzzle) -> i32 {
    0
}

pub fn part2(_input: &Puzzle) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "TODO",
        ]
        .join("\n")
    }

    fn real() -> String {
       util::read_input($((10#${day})))
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 0);
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
EOF
