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

pub fn neighbors(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    let mut r = vec![];
    let min_x = if x == 0 { 0 } else { x - 1 };
    let min_y = if y == 0 { 0 } else { y - 1 };
    let max_x = max_x.min(x + 2);
    let max_y = max_y.min(y + 2);

    for x_n in min_x..max_x {
        for y_n in min_y..max_y {
            if !(x_n == x && y_n == y) {
                r.push((x_n, y_n));
            }
        }
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbors_top_left() {
        let actual = neighbors(0, 0, 3, 3);
        assert_eq!(actual, vec![(0, 1), (1, 0), (1, 1)]);
    }

    #[test]
    fn test_neighbors_bottom_right() {
        let actual = neighbors(2, 2, 3, 3);
        assert_eq!(actual, vec![(1, 1), (1, 2), (2, 1)]);
    }

    #[test]
    fn test_neighbors_middle() {
        let actual = neighbors(1, 1, 3, 3);
        assert_eq!(
            actual,
            vec![
                (0, 0),
                (0, 1),
                (0, 2),
                (1, 0),
                (1, 2),
                (2, 0),
                (2, 1),
                (2, 2),
            ]
        );
    }

    #[test]
    fn test_as_ints() {
        let actual = as_ints("1,2,3,4,5\n6,7,8\n");
        assert_eq!(actual, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }
}
