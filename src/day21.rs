use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Devide,
}

#[derive(Debug)]
struct Op {
    lhs: String,
    rhs: String,
    op: Operation,
}

#[derive(Debug)]
enum Action {
    Op(Op),
    Number(i64),
}

#[derive(Debug)]
struct Monkey {
    name: String,
    number: Action,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let name = parts.next().unwrap().strip_suffix(':').unwrap().to_string();
        let lhs = parts.next().unwrap();

        if let Ok(number) = lhs.parse::<i64>() {
            return Ok(Monkey {
                name,
                number: Action::Number(number),
            });
        }

        let op = match parts.next().unwrap() {
            "+" => Operation::Add,
            "-" => Operation::Sub,
            "*" => Operation::Mul,
            "/" => Operation::Devide,
            _ => panic!("unknown op"),
        };
        let rhs = parts.next().unwrap().to_string();

        Ok(Monkey {
            name,
            number: Action::Op(Op {
                lhs: lhs.to_string(),
                rhs,
                op,
            }),
        })
    }
}

fn number(monkeys: &HashMap<String, Monkey>, name: String) -> i64 {
    match monkeys.get(&name).unwrap().number {
        Action::Number(n) => n,
        Action::Op(ref op) => {
            let lhs = match op.lhs.parse::<i64>() {
                Ok(n) => n,
                Err(_) => number(monkeys, op.lhs.clone()),
            };
            let rhs = match op.rhs.parse::<i64>() {
                Ok(n) => n,
                Err(_) => number(monkeys, op.rhs.clone()),
            };
            match op.op {
                Operation::Add => lhs + rhs,
                Operation::Sub => lhs - rhs,
                Operation::Mul => lhs * rhs,
                Operation::Devide => lhs / rhs,
            }
        }
    }
}

fn part1() -> i64 {
    let input = include_str!("../input/21");

    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let monkey = line.parse::<Monkey>().unwrap();
        println!("{:?}", monkey);
        monkeys.insert(monkey.name.clone(), monkey);
    }

    number(&monkeys, "root".to_string())
}

pub fn run() {
    println!("day21");

    println!("part1: {}", part1());
}
