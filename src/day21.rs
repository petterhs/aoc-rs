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

fn path_to_humn(monkeys: &HashMap<String, Monkey>, name: String, path: Vec<u8>) -> Option<Vec<u8>> {
    //Depth first search to find the path

    if name == "humn" {
        return Some(path);
    }

    let monkey = monkeys.get(&name).unwrap();

    let mut path = path.clone();

    match monkey.number {
        Action::Number(_) => None,
        Action::Op(ref op) => {
            let mut path1 = path.clone();
            path.push(0);
            path1.push(1);
            if let Some(path) = path_to_humn(monkeys, op.lhs.clone(), path) {
                return Some(path);
            }

            if let Some(path) = path_to_humn(monkeys, op.rhs.clone(), path1) {
                return Some(path);
            }

            None
        }
    }
}

fn part1() -> i64 {
    let input = include_str!("../input/21");

    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let monkey = line.parse::<Monkey>().unwrap();
        // println!("{:?}", monkey);
        monkeys.insert(monkey.name.clone(), monkey);
    }

    number(&monkeys, "root".to_string())
}

fn part2() -> i64 {
    let input = include_str!("../input/21");

    let mut monkeys = HashMap::new();

    for line in input.lines() {
        let monkey = line.parse::<Monkey>().unwrap();

        monkeys.insert(monkey.name.clone(), monkey);
    }

    let path = path_to_humn(&monkeys, "root".to_string(), Vec::new()).unwrap();

    let root = monkeys.get("root").unwrap();
    let mut next;
    let mut goal_number = match root.number {
        Action::Number(_) => panic!("root is number"),
        Action::Op(ref op) => match path[0] {
            0 => {
                next = op.lhs.clone();
                number(&monkeys, op.rhs.clone())
            }
            1 => {
                next = op.rhs.clone();
                number(&monkeys, op.lhs.clone())
            }
            _ => panic!("unknown path"),
        },
    };

    for i in path.iter().skip(1) {
        let monkey = monkeys.get(&next).unwrap();
        match monkey.number {
            Action::Number(_) => panic!("root is number"),
            Action::Op(ref op) => match *i {
                0 => {
                    next = op.lhs.clone();
                    match op.op {
                        Operation::Add => goal_number -= number(&monkeys, op.rhs.clone()),
                        Operation::Sub => goal_number += number(&monkeys, op.rhs.clone()),
                        Operation::Mul => goal_number /= number(&monkeys, op.rhs.clone()),
                        Operation::Devide => goal_number *= number(&monkeys, op.rhs.clone()),
                    }
                }
                1 => {
                    next = op.rhs.clone();
                    match op.op {
                        Operation::Add => goal_number -= number(&monkeys, op.lhs.clone()),
                        Operation::Sub => {
                            goal_number = number(&monkeys, op.lhs.clone()) - goal_number
                        }
                        Operation::Mul => goal_number /= number(&monkeys, op.lhs.clone()),
                        Operation::Devide => goal_number *= number(&monkeys, op.lhs.clone()),
                    }
                }
                _ => panic!("unknown path"),
            },
        }
    }

    let humn = monkeys.get_mut("humn").unwrap();
    match humn.number {
        Action::Number(ref mut n) => *n = goal_number,
        Action::Op(_) => panic!("humn is op"),
    }

    let root = monkeys.get_mut("root").unwrap();

    match root.number {
        Action::Number(_) => panic!("root is number"),
        Action::Op(ref mut op) => op.op = Operation::Sub,
    }

    assert_eq!(0, number(&monkeys, "root".to_string()));

    goal_number
}

pub fn run() {
    println!("day21");

    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
