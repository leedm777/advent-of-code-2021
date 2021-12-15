use crate::util::neighbors;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
pub struct Cave {
    risk: Vec<Vec<usize>>,
}

// inspired from https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str) -> Cave {
    let mut risk = vec![];
    for line in input.lines() {
        let mut risk_row = vec![];
        for ch in line.chars() {
            risk_row.push(ch as usize - '0' as usize);
        }
        risk.push(risk_row);
    }

    Cave { risk }
}

pub fn part1(input: &Cave) -> usize {
    let start = (0, 0);
    let goal = (input.risk[0].len() - 1, input.risk.len() - 1);

    // dist[node] = current shortest distance from start to node
    let mut dist: Vec<_> = input
        .risk
        .iter()
        .map(|row| row.iter().map(|_| usize::MAX).collect::<Vec<_>>())
        .collect();

    let mut heap = BinaryHeap::new();

    dist[start.1][start.0] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // found it!
        if position == goal {
            return cost;
        }

        if cost > dist[position.1][position.0] {
            continue;
        }

        let mut n = vec![];
        if position.0 > 0 {
            n.push((position.0 - 1, position.1));
        }
        if position.1 > 0 {
            n.push((position.0, position.1 - 1));
        }
        if position.0 < goal.0 {
            n.push((position.0 + 1, position.1));
        }
        if position.1 < goal.1 {
            n.push((position.0, position.1 + 1));
        }

        n.iter().for_each(|&(x, y)| {
            let next = State {
                cost: cost + input.risk[y][x],
                position: (x, y),
            };
            if next.cost < dist[next.position.1][next.position.0] {
                heap.push(next);
                dist[next.position.1][next.position.0] = next.cost;
            }
        })
    }

    usize::MAX
}

pub fn part2(_input: &Cave) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec![
            "1163751742",
            "1381373672",
            "2136511328",
            "3694931569",
            "7463417111",
            "1319128137",
            "1359912421",
            "3125421639",
            "1293138521",
            "2311944581",
        ]
        .join("\n")
    }

    fn real() -> String {
        util::read_input(15)
    }

    #[test]
    fn test_parse_ex1() {
        let actual = parse(&ex1());
        assert_eq!(
            actual,
            Cave {
                risk: vec![
                    vec![1, 1, 6, 3, 7, 5, 1, 7, 4, 2],
                    vec![1, 3, 8, 1, 3, 7, 3, 6, 7, 2],
                    vec![2, 1, 3, 6, 5, 1, 1, 3, 2, 8],
                    vec![3, 6, 9, 4, 9, 3, 1, 5, 6, 9],
                    vec![7, 4, 6, 3, 4, 1, 7, 1, 1, 1],
                    vec![1, 3, 1, 9, 1, 2, 8, 1, 3, 7],
                    vec![1, 3, 5, 9, 9, 1, 2, 4, 2, 1],
                    vec![3, 1, 2, 5, 4, 2, 1, 6, 3, 9],
                    vec![1, 2, 9, 3, 1, 3, 8, 5, 2, 1],
                    vec![2, 3, 1, 1, 9, 4, 4, 5, 8, 1],
                ]
            }
        )
    }
    #[test]
    fn test_part1_ex1() {
        let actual = part1(&parse(&ex1()));
        assert_eq!(actual, 40);
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 527);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 315);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}
