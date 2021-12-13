use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cave {
    id: String,
    neighbors: HashSet<String>,
}

impl Cave {
    fn create(id: &str) -> Cave {
        Cave {
            id: id.to_string(),
            neighbors: HashSet::new(),
        }
    }

    fn is_small_cave(id: &str) -> bool {
        return id.chars().all(|ch| ch.is_lowercase());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Caves {
    map: HashMap<String, Cave>,
}

impl Caves {
    fn init() -> Caves {
        Caves {
            map: HashMap::new(),
        }
    }
    fn upsert(&mut self, id: &str) -> &mut Cave {
        self.map.entry(id.to_string()).or_insert(Cave::create(id))
    }
}

pub fn parse(input: &str) -> Caves {
    let mut caves = Caves::init();

    input.lines().for_each(|line| {
        let (lhs_id, rhs_id) = line.split_once("-").expect("Could not find -");
        {
            // block to limit the mutable borrow lifetime
            let lhs = caves.upsert(lhs_id);
            lhs.neighbors.insert(rhs_id.to_string());
        }
        {
            let rhs = caves.upsert(rhs_id);
            rhs.neighbors.insert(lhs_id.to_string());
        }
    });

    caves
}

struct Path {
    steps: Vec<String>,
    double_visit: bool,
}

impl Path {
    fn init() -> Path {
        Path {
            steps: vec!["start".to_string()],
            double_visit: false,
        }
    }

    fn next_path(&self, id: &str) -> Path {
        let mut steps = self.steps.clone();
        steps.push(id.to_string());
        Path {
            steps,
            double_visit: self.double_visit,
        }
    }

    fn last(&self) -> &String {
        self.steps.last().expect("Found an empty todo path")
    }

    fn contains(&self, id: &String) -> bool {
        self.steps.contains(id)
    }
}

pub fn part1(caves: &Caves) -> usize {
    let mut paths = vec![];
    let mut todo = vec![Path::init()];

    while let Some(path) = todo.pop() {
        let node = path.last();
        let node = caves.map.get(node).expect("Found an unexpected cave id");
        for neighbor in &node.neighbors {
            if neighbor == "end" {
                // done
                paths.push(path.next_path("end"));
            } else if Cave::is_small_cave(&neighbor) && path.contains(&neighbor) {
                // small cave already visited; skip
                continue;
            } else {
                // found another path to search
                todo.push(path.next_path(neighbor));
            }
        }
    }

    paths.len()
}

pub fn part2(caves: &Caves) -> usize {
    let mut paths = vec![];
    let mut todo = vec![Path::init()];

    while let Some(path) = todo.pop() {
        let node = path.last();
        let node = caves.map.get(node).expect("Found an unexpected cave id");
        for neighbor in &node.neighbors {
            if neighbor == "end" {
                // done
                paths.push(path.next_path("end"));
            } else if neighbor == "start" {
                // cannot revisit the start
                continue;
            } else if Cave::is_small_cave(&neighbor) && path.contains(&neighbor) {
                if path.double_visit {
                    // already paid a double visit; skip
                    continue;
                }
                let mut next_path = path.next_path(neighbor);
                next_path.double_visit = true;
                todo.push(next_path);
            } else {
                // found another path to search
                todo.push(path.next_path(neighbor));
            }
        }
    }

    paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn test_cave(id: &str, neighbors: &[&str]) -> Cave {
        Cave {
            id: id.to_string(),
            neighbors: neighbors.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn ex1() -> String {
        vec!["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"].join("\n")
    }
    fn ex2() -> String {
        vec![
            "dc-end", "HN-start", "start-kj", "dc-start", "dc-HN", "LN-dc", "HN-end", "kj-sa",
            "kj-HN", "kj-dc",
        ]
        .join("\n")
    }
    fn ex3() -> String {
        vec![
            "fs-end", "he-DX", "fs-he", "start-DX", "pj-DX", "end-zg", "zg-sl", "zg-pj", "pj-he",
            "RW-he", "fs-DX", "pj-RW", "zg-RW", "start-pj", "he-WI", "zg-he", "pj-fs", "start-RW",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(12)
    }

    #[test]
    fn test_parse_ex1() {
        let actual = parse(&ex1());
        let expected = Caves {
            map: [
                ("start".to_string(), test_cave("start", &["A", "b"])),
                ("A".to_string(), test_cave("A", &["start", "c", "b", "end"])),
                ("b".to_string(), test_cave("b", &["start", "A", "d", "end"])),
                ("c".to_string(), test_cave("c", &["A"])),
                ("d".to_string(), test_cave("d", &["b"])),
                ("end".to_string(), test_cave("end", &["A", "b"])),
            ]
            .iter()
            .cloned()
            .collect(),
        };

        assert_eq!(
            actual.map.keys().collect::<HashSet<&String>>(),
            expected.map.keys().collect::<HashSet<&String>>(),
            "Keys do not match"
        );
        actual.map.keys().for_each(|id| {
            assert_eq!(
                actual.map.get(id),
                expected.map.get(id),
                "Failed for {}",
                id
            );
        })
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 10);
    }
    #[test]
    fn test_part1_ex2() {
        let actual = part1(&parse(&ex2()));
        assert_eq!(actual, 19);
    }
    #[test]
    fn test_part1_ex3() {
        let actual = part1(&parse(&ex3()));
        assert_eq!(actual, 226);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 4970);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 36);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 137948);
    }
}
