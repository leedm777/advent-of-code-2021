use std::collections::{HashMap, HashSet};

// TODO: I'm in the middle of trying to repace strings with i32

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Cave {
    id: i32,
    id_str: String,
    neighbors: HashSet<i32>,
}

impl Cave {
    fn create(id: i32, id_str: &str) -> Cave {
        Cave {
            id,
            id_str: id_str.to_string(),
            neighbors: HashSet::new(),
        }
    }

    fn is_small_cave(id: &str) -> bool {
        return id.chars().all(|ch| ch.is_lowercase());
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Caves {
    map: HashMap<i32, Cave>,
    ids: HashMap<String, i32>,
    next_id: i32,
}

const START: i32 = -1;
const END: i32 = -2;

impl Caves {
    fn init() -> Caves {
        let mut ids = HashMap::new();

        ids.insert("start".to_string(), START);
        ids.insert("end".to_string(), END);

        let mut map = HashMap::new();
        map.insert(START, Cave::create(START, "start"));
        map.insert(END, Cave::create(END, "end"));

        Caves {
            map,
            ids,
            next_id: 3,
        }
    }

    fn upsert(&mut self, id_str: &str) -> &Cave {
        match self.ids.get(id_str) {
            Some(id) => self.map.get_mut(id).expect("Should have had that cave"),
            None => {
                let id = if Cave::is_small_cave(id_str) {
                    -self.next_id
                } else {
                    self.next_id
                };
                self.next_id += 1;
                self.ids.insert(id_str.to_string(), id);
                let cave = Cave::create(id, id_str);
                self.map.insert(id, cave);
                self.map.get_mut(&id).expect("Should have had that cave")
            }
        }
    }
}

pub fn parse(input: &str) -> Caves {
    let mut caves = Caves::init();

    input.lines().for_each(|line| {
        let (lhs_str, rhs_str) = line.split_once("-").expect("Could not find -");
        let lhs_id = caves.upsert(lhs_str).id;
        let rhs_id = caves.upsert(rhs_str).id;
        {
            let lhs = caves
                .map
                .get_mut(&lhs_id)
                .expect("But I just put it there?");
            lhs.neighbors.insert(rhs_id);
        }
        {
            let rhs = caves
                .map
                .get_mut(&rhs_id)
                .expect("But I just put it there?");
            rhs.neighbors.insert(lhs_id);
        }

        // {
        //     let rhs = caves.upsert(rhs_id);
        //     lhs.neighbors.insert(rhs.id);
        // }
        //
        // {
        //     let lhs = caves.upsert(lhs_id);
        //     let rhs = caves.upsert_mut(rhs_id);
        //     rhs.neighbors.insert(lhs.id);
        // }
    });

    caves
}

struct Path {
    steps: Vec<i32>,
    double_visit: bool,
}

impl Path {
    fn init() -> Path {
        Path {
            steps: vec![START],
            double_visit: false,
        }
    }

    fn next_path(&self, id: i32) -> Path {
        let mut steps = self.steps.clone();
        steps.push(id);
        Path {
            steps,
            double_visit: self.double_visit,
        }
    }

    fn last(&self) -> i32 {
        *self.steps.last().expect("Found an empty todo path")
    }

    fn contains(&self, id: i32) -> bool {
        self.steps.contains(&id)
    }
}

pub fn part1(caves: &Caves) -> usize {
    let mut num_paths: usize = 0;
    let mut todo = vec![Path::init()];

    while let Some(path) = todo.pop() {
        let node = path.last();
        let node = caves.map.get(&node).expect("Found an unexpected cave id");
        for &neighbor in &node.neighbors {
            if neighbor == END {
                // done
                num_paths += 1;
            } else if neighbor < 0 && path.contains(neighbor) {
                // small cave already visited; skip
                continue;
            } else {
                // found another path to search
                todo.push(path.next_path(neighbor));
            }
        }
    }

    num_paths
}

pub fn part2(caves: &Caves) -> usize {
    let mut num_paths: usize = 0;
    let mut todo = vec![Path::init()];

    while let Some(path) = todo.pop() {
        let node = path.last();
        let node = caves.map.get(&node).expect("Found an unexpected cave id");
        for &neighbor in &node.neighbors {
            if neighbor == END {
                // done
                num_paths += 1;
            } else if neighbor == START {
                // cannot revisit the start
                continue;
            } else if neighbor < 0 && path.contains(neighbor) {
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

    num_paths
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    // fn test_cave(id: &str, neighbors: &[&str]) -> Cave {
    //     Cave {
    //         id: id.to_string(),
    //         neighbors: neighbors.iter().map(|s| s.to_string()).collect(),
    //     }
    // }

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

    // #[test]
    // fn test_parse_ex1() {
    //     let actual = parse(&ex1());
    //     let expected = Caves {
    //         map: [
    //             ("start".to_string(), test_cave("start", &["A", "b"])),
    //             ("A".to_string(), test_cave("A", &["start", "c", "b", "end"])),
    //             ("b".to_string(), test_cave("b", &["start", "A", "d", "end"])),
    //             ("c".to_string(), test_cave("c", &["A"])),
    //             ("d".to_string(), test_cave("d", &["b"])),
    //             ("end".to_string(), test_cave("end", &["A", "b"])),
    //         ]
    //         .iter()
    //         .cloned()
    //         .collect(),
    //     };
    //
    //     assert_eq!(
    //         actual.map.keys().collect::<HashSet<&String>>(),
    //         expected.map.keys().collect::<HashSet<&String>>(),
    //         "Keys do not match"
    //     );
    //     actual.map.keys().for_each(|id| {
    //         assert_eq!(
    //             actual.map.get(id),
    //             expected.map.get(id),
    //             "Failed for {}",
    //             id
    //         );
    //     })
    // }
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
