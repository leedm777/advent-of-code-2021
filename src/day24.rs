use regex::Regex;
use std::collections::HashMap;

enum Value {
    Literal(i32),
    Variable(char),
}

impl Value {
    fn get(&self, alu: &ALU) -> i32 {
        match self {
            Value::Literal(v) => *v,
            Value::Variable(v) => alu.get(*v),
        }
    }
}

enum Operation {
    Inp(char),
    Add(char, Value),
    Mul(char, Value),
    Div(char, Value),
    Mod(char, Value),
    Eql(char, Value),
}

impl Operation {
    fn parse(loc: &str) -> Operation {
        let re =
            // Regex::new(r"^(<?op>[a-z]+)").expect("Invalid regex");
            Regex::new(r"^(?P<op>[a-z]+) (?P<a>[a-z])( (?P<b>[a-z]|-?\d+))?$").expect("Invalid regex");
        let cap = re.captures(loc).expect("Invalid line of code");

        let op = cap.name("op").expect("Missing op").as_str();
        let a = cap
            .name("a")
            .expect("Missing a")
            .as_str()
            .chars()
            .next()
            .expect("Invalid a");

        if op == "inp" {
            return Operation::Inp(a);
        }

        let b = cap.name("b").expect("Missing b").as_str();
        let b = match b.parse::<i32>() {
            Ok(v) => Value::Literal(v),
            _ => Value::Variable(b.chars().next().expect("Invalid b")),
        };

        match op {
            "add" => Operation::Add(a, b),
            "mul" => Operation::Mul(a, b),
            "div" => Operation::Div(a, b),
            "mod" => Operation::Mod(a, b),
            "eql" => Operation::Eql(a, b),
            _ => panic!("Invalid op"),
        }
    }

    fn exec(&self, alu: &mut ALU) {
        match self {
            Operation::Inp(ch) => {
                let v = alu.input.pop().expect("End of input");
                alu.set(*ch, v);
            }
            Operation::Add(ch, v) => {
                let lhs = alu.get(*ch);
                let rhs = v.get(alu);
                alu.set(*ch, lhs + rhs);
            }
            Operation::Mul(ch, v) => {
                let lhs = alu.get(*ch);
                let rhs = v.get(alu);
                alu.set(*ch, lhs * rhs);
            }
            Operation::Div(ch, v) => {
                let lhs = alu.get(*ch);
                let rhs = v.get(alu);
                alu.set(*ch, lhs / rhs);
            }
            Operation::Mod(ch, v) => {
                let lhs = alu.get(*ch);
                let rhs = v.get(alu);
                alu.set(*ch, lhs % rhs);
            }
            Operation::Eql(ch, v) => {
                let lhs = alu.get(*ch);
                let rhs = v.get(alu);
                let res = if lhs == rhs { 1 } else { 0 };
                alu.set(*ch, res);
            }
        }
    }
}

pub struct ALU {
    program: Vec<Operation>,
    memory: HashMap<char, i32>,
    input: Vec<i32>,
}

impl ALU {
    fn get(&self, ch: char) -> i32 {
        *self.memory.get(&ch).unwrap_or(&0)
    }

    fn set(&mut self, ch: char, v: i32) {
        self.memory.insert(ch, v);
    }

    fn run(&mut self) {
        while let Some(op) = self.program.pop() {
            op.exec(self);
        }
    }
}

pub fn parse(input: &str) -> ALU {
    return ALU {
        program: input.lines().map(Operation::parse).rev().collect(),
        memory: HashMap::new(),
        input: vec![],
    };
}

pub fn part1(_input: &ALU) -> i32 {
    0
}

pub fn part2(_input: &ALU) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn ex1() -> String {
        vec!["inp x", "mul x -1"].join("\n")
    }

    #[test]
    fn test_ex1() {
        let mut alu = parse(&ex1());
        alu.input = vec![314];
        alu.run();
        assert_eq!(alu.get('x'), -314);
    }

    fn ex2() -> String {
        vec![
            "inp w", "add z w", "mod z 2", "div w 2", "add y w", "mod y 2", "div w 2", "add x w",
            "mod x 2", "div w 2", "mod w 2",
        ]
        .join("\n")
    }

    #[test]
    fn test_ex2() {
        let mut alu = parse(&ex2());
        alu.input = vec![10];
        alu.run();
        assert_eq!(alu.get('w'), 1);
        assert_eq!(alu.get('x'), 0);
        assert_eq!(alu.get('y'), 1);
        assert_eq!(alu.get('z'), 0);
    }

    fn real() -> String {
        util::read_input(24)
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_ex1() {
        let actual = part2(&parse(&ex1()));
        assert_eq!(actual, 0);
    }

    #[test]
    fn test_part2_real() {
        let actual = part2(&parse(&real()));
        assert_eq!(actual, 0);
    }
}