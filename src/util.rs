use std::fs;

pub fn file_as_strings(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents.lines().map(str::to_string).collect();
}

pub fn file_as_numbers(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    return contents
        .lines()
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_as_strings() {
        let actual = file_as_strings("./src/util_test.txt");
        assert_eq!(actual, vec!["test", "input"]);
    }
}
