fn count_bits(input: &Vec<String>) -> Vec<i32> {
    let len = input[0].len();
    let init: Vec<i32> = (0..len).map(|_| 0).collect();
    let counts = input.iter().fold(init, |acc, b| {
        return acc
            .iter()
            .zip(b.chars())
            .map(|(cnt, bit)| {
                return match bit {
                    '0' => cnt + 0,
                    '1' => cnt + 1,
                    _ => panic!("Invalid bit {}", bit),
                };
            })
            .collect();
    });
    counts
}

pub fn parse(input: &str) -> Vec<String> {
    return input.lines().map(str::to_string).collect();
}

pub fn part1(input: &Vec<String>) -> i32 {
    let len = input.len();
    let counts = count_bits(input);
    let (gamma, epsilon) = counts.iter().fold((0, 0), |(g, e), bit_count| {
        let bit_count_usize = *bit_count as usize;
        if bit_count_usize == len / 2 {
            panic!("Not clear what to do with {} of {}", bit_count_usize, len);
        }
        return if bit_count_usize > len / 2 {
            ((g << 1) + 1, e << 1)
        } else {
            (g << 1, (e << 1) + 1)
        };
    });

    return gamma * epsilon;
}

fn oxygen_generator_rating(input: &Vec<String>) -> i32 {
    let mut work = input.clone();
    let mut bit_index = 0;

    while work.len() > 1 {
        let bit_counts = count_bits(&work);
        let bit_count = bit_counts[bit_index] as usize;
        let half = (work.len() - 1) / 2;

        if bit_count > half {
            work.retain(|bits| bits.as_bytes()[bit_index] == '1' as u8);
        } else {
            work.retain(|bits| bits.as_bytes()[bit_index] == '0' as u8);
        }
        bit_index += 1;
    }

    if work.is_empty() {
        panic!("Could not find oxygen generator rating");
    }

    return isize::from_str_radix(&work[0], 2).unwrap() as i32;
}

fn co2_scrubber_rating(input: &Vec<String>) -> i32 {
    let mut work = input.clone();
    let mut bit_index = 0;

    while work.len() > 1 {
        let bit_counts = count_bits(&work);
        let bit_count = bit_counts[bit_index] as usize;
        let half = (work.len() + 1) / 2;

        if bit_count < half {
            work.retain(|bits| bits.as_bytes()[bit_index] == '1' as u8);
        } else {
            work.retain(|bits| bits.as_bytes()[bit_index] == '0' as u8);
        }
        bit_index += 1;
    }

    if work.is_empty() {
        panic!("Could not find oxygen generator rating");
    }

    return isize::from_str_radix(&work[0], 2).unwrap() as i32;
}

pub fn part2(input: &Vec<String>) -> i32 {
    let o2_rating = oxygen_generator_rating(input);
    let co2_rating = co2_scrubber_rating(input);

    return o2_rating * co2_rating;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        return [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .join("\n");
    }

    fn real() -> String {
        return util::read_input(3);
    }

    #[test]
    fn test_count_bits() {
        let actual = count_bits(&parse(&ex1()));
        assert_eq!(actual, vec![7, 5, 8, 7, 5]);
    }

    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 198);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 3148794);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 230);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 2795310);
    }
}
