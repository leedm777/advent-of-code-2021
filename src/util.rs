use std::fs;

#[allow(dead_code)]
pub fn file_as_strings(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents.lines().map(str::to_string).collect();
}

pub fn file_as_numbers(filename: &str) -> Vec<i32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    return contents.lines().map(|s| s.parse::<i32>().unwrap()).collect();
}
