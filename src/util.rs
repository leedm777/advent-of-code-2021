use std::fs;

pub fn read_input(day: i32) -> String {
    let filename = format!("./src/day{:02}.txt", day);
    return fs::read_to_string(filename).expect("Something went wrong reading the file");
}

pub fn as_ints(input: &str) -> Vec<i32> {
    return input
        .trim()
        .split(|c| c == ',' || char::is_whitespace(c))
        .map(|s| s.parse().expect("Could not parse number"))
        .collect();
}

pub fn file_as_strings(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents.lines().map(str::to_string).collect();
}

pub fn file_as_numbers(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents
        .lines()
        .flat_map(|s| s.split(","))
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_ints() {
        let actual = as_ints("1,2,3,4,5\n6,7,8\n");
        assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
    #[test]
    fn test_file_as_strings() {
        let actual = file_as_strings("./src/util_test.txt");
        assert_eq!(actual, vec!["test", "input"]);
    }
}
