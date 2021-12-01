use std::collections::HashMap;

fn part1(input: Vec<&str>) -> i32 {
    let mut has2 = 0;
    let mut has3 = 0;

    for s in input {
        let mut counter: HashMap<char, i32> = HashMap::new();
        for c in s.chars() {
            let v = match counter.get(&c) {
                Some(x) => x + 1,
                None => 1
            };
            counter.insert(c, v);
        }
        let mut is2 = false;
        let mut is3 = false;
        for c in counter.into_values() {
            if c == 2 {
                is2 = true;
            }
            if c == 3 {
                is3 = true;
            }
        }

        if is2 {
            has2 = has2 + 1;
        }

        if is3 {
            has3 = has3 + 1;
        }
    }
    return has2 * has3;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1_ex1() {
        let input = vec![
            "abcdef",
            "bababc",
            "abbcde",
            "abcccd",
            "aabcdd",
            "abcdee",
            "ababab"
        ];

        let actual = part1(input);
        assert_eq!(actual, 12);
    }
}