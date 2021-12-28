use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Expression {
    Constant(i64),
    Input(u8),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Mod(Box<Expression>, Box<Expression>),
    Eql(Box<Expression>, Box<Expression>),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Constant(c) => c.to_string(),
            Expression::Input(i) => format!("i{}", i),
            Expression::Add(lhs, rhs) => format!("{} + {}", lhs.to_string(), rhs.to_string()),
            Expression::Mul(lhs, rhs) => format!("{} * {}", lhs.to_string(), rhs.to_string()),
            Expression::Div(lhs, rhs) => format!("{} / {}", lhs.to_string(), rhs.to_string()),
            Expression::Mod(lhs, rhs) => format!("{} % {}", lhs.to_string(), rhs.to_string()),
            Expression::Eql(lhs, rhs) => format!("{} == {}", lhs.to_string(), rhs.to_string()),
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

    fn exec(&self, alu: &mut ALU) {
        println!("({}) {:?}", alu.next_input, self);
        match self {
            Operation::Inp(ch) => {
                let exp = Expression::Input(alu.next_input);
                alu.next_input += 1;
                alu.set(*ch, exp);
            }
            Operation::Add(ch, v) => {
                // replace what's on the left hand side to make the borrow checker happy
                let lhs = std::mem::replace(
                    alu.memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                    Expression::Constant(0),
                );
                let rhs = match v {
                    Value::Literal(i) => alu
                        .constants
                        .entry(*i)
                        .or_insert_with(|| Expression::Constant(*i)),
                    Value::Variable(ch) => alu
                        .memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                };

                let exp = match (lhs, rhs) {
                    (Expression::Constant(lhs), Expression::Constant(rhs)) => {
                        Expression::Constant(lhs + *rhs)
                    }
                    (lhs, rhs) => Expression::Add(Box::new(lhs), Box::new(rhs.clone())),
                };
                alu.set(*ch, exp);
            }
            Operation::Mul(ch, v) => {
                // replace what's on the left hand side to make the borrow checker happy
                let lhs = std::mem::replace(
                    alu.memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                    Expression::Constant(0),
                );
                let rhs = match v {
                    Value::Literal(i) => alu
                        .constants
                        .entry(*i)
                        .or_insert_with(|| Expression::Constant(*i)),
                    Value::Variable(ch) => alu
                        .memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                };

                let exp = match (lhs, rhs) {
                    (Expression::Constant(lhs), Expression::Constant(rhs)) => {
                        Expression::Constant(lhs * *rhs)
                    }
                    (lhs, rhs) => Expression::Mul(Box::new(lhs), Box::new(rhs.clone())),
                };
                alu.set(*ch, exp);
            }
            Operation::Div(ch, v) => {
                // replace what's on the left hand side to make the borrow checker happy
                let lhs = std::mem::replace(
                    alu.memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                    Expression::Constant(0),
                );
                let rhs = match v {
                    Value::Literal(i) => alu
                        .constants
                        .entry(*i)
                        .or_insert_with(|| Expression::Constant(*i)),
                    Value::Variable(ch) => alu
                        .memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                };

                let exp = match (lhs, rhs) {
                    (Expression::Constant(lhs), Expression::Constant(rhs)) => {
                        Expression::Constant(lhs / *rhs)
                    }
                    (lhs, rhs) => Expression::Div(Box::new(lhs), Box::new(rhs.clone())),
                };
                alu.set(*ch, exp);
            }
            Operation::Mod(ch, v) => {
                // replace what's on the left hand side to make the borrow checker happy
                let lhs = std::mem::replace(
                    alu.memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                    Expression::Constant(0),
                );
                let rhs = match v {
                    Value::Literal(i) => alu
                        .constants
                        .entry(*i)
                        .or_insert_with(|| Expression::Constant(*i)),
                    Value::Variable(ch) => alu
                        .memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                };

                let exp = match (lhs, rhs) {
                    (Expression::Constant(lhs), Expression::Constant(rhs)) => {
                        Expression::Constant(lhs % *rhs)
                    }
                    (lhs, rhs) => Expression::Mod(Box::new(lhs), Box::new(rhs.clone())),
                };
                alu.set(*ch, exp);
            }
            Operation::Eql(ch, v) => {
                // replace what's on the left hand side to make the borrow checker happy
                let lhs = std::mem::replace(
                    alu.memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                    Expression::Constant(0),
                );
                let rhs = match v {
                    Value::Literal(i) => alu
                        .constants
                        .entry(*i)
                        .or_insert_with(|| Expression::Constant(*i)),
                    Value::Variable(ch) => alu
                        .memory
                        .entry(*ch)
                        .or_insert_with(|| Expression::Constant(0)),
                };

                let exp = match (lhs, rhs) {
                    (Expression::Constant(lhs), Expression::Constant(rhs)) => {
                        Expression::Constant(if lhs == *rhs { 1 } else { 0 })
                    }
                    (Expression::Input(inp), Expression::Constant(rhs)) => {
                        if *rhs < 1 || *rhs > 9 {
                            Expression::Constant(0)
                        } else {
                            Expression::Eql(
                                Box::new(Expression::Input(inp)),
                                Box::new(Expression::Constant(*rhs)),
                            )
                        }
                    }
                    (lhs, rhs) => Expression::Eql(Box::new(lhs), Box::new(rhs.clone())),
                };
                alu.set(*ch, exp);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ALU {
    program: Vec<Operation>,
    memory: HashMap<char, Expression>,
    next_input: u8,

    constants: HashMap<i64, Expression>,
}

impl ALU {
    fn run(&mut self) {
        while let Some(op) = self.program.pop() {
            op.exec(self);
        }
    }

    fn get(&mut self, ch: char) -> &Expression {
        self.memory
            .entry(ch)
            .or_insert_with(|| Expression::Constant(0))
    }
    fn set(&mut self, ch: char, exp: Expression) {
        self.memory.insert(ch, exp);
    }
}

pub fn parse(input: &str) -> ALU {
    return ALU {
        program: input.lines().map(Operation::parse).rev().collect(),
        memory: HashMap::new(),
        next_input: 0,

        constants: HashMap::new(),
    };
}

pub fn part1(init: &ALU) -> i64 {
    let mut meta = init.clone();
    meta.run();

    let ch = 'z';
    println!(
        "{:?}",
        meta.memory
            .entry(ch)
            .or_insert_with(|| Expression::Constant(0))
    );

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
