use regex::Regex;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

#[derive(Clone, Debug)]
enum Expression {
    Constant(i64),
    Input(u8),
    Calculation(Op, Box<Expression>, Box<Expression>),
}

impl ToString for Expression {
    fn to_string(&self) -> String {
        match self {
            Expression::Constant(c) => format!("{}", c),
            Expression::Input(i) => format!("i{}", i),
            Expression::Calculation(op, lhs, rhs) => format!("({:?} {:?} {:?})", op, lhs, rhs),
        }
    }
}

#[derive(Clone, Debug)]
enum Value {
    Literal(i64),
    Variable(char),
}

impl Value {
    fn get(&self, alu: &ALU) -> i64 {
        match self {
            Value::Literal(v) => *v,
            Value::Variable(v) => alu.get(*v),
        }
    }

    fn meta_get(&self, alu: &ALU) -> Expression {
        match self {
            Value::Literal(v) => Expression::Constant(*v),
            Value::Variable(v) => alu.meta_get(*v).clone(),
        }
    }
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

    fn meta_exec(&self, alu: &mut ALU) {
        println!("{:?}", self);
        match self {
            Operation::Inp(ch) => {
                let i = alu.next_input;
                alu.next_input += 1;

                let v = Expression::Input(i);
                alu.meta_set(*ch, v);
            }
            Operation::Add(ch, v) => {
                let lhs = alu.meta_get(*ch).clone();
                let rhs = v.meta_get(alu);
                alu.meta_set(
                    *ch,
                    Expression::Calculation(Op::Add, Box::new(lhs), Box::new(rhs)),
                );
            }
            Operation::Mul(ch, v) => {
                let lhs = alu.meta_get(*ch).clone();
                let rhs = v.meta_get(alu);
                alu.meta_set(
                    *ch,
                    Expression::Calculation(Op::Mul, Box::new(lhs), Box::new(rhs)),
                );
            }
            Operation::Div(ch, v) => {
                let lhs = alu.meta_get(*ch).clone();
                let rhs = v.meta_get(alu);
                alu.meta_set(
                    *ch,
                    Expression::Calculation(Op::Div, Box::new(lhs), Box::new(rhs)),
                );
            }
            Operation::Mod(ch, v) => {
                let lhs = alu.meta_get(*ch).clone();
                let rhs = v.meta_get(alu);
                alu.meta_set(
                    *ch,
                    Expression::Calculation(Op::Mod, Box::new(lhs), Box::new(rhs)),
                );
            }
            Operation::Eql(ch, v) => {
                let lhs = alu.meta_get(*ch).clone();
                let rhs = v.meta_get(alu);
                alu.meta_set(
                    *ch,
                    Expression::Calculation(Op::Eql, Box::new(lhs), Box::new(rhs)),
                );
            }
        }
    }
}

#[derive(Clone)]
pub struct ALU {
    program: Vec<Operation>,
    memory: HashMap<char, i64>,
    input: Vec<i64>,

    meta_memory: HashMap<char, Expression>,
    next_input: u8,
}

impl ALU {
    fn get(&self, ch: char) -> i64 {
        *self.memory.get(&ch).unwrap_or(&0)
    }

    fn set(&mut self, ch: char, v: i64) {
        self.memory.insert(ch, v);
    }

    fn meta_get(&self, ch: char) -> &Expression {
        self.meta_memory.get(&ch).expect("Unexpected variable")
    }

    fn meta_set(&mut self, ch: char, v: Expression) {
        self.meta_memory.insert(ch, v);
    }

    fn run(&mut self) {
        while let Some(op) = self.program.pop() {
            op.exec(self);
        }
    }

    fn meta_run(&mut self) {
        while let Some(op) = self.program.pop() {
            op.meta_exec(self);
        }
    }

    fn run_input(&self, input: &Vec<i64>) -> Self {
        let mut alu = self.clone();
        alu.input = input.clone();
        alu.run();
        alu
    }
}

pub fn parse(input: &str) -> ALU {
    let mut meta_memory = HashMap::new();
    meta_memory.insert('w', Expression::Constant(0));
    meta_memory.insert('x', Expression::Constant(0));
    meta_memory.insert('y', Expression::Constant(0));
    meta_memory.insert('z', Expression::Constant(0));

    return ALU {
        program: input.lines().map(Operation::parse).rev().collect(),
        memory: HashMap::new(),
        input: vec![],

        meta_memory,
        next_input: 0,
    };
}

pub fn part1(init: &ALU) -> i64 {
    let mut meta = init.clone();
    meta.meta_run();

    println!("{:?}", meta.meta_get('z'));

    /*
    let mut input = vec![1i64; 14];
    let mut best = input.clone();
    let mut min_z = i64::MAX;

    // find the best we can with single digits
    loop {
        for digit in 0..14 {
            let mut min_digit = 0i64;
            for i in 1..=9 {
                input[digit] = i;
                let next = init.run_input(&input);
                let z = next.get('z');
                if z == 0 {
                    return input_to_i64(&input);
                }
                if z <= min_z {
                    min_z = z;
                    min_digit = i;
                }
            }
            input[digit] = min_digit;
            let next = init.run_input(&input);
            println!("{} => {}", input_to_i64(&input), next.get('z'));
        }
        println!();
        if best == input {
            break;
        }
        best = input.clone();
    }

    // double digits
    loop {
        for digit1 in 0..14 {
            let mut min_digit1 = input[digit1];

            for i in 1..=9 {
                input[digit1] = i;

                for digit2 in 0..14 {
                    if digit1 == digit2 {
                        continue;
                    }

                    let mut min_digit2 = input[digit2];
                    for j in 1..=9 {
                        input[digit2] = j;
                        let next = init.run_input(&input);
                        let z = next.get('z');
                        if z == 0 {
                            return input_to_i64(&input);
                        }
                        if z <= min_z {
                            min_z = z;
                            min_digit1 = i;
                            min_digit2 = j;
                        }
                    }
                    assert_ne!(min_digit2, 0);
                    input[digit2] = min_digit2;
                }
            }

            assert_ne!(min_digit1, 0);
            input[digit1] = min_digit1;

            let next = init.run_input(&input);
            println!("{} => {}", input_to_i64(&input), next.get('z'));
        }
        println!();

        // didn't find anything better; break
        if best == input {
            break;
        }

        // slight improvement; keep trying
        best = input.clone();
    }
     */

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
