use regex::Regex;
use std::collections::HashMap;

// TODO: Try removing all the lifetimes and just use boxes instead

#[derive(Clone, Debug)]
enum Expression<'a> {
    Constant(i64),
    Input(u8),
    Add(&'a Expression<'a>, &'a Expression<'a>),
    Mul(&'a Expression<'a>, &'a Expression<'a>),
    Div(&'a Expression<'a>, &'a Expression<'a>),
    Mod(&'a Expression<'a>, &'a Expression<'a>),
    Eql(&'a Expression<'a>, &'a Expression<'a>),
}

impl ToString for Expression<'_> {
    fn to_string(&self) -> String {
        match self {
            Expression::Constant(c) => c.to_string(),
            Expression::Input(i) => format!("i{}", i),
            Expression::Add(lhs, rhs) => format!("({}) _ ({})", lhs.to_string(), rhs.to_string()),
            Expression::Mul(lhs, rhs) => format!("({}) _ ({})", lhs.to_string(), rhs.to_string()),
            Expression::Div(lhs, rhs) => format!("({}) _ ({})", lhs.to_string(), rhs.to_string()),
            Expression::Mod(lhs, rhs) => format!("({}) _ ({})", lhs.to_string(), rhs.to_string()),
            Expression::Eql(lhs, rhs) => format!("({}) _ ({})", lhs.to_string(), rhs.to_string()),
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Literal(i64),
    Variable(char),
}

#[derive(Clone, Debug)]
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
        let b = match b.parse::<i64>() {
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

    fn exec<'a>(&self, alu: &'a mut ALU<'a>) {
        match self {
            Operation::Inp(ch) => {
                alu.input(*ch);
            }
            Operation::Add(ch, v) => {}
            Operation::Mul(ch, v) => {}
            Operation::Div(ch, v) => {}
            Operation::Mod(ch, v) => {}
            Operation::Eql(ch, v) => {}
        }

        unimplemented!("TODO!")
    }
}

#[derive(Clone, Debug)]
pub struct ALU<'a> {
    program: Vec<Operation>,
    memory: HashMap<char, &'a Expression<'a>>,
    next_input: u8,

    // memory management
    expressions: Vec<Expression<'a>>,
}

impl<'a> ALU<'a> {
    fn run(&'a mut self) {
        while let Some(op) = self.program.pop() {
            op.exec(self);
        }
    }

    fn get(&mut self, ch: char) -> &Expression<'_> {
        match self.memory.get(&ch) {
            Some(exp) => exp,
            None => {
                let exp = Expression::Constant(0);
                self.expressions.push(exp);
                self.expressions.last().expect("But I just pushed it there")
            }
        }
    }

    fn set(&mut self, ch: char, exp: &'a Expression<'a>) {
        self.memory.insert(ch, exp);
    }

    fn input<'b>(&'a mut self, ch: char) {
        let exp = Expression::Input(self.next_input);
        self.next_input += 1;
        self.expressions.push(exp);
        let exp = self.expressions.last().expect("But I just pushed it");

        self.set(ch, exp);
    }
}

pub fn parse<'a, 'b>(input: &'a str) -> ALU<'b> {
    return ALU {
        program: input.lines().map(Operation::parse).rev().collect(),
        memory: HashMap::new(),
        next_input: 0,
        expressions: vec![],
    };
}

pub fn part1(init: &ALU) -> i64 {
    let mut meta = init.clone();
    meta.run();

    println!("{:?}", meta.get('z'));

    -1
}

fn input_to_i64(input: &Vec<i64>) -> i64 {
    input.iter().rev().fold(0, |acc, d| acc * 10 + d)
}

pub fn part2(_input: &ALU) -> i64 {
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
        alu.run();
        assert_eq!(alu.get('x').to_string(), "(i0) * (-1)");
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
        alu.run();
        assert_eq!(alu.get('w').to_string(), "");
        assert_eq!(alu.get('x').to_string(), "");
        assert_eq!(alu.get('y').to_string(), "");
        assert_eq!(alu.get('z').to_string(), "");
    }

    fn real() -> String {
        util::read_input(24)
    }

    #[test]
    fn test_part1_real() {
        let actual = part1(&parse(&real()));
        assert_eq!(actual, 0); // 83719996499591 is too high
                               // 19599469991738 is too low
                               // 29599458991739 is too low
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
