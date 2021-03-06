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

fn solve_maze(cave: &Cave) -> usize {
    let start = (0, 0);
    let goal = (cave.risk[0].len() - 1, cave.risk.len() - 1);

    // dist[node] = current shortest distance from start to node
    let mut dist: Vec<_> = cave
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
                cost: cost + cave.risk[y][x],
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

pub fn part1(cave: &Cave) -> usize {
    solve_maze(cave)
}

fn expand(cave: &Cave) -> Cave {
    let mut risk = vec![];

    for tile_y in 0..5 {
        for row in cave.risk.iter() {
            let mut tiled_row = vec![];
            for tile_x in 0..5 {
                let added_risk = tile_y + tile_x;

                for cell_risk in row.iter() {
                    let new_risk = (cell_risk + added_risk - 1) % 9 + 1;

                    tiled_row.push(new_risk);
                }
            }
            risk.push(tiled_row);
        }
    }

    Cave { risk }
}

pub fn part2(cave: &Cave) -> usize {
    let cave = expand(cave);
    solve_maze(&cave)
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
    fn test_expand() {
        let actual = expand(&parse(&ex1()));
        assert_eq!(
            actual,
            Cave {
                risk: vec![
                    vec![
                        1, 1, 6, 3, 7, 5, 1, 7, 4, 2, 2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9,
                        7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6
                    ],
                    vec![
                        1, 3, 8, 1, 3, 7, 3, 6, 7, 2, 2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5,
                        9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6
                    ],
                    vec![
                        2, 1, 3, 6, 5, 1, 1, 3, 2, 8, 3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7,
                        3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3
                    ],
                    vec![
                        3, 6, 9, 4, 9, 3, 1, 5, 6, 9, 4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2,
                        5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4
                    ],
                    vec![
                        7, 4, 6, 3, 4, 1, 7, 1, 1, 1, 8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6,
                        3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5
                    ],
                    vec![
                        1, 3, 1, 9, 1, 2, 8, 1, 3, 7, 2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3,
                        4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2
                    ],
                    vec![
                        1, 3, 5, 9, 9, 1, 2, 4, 2, 1, 2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2,
                        3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5
                    ],
                    vec![
                        3, 1, 2, 5, 4, 2, 1, 6, 3, 9, 4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6,
                        4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4
                    ],
                    vec![
                        1, 2, 9, 3, 1, 3, 8, 5, 2, 1, 2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3,
                        5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5
                    ],
                    vec![
                        2, 3, 1, 1, 9, 4, 4, 5, 8, 1, 3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2,
                        6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5
                    ],
                    vec![
                        2, 2, 7, 4, 8, 6, 2, 8, 5, 3, 3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1,
                        8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7
                    ],
                    vec![
                        2, 4, 9, 2, 4, 8, 4, 7, 8, 3, 3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6,
                        1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7
                    ],
                    vec![
                        3, 2, 4, 7, 6, 2, 2, 4, 3, 9, 4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8,
                        4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4
                    ],
                    vec![
                        4, 7, 1, 5, 1, 4, 2, 6, 7, 1, 5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3,
                        6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5
                    ],
                    vec![
                        8, 5, 7, 4, 5, 2, 8, 2, 2, 2, 9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7,
                        4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6
                    ],
                    vec![
                        2, 4, 2, 1, 2, 3, 9, 2, 4, 8, 3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4,
                        5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3
                    ],
                    vec![
                        2, 4, 6, 1, 1, 2, 3, 5, 3, 2, 3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3,
                        4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6
                    ],
                    vec![
                        4, 2, 3, 6, 5, 3, 2, 7, 4, 1, 5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7,
                        5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5
                    ],
                    vec![
                        2, 3, 1, 4, 2, 4, 9, 6, 3, 2, 3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4,
                        6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6
                    ],
                    vec![
                        3, 4, 2, 2, 1, 5, 5, 6, 9, 2, 4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3,
                        7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6
                    ],
                    vec![
                        3, 3, 8, 5, 9, 7, 3, 9, 6, 4, 4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2,
                        9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8
                    ],
                    vec![
                        3, 5, 1, 3, 5, 9, 5, 8, 9, 4, 4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7,
                        2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8
                    ],
                    vec![
                        4, 3, 5, 8, 7, 3, 3, 5, 4, 1, 5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9,
                        5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5
                    ],
                    vec![
                        5, 8, 2, 6, 2, 5, 3, 7, 8, 2, 6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4,
                        7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6
                    ],
                    vec![
                        9, 6, 8, 5, 6, 3, 9, 3, 3, 3, 1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8,
                        5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7
                    ],
                    vec![
                        3, 5, 3, 2, 3, 4, 1, 3, 5, 9, 4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5,
                        6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4
                    ],
                    vec![
                        3, 5, 7, 2, 2, 3, 4, 6, 4, 3, 4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4,
                        5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7
                    ],
                    vec![
                        5, 3, 4, 7, 6, 4, 3, 8, 5, 2, 6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8,
                        6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6
                    ],
                    vec![
                        3, 4, 2, 5, 3, 5, 1, 7, 4, 3, 4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5,
                        7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7
                    ],
                    vec![
                        4, 5, 3, 3, 2, 6, 6, 7, 1, 3, 5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4,
                        8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7
                    ],
                    vec![
                        4, 4, 9, 6, 1, 8, 4, 1, 7, 5, 5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3,
                        1, 6, 3, 9, 7, 7, 7, 3, 9, 4, 2, 7, 4, 1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9
                    ],
                    vec![
                        4, 6, 2, 4, 6, 1, 6, 9, 1, 5, 5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8,
                        3, 8, 2, 3, 7, 7, 9, 5, 7, 9, 4, 9, 3, 4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9
                    ],
                    vec![
                        5, 4, 6, 9, 8, 4, 4, 6, 5, 2, 6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1,
                        6, 6, 8, 7, 4, 8, 7, 9, 3, 2, 7, 7, 9, 8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6
                    ],
                    vec![
                        6, 9, 3, 7, 3, 6, 4, 8, 9, 3, 7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5,
                        8, 6, 1, 2, 5, 9, 3, 6, 1, 6, 9, 7, 2, 3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7
                    ],
                    vec![
                        1, 7, 9, 6, 7, 4, 1, 4, 4, 4, 2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9,
                        6, 3, 6, 6, 6, 4, 1, 3, 9, 1, 7, 4, 7, 7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8
                    ],
                    vec![
                        4, 6, 4, 3, 4, 5, 2, 4, 6, 1, 5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6,
                        7, 4, 6, 8, 3, 7, 9, 7, 6, 7, 8, 5, 7, 9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5
                    ],
                    vec![
                        4, 6, 8, 3, 3, 4, 5, 7, 5, 4, 5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5,
                        6, 7, 9, 7, 6, 7, 9, 2, 6, 6, 7, 8, 1, 8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8
                    ],
                    vec![
                        6, 4, 5, 8, 7, 5, 4, 9, 6, 3, 7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9,
                        7, 6, 2, 8, 5, 9, 7, 8, 2, 1, 8, 7, 3, 9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7
                    ],
                    vec![
                        4, 5, 3, 6, 4, 6, 2, 8, 5, 4, 5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6,
                        8, 4, 1, 7, 6, 7, 8, 6, 9, 7, 9, 5, 2, 8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8
                    ],
                    vec![
                        5, 6, 4, 4, 3, 7, 7, 8, 2, 4, 6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5,
                        9, 9, 1, 4, 6, 8, 9, 7, 7, 6, 1, 1, 2, 5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8
                    ],
                    vec![
                        5, 5, 1, 7, 2, 9, 5, 2, 8, 6, 6, 6, 2, 8, 3, 1, 6, 3, 9, 7, 7, 7, 3, 9, 4,
                        2, 7, 4, 1, 8, 8, 8, 4, 1, 5, 3, 8, 5, 2, 9, 9, 9, 5, 2, 6, 4, 9, 6, 3, 1
                    ],
                    vec![
                        5, 7, 3, 5, 7, 2, 7, 1, 2, 6, 6, 8, 4, 6, 8, 3, 8, 2, 3, 7, 7, 9, 5, 7, 9,
                        4, 9, 3, 4, 8, 8, 1, 6, 8, 1, 5, 1, 4, 5, 9, 9, 2, 7, 9, 2, 6, 2, 5, 6, 1
                    ],
                    vec![
                        6, 5, 7, 1, 9, 5, 5, 7, 6, 3, 7, 6, 8, 2, 1, 6, 6, 8, 7, 4, 8, 7, 9, 3, 2,
                        7, 7, 9, 8, 5, 9, 8, 1, 4, 3, 8, 8, 1, 9, 6, 1, 9, 2, 5, 4, 9, 9, 2, 1, 7
                    ],
                    vec![
                        7, 1, 4, 8, 4, 7, 5, 9, 1, 4, 8, 2, 5, 9, 5, 8, 6, 1, 2, 5, 9, 3, 6, 1, 6,
                        9, 7, 2, 3, 6, 1, 4, 7, 2, 7, 1, 8, 3, 4, 7, 2, 5, 8, 3, 8, 2, 9, 4, 5, 8
                    ],
                    vec![
                        2, 8, 1, 7, 8, 5, 2, 5, 5, 5, 3, 9, 2, 8, 9, 6, 3, 6, 6, 6, 4, 1, 3, 9, 1,
                        7, 4, 7, 7, 7, 5, 2, 4, 1, 2, 8, 5, 8, 8, 8, 6, 3, 5, 2, 3, 9, 6, 9, 9, 9
                    ],
                    vec![
                        5, 7, 5, 4, 5, 6, 3, 5, 7, 2, 6, 8, 6, 5, 6, 7, 4, 6, 8, 3, 7, 9, 7, 6, 7,
                        8, 5, 7, 9, 4, 8, 1, 8, 7, 8, 9, 6, 8, 1, 5, 9, 2, 9, 8, 9, 1, 7, 9, 2, 6
                    ],
                    vec![
                        5, 7, 9, 4, 4, 5, 6, 8, 6, 5, 6, 8, 1, 5, 5, 6, 7, 9, 7, 6, 7, 9, 2, 6, 6,
                        7, 8, 1, 8, 7, 8, 1, 3, 7, 7, 8, 9, 2, 9, 8, 9, 2, 4, 8, 8, 9, 1, 3, 1, 9
                    ],
                    vec![
                        7, 5, 6, 9, 8, 6, 5, 1, 7, 4, 8, 6, 7, 1, 9, 7, 6, 2, 8, 5, 9, 7, 8, 2, 1,
                        8, 7, 3, 9, 6, 1, 8, 9, 3, 2, 9, 8, 4, 1, 7, 2, 9, 1, 4, 3, 1, 9, 5, 2, 8
                    ],
                    vec![
                        5, 6, 4, 7, 5, 7, 3, 9, 6, 5, 6, 7, 5, 8, 6, 8, 4, 1, 7, 6, 7, 8, 6, 9, 7,
                        9, 5, 2, 8, 7, 8, 9, 7, 1, 8, 1, 6, 3, 9, 8, 9, 1, 8, 2, 9, 2, 7, 4, 1, 9
                    ],
                    vec![
                        6, 7, 5, 5, 4, 8, 8, 9, 3, 5, 7, 8, 6, 6, 5, 9, 9, 1, 4, 6, 8, 9, 7, 7, 6,
                        1, 1, 2, 5, 7, 9, 1, 8, 8, 7, 2, 2, 3, 6, 8, 1, 2, 9, 9, 8, 3, 3, 4, 7, 9
                    ],
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
        assert_eq!(actual, 2887);
    }
}
